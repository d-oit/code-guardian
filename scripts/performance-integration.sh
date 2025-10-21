#!/bin/bash
# Performance Integration Script - Orchestrates all performance monitoring components
# Integrates existing performance-monitor.sh with new continuous monitoring and dashboard

set -euo pipefail

# Load common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Configuration
PERFORMANCE_DIR="performance"
DASHBOARD_PORT=8080
MONITOR_PID_FILE="$PERFORMANCE_DIR/monitor.pid"
DASHBOARD_PID_FILE="$PERFORMANCE_DIR/dashboard.pid"

log_perf() {
    log $LOG_INFO "$1"
}

check_dependencies() {
    log_perf "🔍 Checking dependencies..."
    
    # Check for required commands
    local missing_deps=()
    
    if ! command -v bc >/dev/null 2>&1; then
        missing_deps+=("bc")
    fi
    
    if ! command -v jq >/dev/null 2>&1; then
        missing_deps+=("jq")
    fi
    
    if ! command -v python3 >/dev/null 2>&1; then
        missing_deps+=("python3")
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log $LOG_ERROR "Missing dependencies: ${missing_deps[*]}"
        log $LOG_INFO "Install missing dependencies:"
        log $LOG_INFO "  Ubuntu/Debian: sudo apt-get install ${missing_deps[*]}"
        log $LOG_INFO "  macOS: brew install ${missing_deps[*]}"
        return 1
    fi
    
    # Check Python dependencies
    if ! python3 -c "import flask" 2>/dev/null; then
        log $LOG_WARN "Flask not installed. Installing..."
        python3 -m pip install flask --user || {
            log $LOG_ERROR "Failed to install Flask. Please install manually: pip install flask"
            return 1
        }
    fi
    
    log_perf "✅ All dependencies satisfied"
    return 0
}

setup_performance_monitoring() {
    log_perf "🔧 Setting up integrated performance monitoring..."
    
    # Create performance directory structure
    mkdir -p "$PERFORMANCE_DIR"/{monitoring,reports,dashboard,benchmarks}
    
    # Initialize monitoring state
    if [[ ! -f "$PERFORMANCE_DIR/monitor_state.json" ]]; then
        cat > "$PERFORMANCE_DIR/monitor_state.json" << 'EOF'
{
    "consecutive_failures": 0,
    "last_successful_run": "",
    "metrics_history": [],
    "setup_completed": true
}
EOF
    fi
    
    # Create performance configuration
    cat > "$PERFORMANCE_DIR/config.json" << 'EOF'
{
    "monitoring": {
        "enabled": true,
        "interval_seconds": 1800,
        "retention_days": 30
    },
    "thresholds": {
        "build_time_seconds": 180,
        "test_time_seconds": 120,
        "coverage_percent": 82,
        "memory_mb": 200
    },
    "dashboard": {
        "port": 8080,
        "auto_refresh_seconds": 30,
        "trend_hours": 24
    },
    "alerting": {
        "consecutive_failures_threshold": 3,
        "email_enabled": false,
        "webhook_enabled": false
    }
}
EOF
    
    log_perf "✅ Performance monitoring setup complete"
}

start_continuous_monitoring() {
    log_perf "🚀 Starting continuous performance monitoring..."
    
    if [[ -f "$MONITOR_PID_FILE" ]] && kill -0 "$(cat "$MONITOR_PID_FILE")" 2>/dev/null; then
        log $LOG_WARN "Continuous monitoring already running (PID: $(cat "$MONITOR_PID_FILE"))"
        return 0
    fi
    
    # Start continuous monitoring in background
    nohup "$SCRIPT_DIR/continuous-monitor.sh" start > "$PERFORMANCE_DIR/monitor.log" 2>&1 &
    local monitor_pid=$!
    echo "$monitor_pid" > "$MONITOR_PID_FILE"
    
    # Wait a moment to check if it started successfully
    sleep 2
    if kill -0 "$monitor_pid" 2>/dev/null; then
        log_perf "✅ Continuous monitoring started (PID: $monitor_pid)"
        return 0
    else
        log $LOG_ERROR "❌ Failed to start continuous monitoring"
        rm -f "$MONITOR_PID_FILE"
        return 1
    fi
}

start_dashboard_server() {
    log_perf "🌐 Starting performance dashboard server..."
    
    if [[ -f "$DASHBOARD_PID_FILE" ]] && kill -0 "$(cat "$DASHBOARD_PID_FILE")" 2>/dev/null; then
        log $LOG_WARN "Dashboard server already running (PID: $(cat "$DASHBOARD_PID_FILE"))"
        log_perf "📊 Dashboard available at: http://localhost:$DASHBOARD_PORT"
        return 0
    fi
    
    # Check if port is available
    if netstat -tuln 2>/dev/null | grep -q ":$DASHBOARD_PORT "; then
        log $LOG_ERROR "Port $DASHBOARD_PORT is already in use"
        return 1
    fi
    
    # Start dashboard server in background
    nohup python3 "$SCRIPT_DIR/performance-dashboard-server.py" > "$PERFORMANCE_DIR/dashboard.log" 2>&1 &
    local dashboard_pid=$!
    echo "$dashboard_pid" > "$DASHBOARD_PID_FILE"
    
    # Wait for server to start
    local attempts=0
    while [[ $attempts -lt 10 ]]; do
        if curl -s "http://localhost:$DASHBOARD_PORT" >/dev/null 2>&1; then
            log_perf "✅ Dashboard server started (PID: $dashboard_pid)"
            log_perf "📊 Dashboard available at: http://localhost:$DASHBOARD_PORT"
            return 0
        fi
        sleep 1
        ((attempts++))
    done
    
    log $LOG_ERROR "❌ Dashboard server failed to start"
    rm -f "$DASHBOARD_PID_FILE"
    return 1
}

stop_services() {
    log_perf "🛑 Stopping performance monitoring services..."
    
    local stopped_any=false
    
    # Stop continuous monitoring
    if [[ -f "$MONITOR_PID_FILE" ]]; then
        local monitor_pid=$(cat "$MONITOR_PID_FILE")
        if kill -0 "$monitor_pid" 2>/dev/null; then
            kill "$monitor_pid"
            log_perf "✅ Stopped continuous monitoring (PID: $monitor_pid)"
            stopped_any=true
        fi
        rm -f "$MONITOR_PID_FILE"
    fi
    
    # Stop dashboard server
    if [[ -f "$DASHBOARD_PID_FILE" ]]; then
        local dashboard_pid=$(cat "$DASHBOARD_PID_FILE")
        if kill -0 "$dashboard_pid" 2>/dev/null; then
            kill "$dashboard_pid"
            log_perf "✅ Stopped dashboard server (PID: $dashboard_pid)"
            stopped_any=true
        fi
        rm -f "$DASHBOARD_PID_FILE"
    fi
    
    if [[ "$stopped_any" == "true" ]]; then
        log_perf "🎯 All services stopped"
    else
        log_perf "ℹ️ No services were running"
    fi
}

show_status() {
    log_perf "📊 Performance Monitoring Status"
    echo "=================================="
    
    # Check continuous monitoring
    if [[ -f "$MONITOR_PID_FILE" ]] && kill -0 "$(cat "$MONITOR_PID_FILE")" 2>/dev/null; then
        echo "🟢 Continuous Monitoring: RUNNING (PID: $(cat "$MONITOR_PID_FILE"))"
    else
        echo "🔴 Continuous Monitoring: STOPPED"
    fi
    
    # Check dashboard server
    if [[ -f "$DASHBOARD_PID_FILE" ]] && kill -0 "$(cat "$DASHBOARD_PID_FILE")" 2>/dev/null; then
        echo "🟢 Dashboard Server: RUNNING (PID: $(cat "$DASHBOARD_PID_FILE"))"
        echo "📊 Dashboard URL: http://localhost:$DASHBOARD_PORT"
    else
        echo "🔴 Dashboard Server: STOPPED"
    fi
    
    # Show recent metrics
    if [[ -f "$PERFORMANCE_DIR/monitor_state.json" ]]; then
        echo ""
        echo "📈 Latest Performance Metrics:"
        jq -r '.metrics_history[-1] // {} | 
            "  Build Time: \(.build_time // 0)s\n  Test Time: \(.test_time // 0)s\n  Coverage: \(.coverage // 0)%\n  Memory: \(.memory_mb // 0)MB\n  Timestamp: \(.timestamp // "Never")"' \
            "$PERFORMANCE_DIR/monitor_state.json" 2>/dev/null || echo "  No metrics available"
    fi
    
    # Show system resources
    echo ""
    echo "💻 System Resources:"
    echo "  CPU Load: $(uptime | awk -F'load average:' '{print $2}' | awk '{print $1}' | sed 's/,//')"
    echo "  Memory: $(free -h 2>/dev/null | awk '/^Mem:/ {print $3 "/" $2}' || echo "Unknown")"
    echo "  Disk: $(df -h . 2>/dev/null | awk 'NR==2 {print $3 "/" $2 " (" $5 " used)"}' || echo "Unknown")"
}

run_performance_check() {
    log_perf "🧪 Running comprehensive performance check..."
    
    # Run the main performance monitor
    log_perf "Running existing performance monitor..."
    if "$SCRIPT_DIR/performance-monitor.sh" full; then
        log_perf "✅ Main performance check passed"
    else
        log $LOG_WARN "⚠️ Main performance check had issues"
    fi
    
    # Run continuous monitor check
    log_perf "Running continuous monitor check..."
    if "$SCRIPT_DIR/continuous-monitor.sh" check; then
        log_perf "✅ Continuous monitor check passed"
    else
        log $LOG_WARN "⚠️ Continuous monitor check had issues"
    fi
    
    log_perf "🎯 Performance check completed"
}

generate_comprehensive_report() {
    local report_file="$PERFORMANCE_DIR/comprehensive_report_$(date +%Y%m%d_%H%M%S).json"
    
    log_perf "📊 Generating comprehensive performance report..."
    
    # Collect all available data
    local main_monitor_data="{}"
    local continuous_data="{}"
    local system_data="{}"
    
    # Get data from main performance monitor
    if [[ -f "$PERFORMANCE_DIR/performance_history.json" ]]; then
        main_monitor_data=$(cat "$PERFORMANCE_DIR/performance_history.json")
    fi
    
    # Get data from continuous monitor
    if [[ -f "$PERFORMANCE_DIR/monitor_state.json" ]]; then
        continuous_data=$(cat "$PERFORMANCE_DIR/monitor_state.json")
    fi
    
    # Collect system information
    system_data=$(cat << EOF
{
    "cpu_count": $(nproc 2>/dev/null || echo "unknown"),
    "memory_total": "$(free -h 2>/dev/null | awk '/^Mem:/ {print $2}' || echo "unknown")",
    "disk_space": "$(df -h . 2>/dev/null | awk 'NR==2 {print $4}' || echo "unknown")",
    "load_average": "$(uptime | awk -F'load average:' '{print $2}' | awk '{print $1}' | sed 's/,//' || echo "unknown")",
    "uptime": "$(uptime -p 2>/dev/null || echo "unknown")"
}
EOF
)
    
    # Generate comprehensive report
    cat > "$report_file" << EOF
{
    "report_type": "comprehensive_performance",
    "generated_at": "$(date -Iseconds)",
    "reporting_period": {
        "start": "$(date -d '7 days ago' -Iseconds)",
        "end": "$(date -Iseconds)"
    },
    "system_info": $system_data,
    "main_monitor": $main_monitor_data,
    "continuous_monitor": $continuous_data,
    "services_status": {
        "continuous_monitoring": $(if [[ -f "$MONITOR_PID_FILE" ]] && kill -0 "$(cat "$MONITOR_PID_FILE")" 2>/dev/null; then echo "true"; else echo "false"; fi),
        "dashboard_server": $(if [[ -f "$DASHBOARD_PID_FILE" ]] && kill -0 "$(cat "$DASHBOARD_PID_FILE")" 2>/dev/null; then echo "true"; else echo "false"; fi)
    },
    "configuration": $(cat "$PERFORMANCE_DIR/config.json" 2>/dev/null || echo "{}")
}
EOF
    
    log_perf "✅ Comprehensive report generated: $report_file"
    echo "📄 Report location: $report_file"
}

open_dashboard() {
    if [[ -f "$DASHBOARD_PID_FILE" ]] && kill -0 "$(cat "$DASHBOARD_PID_FILE")" 2>/dev/null; then
        local url="http://localhost:$DASHBOARD_PORT"
        log_perf "🌐 Opening dashboard at $url"
        
        if command -v open >/dev/null 2>&1; then
            open "$url"
        elif command -v xdg-open >/dev/null 2>&1; then
            xdg-open "$url"
        else
            log_perf "💡 Open $url in your browser"
        fi
    else
        log $LOG_ERROR "Dashboard server is not running. Start it with: $0 start"
        return 1
    fi
}

main() {
    case "${1:-help}" in
        "setup")
            check_dependencies && setup_performance_monitoring
            ;;
        "start")
            check_dependencies || exit 1
            setup_performance_monitoring
            start_continuous_monitoring
            start_dashboard_server
            show_status
            ;;
        "stop")
            stop_services
            ;;
        "restart")
            stop_services
            sleep 2
            check_dependencies || exit 1
            setup_performance_monitoring
            start_continuous_monitoring
            start_dashboard_server
            show_status
            ;;
        "status")
            show_status
            ;;
        "check")
            run_performance_check
            ;;
        "report")
            generate_comprehensive_report
            ;;
        "dashboard")
            open_dashboard
            ;;
        "logs")
            echo "📋 Performance Monitoring Logs:"
            echo "==============================="
            echo "Monitor Log: $PERFORMANCE_DIR/monitor.log"
            echo "Dashboard Log: $PERFORMANCE_DIR/dashboard.log"
            echo ""
            if [[ -f "$PERFORMANCE_DIR/monitor.log" ]]; then
                echo "Recent Monitor Activity:"
                tail -n 10 "$PERFORMANCE_DIR/monitor.log"
            fi
            ;;
        "help"|*)
            cat << EOF
Integrated Performance Monitoring for Code Guardian

Usage: $0 {setup|start|stop|restart|status|check|report|dashboard|logs|help}

Commands:
  setup      - Install dependencies and configure monitoring
  start      - Start all monitoring services
  stop       - Stop all monitoring services  
  restart    - Stop and start all services
  status     - Show current status and metrics
  check      - Run comprehensive performance check
  report     - Generate detailed performance report
  dashboard  - Open dashboard in browser
  logs       - Show recent log activity
  help       - Show this help

Services:
  - Continuous monitoring (30min intervals)
  - Live web dashboard (port $DASHBOARD_PORT)
  - Performance trend analysis
  - Automated alerting

Files:
  - Configuration: $PERFORMANCE_DIR/config.json
  - Monitor state: $PERFORMANCE_DIR/monitor_state.json  
  - Reports: $PERFORMANCE_DIR/reports/

Dashboard URL: http://localhost:$DASHBOARD_PORT
EOF
            ;;
    esac
}

main "$@"