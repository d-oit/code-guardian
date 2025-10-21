#!/bin/bash
# Continuous Performance Monitoring - Automated monitoring with alerting
# Complements the existing performance-monitor.sh with continuous monitoring capabilities

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Configuration
MONITOR_INTERVAL=1800  # 30 minutes
ALERT_THRESHOLD_BUILDS=3  # Alert after 3 consecutive failures
PERFORMANCE_LOG="performance/continuous_monitor.log"
ALERT_LOG="performance/alerts.log"
STATE_FILE="performance/monitor_state.json"

# Performance thresholds
MAX_BUILD_TIME=180    # 3 minutes
MAX_TEST_TIME=120     # 2 minutes
MIN_COVERAGE=82       # 82% minimum coverage
MAX_MEMORY_MB=200     # 200MB max memory

log_performance() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$PERFORMANCE_LOG"
}

alert() {
    local message="$1"
    local severity="${2:-WARNING}"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    echo "[$timestamp] [$severity] $message" | tee -a "$ALERT_LOG"
    log_performance "ALERT [$severity]: $message"
    
    # Could integrate with external alerting systems here
    # e.g., Slack, email, PagerDuty
}

check_performance_regression() {
    local metric="$1"
    local current_value="$2"
    local threshold="$3"
    local comparison="$4"  # "gt" or "lt"
    
    case "$comparison" in
        "gt")
            if (( $(echo "$current_value > $threshold" | bc -l) )); then
                return 1  # Regression detected
            fi
            ;;
        "lt")
            if (( $(echo "$current_value < $threshold" | bc -l) )); then
                return 1  # Regression detected
            fi
            ;;
    esac
    return 0  # No regression
}

run_monitoring_cycle() {
    log_performance "Starting monitoring cycle..."
    
    # Initialize state if it doesn't exist
    if [[ ! -f "$STATE_FILE" ]]; then
        echo '{"consecutive_failures": 0, "last_successful_run": "", "metrics_history": []}' > "$STATE_FILE"
    fi
    
    local failures=0
    local current_time=$(date -Iseconds)
    
    # Run performance checks
    log_performance "Running performance analysis..."
    
    # Build performance check
    local build_start=$(date +%s.%N)
    if cargo build --workspace --quiet; then
        local build_time=$(echo "$(date +%s.%N) - $build_start" | bc -l)
        log_performance "Build completed in ${build_time}s"
        
        if check_performance_regression "build_time" "$build_time" "$MAX_BUILD_TIME" "gt"; then
            log_performance "✅ Build performance: OK (${build_time}s < ${MAX_BUILD_TIME}s)"
        else
            alert "Build time regression: ${build_time}s > ${MAX_BUILD_TIME}s threshold" "WARNING"
            ((failures++))
        fi
    else
        alert "Build failed during monitoring cycle" "CRITICAL"
        ((failures++))
    fi
    
    # Test performance check
    local test_start=$(date +%s.%N)
    if cargo test --workspace --quiet; then
        local test_time=$(echo "$(date +%s.%N) - $test_start" | bc -l)
        log_performance "Tests completed in ${test_time}s"
        
        if check_performance_regression "test_time" "$test_time" "$MAX_TEST_TIME" "gt"; then
            log_performance "✅ Test performance: OK (${test_time}s < ${MAX_TEST_TIME}s)"
        else
            alert "Test time regression: ${test_time}s > ${MAX_TEST_TIME}s threshold" "WARNING"
            ((failures++))
        fi
    else
        alert "Tests failed during monitoring cycle" "CRITICAL"
        ((failures++))
    fi
    
    # Coverage check
    if command -v cargo-llvm-cov >/dev/null 2>&1; then
        local coverage=$(cargo llvm-cov --workspace --summary-only 2>/dev/null | grep "Overall coverage" | awk '{print $3}' | sed 's/%//' || echo "0")
        
        if check_performance_regression "coverage" "$coverage" "$MIN_COVERAGE" "lt"; then
            log_performance "✅ Coverage: OK (${coverage}% >= ${MIN_COVERAGE}%)"
        else
            alert "Coverage regression: ${coverage}% < ${MIN_COVERAGE}% threshold" "WARNING"
            ((failures++))
        fi
    fi
    
    # Memory usage check (approximation)
    local memory_kb=$(ps -o pid,vsz --no-headers -C cargo | awk '{sum+=$2} END {print sum}' || echo "0")
    local memory_mb=$(echo "scale=1; $memory_kb / 1024" | bc -l || echo "0")
    
    if check_performance_regression "memory" "$memory_mb" "$MAX_MEMORY_MB" "gt"; then
        log_performance "✅ Memory usage: OK (${memory_mb}MB < ${MAX_MEMORY_MB}MB)"
    else
        alert "Memory usage high: ${memory_mb}MB > ${MAX_MEMORY_MB}MB threshold" "WARNING"
    fi
    
    # Update state
    local consecutive_failures=$(jq -r '.consecutive_failures' "$STATE_FILE" 2>/dev/null || echo "0")
    
    if [[ $failures -eq 0 ]]; then
        # Reset consecutive failures on success
        jq ". + {\"consecutive_failures\": 0, \"last_successful_run\": \"$current_time\"}" "$STATE_FILE" > "$STATE_FILE.tmp"
        mv "$STATE_FILE.tmp" "$STATE_FILE"
        log_performance "✅ All performance checks passed"
    else
        # Increment consecutive failures
        consecutive_failures=$((consecutive_failures + 1))
        jq ". + {\"consecutive_failures\": $consecutive_failures}" "$STATE_FILE" > "$STATE_FILE.tmp"
        mv "$STATE_FILE.tmp" "$STATE_FILE"
        
        if [[ $consecutive_failures -ge $ALERT_THRESHOLD_BUILDS ]]; then
            alert "Performance degradation detected: $consecutive_failures consecutive failures" "CRITICAL"
        fi
        
        log_performance "❌ Performance issues detected: $failures failures"
    fi
    
    # Store metrics
    local metrics=$(cat << EOF
{
    "timestamp": "$current_time",
    "build_time": ${build_time:-0},
    "test_time": ${test_time:-0},
    "coverage": ${coverage:-0},
    "memory_mb": ${memory_mb:-0},
    "failures": $failures
}
EOF
)
    
    # Append to history (keep last 100 entries)
    jq ".metrics_history += [$metrics] | .metrics_history = .metrics_history[-100:]" "$STATE_FILE" > "$STATE_FILE.tmp"
    mv "$STATE_FILE.tmp" "$STATE_FILE"
    
    log_performance "Monitoring cycle completed"
    return $failures
}

generate_monitoring_report() {
    local report_file="performance/monitoring_report_$(date +%Y%m%d_%H%M%S).json"
    
    if [[ -f "$STATE_FILE" ]]; then
        # Generate comprehensive report
        local state=$(cat "$STATE_FILE")
        local recent_alerts=$(tail -n 20 "$ALERT_LOG" 2>/dev/null || echo "")
        
        cat > "$report_file" << EOF
{
    "generated_at": "$(date -Iseconds)",
    "monitoring_status": $(echo "$state" | jq '.'),
    "recent_alerts": [
        $(echo "$recent_alerts" | sed 's/"/\\"/g' | awk '{print "\"" $0 "\""}' | paste -sd ',' -)
    ],
    "thresholds": {
        "max_build_time_seconds": $MAX_BUILD_TIME,
        "max_test_time_seconds": $MAX_TEST_TIME,
        "min_coverage_percent": $MIN_COVERAGE,
        "max_memory_mb": $MAX_MEMORY_MB
    },
    "alerting": {
        "threshold_builds": $ALERT_THRESHOLD_BUILDS,
        "monitor_interval_seconds": $MONITOR_INTERVAL
    }
}
EOF
        
        log_performance "📊 Monitoring report generated: $report_file"
    else
        log_performance "⚠️ No monitoring state available for report generation"
    fi
}

start_continuous_monitoring() {
    log_performance "🚀 Starting continuous performance monitoring..."
    log_performance "Monitor interval: ${MONITOR_INTERVAL}s ($(echo "scale=1; $MONITOR_INTERVAL/60" | bc)m)"
    log_performance "Performance thresholds: Build=${MAX_BUILD_TIME}s, Test=${MAX_TEST_TIME}s, Coverage=${MIN_COVERAGE}%, Memory=${MAX_MEMORY_MB}MB"
    
    # Create monitoring directories
    mkdir -p performance
    
    # Trap to handle graceful shutdown
    trap 'log_performance "🛑 Continuous monitoring stopped"; exit 0' SIGINT SIGTERM
    
    while true; do
        run_monitoring_cycle
        
        # Generate report every 6 hours (6 * 60 * 60 / MONITOR_INTERVAL cycles)
        local cycles_per_report=$((21600 / MONITOR_INTERVAL))
        local cycle_count=$(jq -r '.metrics_history | length' "$STATE_FILE" 2>/dev/null || echo "0")
        
        if (( cycle_count % cycles_per_report == 0 )) && (( cycle_count > 0 )); then
            generate_monitoring_report
        fi
        
        log_performance "💤 Sleeping for ${MONITOR_INTERVAL}s..."
        sleep "$MONITOR_INTERVAL"
    done
}

show_current_status() {
    echo "📊 Current Performance Monitoring Status"
    echo "========================================"
    
    if [[ -f "$STATE_FILE" ]]; then
        local state=$(cat "$STATE_FILE")
        local consecutive_failures=$(echo "$state" | jq -r '.consecutive_failures')
        local last_successful=$(echo "$state" | jq -r '.last_successful_run')
        local metrics_count=$(echo "$state" | jq -r '.metrics_history | length')
        
        echo "Consecutive failures: $consecutive_failures"
        echo "Last successful run: $last_successful"
        echo "Metrics collected: $metrics_count"
        
        if [[ $metrics_count -gt 0 ]]; then
            echo ""
            echo "Latest metrics:"
            echo "$state" | jq -r '.metrics_history[-1] | "  Build time: \(.build_time)s\n  Test time: \(.test_time)s\n  Coverage: \(.coverage)%\n  Memory: \(.memory_mb)MB\n  Timestamp: \(.timestamp)"'
        fi
        
        echo ""
        echo "Recent alerts:"
        tail -n 5 "$ALERT_LOG" 2>/dev/null || echo "  No recent alerts"
    else
        echo "No monitoring data available. Run 'start' to begin monitoring."
    fi
}

main() {
    case "${1:-help}" in
        "start")
            start_continuous_monitoring
            ;;
        "check")
            run_monitoring_cycle
            exit $?
            ;;
        "status")
            show_current_status
            ;;
        "report")
            generate_monitoring_report
            ;;
        "help"|*)
            cat << EOF
Continuous Performance Monitor for Code Guardian

Usage: $0 {start|check|status|report|help}

Commands:
  start   - Start continuous monitoring (runs indefinitely)
  check   - Run a single monitoring cycle
  status  - Show current monitoring status
  report  - Generate monitoring report
  help    - Show this help

Configuration:
  Monitor interval: ${MONITOR_INTERVAL}s ($(echo "scale=1; $MONITOR_INTERVAL/60" | bc)m)
  Alert threshold: $ALERT_THRESHOLD_BUILDS consecutive failures
  
Performance Thresholds:
  Build time: <${MAX_BUILD_TIME}s
  Test time: <${MAX_TEST_TIME}s
  Coverage: >${MIN_COVERAGE}%
  Memory: <${MAX_MEMORY_MB}MB

Logs:
  Performance: $PERFORMANCE_LOG
  Alerts: $ALERT_LOG
  State: $STATE_FILE
EOF
            ;;
    esac
}

main "$@"