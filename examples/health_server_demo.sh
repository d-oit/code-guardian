#!/bin/bash

# Health Server Demo Script
# Demonstrates the production-ready health monitoring capabilities

set -e

echo "🚀 Code Guardian Health Server Demo"
echo "==================================="
echo

# Function to check if a port is available
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo "❌ Port $port is already in use"
        return 1
    else
        echo "✅ Port $port is available"
        return 0
    fi
}

# Function to wait for server to start
wait_for_server() {
    local port=$1
    local max_attempts=30
    local attempt=1
    
    echo "⏳ Waiting for health server to start on port $port..."
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s -f "http://localhost:$port/live" > /dev/null 2>&1; then
            echo "✅ Health server is running!"
            return 0
        fi
        
        echo "   Attempt $attempt/$max_attempts - waiting..."
        sleep 1
        attempt=$((attempt + 1))
    done
    
    echo "❌ Health server failed to start within timeout"
    return 1
}

# Function to test endpoint
test_endpoint() {
    local endpoint=$1
    local description=$2
    
    echo "🔍 Testing $description ($endpoint)"
    
    response=$(curl -s -w "HTTP_CODE:%{http_code}" "http://localhost:$HEALTH_PORT$endpoint")
    http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
    body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
    
    if [ "$http_code" = "200" ]; then
        echo "   ✅ Status: $http_code"
        echo "   📄 Response preview:"
        echo "$body" | jq '.' 2>/dev/null | head -10 || echo "$body" | head -5
    else
        echo "   ⚠️  Status: $http_code"
        echo "   📄 Response: $body"
    fi
    echo
}

# Main demo
main() {
    echo "📋 Prerequisites Check"
    echo "====================="
    
    # Check if required tools are available
    for tool in curl jq lsof; do
        if command -v $tool >/dev/null 2>&1; then
            echo "✅ $tool is available"
        else
            echo "❌ $tool is required but not installed"
            exit 1
        fi
    done
    echo
    
    # Set port
    HEALTH_PORT=8080
    
    echo "🔧 Setup"
    echo "========"
    echo "Health server port: $HEALTH_PORT"
    
    # Check if port is available
    if ! check_port $HEALTH_PORT; then
        echo "Please free up port $HEALTH_PORT or modify the script to use a different port"
        exit 1
    fi
    echo
    
    echo "🎬 Starting Health Server Demo"
    echo "=============================="
    
    # Start health server in background
    echo "🚀 Starting Code Guardian health server..."
    
    # For demo purposes, we'll simulate the health server since we can't easily run it
    # In a real scenario, you would run: code-guardian health-server --port $HEALTH_PORT &
    
    echo "📝 Demo: Simulating health server responses"
    echo "   In production, you would run:"
    echo "   $ code-guardian health-server --port $HEALTH_PORT"
    echo
    
    # Create mock responses for demonstration
    echo "🧪 Health Check Endpoint Examples"
    echo "================================="
    
    echo "1. 🏥 Comprehensive Health Check (/health)"
    echo "   Response format:"
    cat << 'EOF'
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime_seconds": 3600,
  "checks": {
    "database": "healthy",
    "scanner": "healthy",
    "memory": "healthy",
    "disk": "healthy"
  }
}
EOF
    echo
    
    echo "2. 🎯 Readiness Probe (/ready)"
    echo "   Response format:"
    cat << 'EOF'
{
  "status": "ready",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
EOF
    echo
    
    echo "3. 💓 Liveness Probe (/live)"
    echo "   Response format:"
    cat << 'EOF'
{
  "status": "alive",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
EOF
    echo
    
    echo "4. 📊 Prometheus Metrics (/metrics)"
    echo "   Response format:"
    cat << 'EOF'
# HELP code_guardian_scans_total Total number of scans performed
# TYPE code_guardian_scans_total counter
code_guardian_scans_total 150

# HELP code_guardian_files_scanned_total Total number of files scanned
# TYPE code_guardian_files_scanned_total counter
code_guardian_files_scanned_total 50000

# HELP code_guardian_scan_duration_seconds Time spent scanning in seconds
# TYPE code_guardian_scan_duration_seconds histogram
code_guardian_scan_duration_seconds_bucket{le="0.1"} 10
code_guardian_scan_duration_seconds_bucket{le="0.5"} 45
code_guardian_scan_duration_seconds_bucket{le="1.0"} 120
code_guardian_scan_duration_seconds_bucket{le="+Inf"} 150

# HELP code_guardian_memory_usage_bytes Current memory usage in bytes
# TYPE code_guardian_memory_usage_bytes gauge
code_guardian_memory_usage_bytes 89456640

# HELP code_guardian_llm_detections_total Total number of LLM-specific detections
# TYPE code_guardian_llm_detections_total counter
code_guardian_llm_detections_total 23
EOF
    echo
    
    echo "🐳 Kubernetes Integration Example"
    echo "================================="
    echo "Health server supports Kubernetes deployment with:"
    echo
    echo "Liveness Probe Configuration:"
    cat << 'EOF'
livenessProbe:
  httpGet:
    path: /live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
EOF
    echo
    
    echo "Readiness Probe Configuration:"
    cat << 'EOF'
readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
  timeoutSeconds: 3
EOF
    echo
    
    echo "📈 Monitoring Integration"
    echo "========================"
    echo "Prometheus service discovery annotations:"
    cat << 'EOF'
metadata:
  annotations:
    prometheus.io/scrape: "true"
    prometheus.io/path: "/metrics"
    prometheus.io/port: "8080"
EOF
    echo
    
    echo "🎛️ CLI Commands Demo"
    echo "===================="
    echo "Available health monitoring commands:"
    echo
    echo "1. Start health server:"
    echo "   $ code-guardian health-server --port 8080"
    echo
    echo "2. Start in background:"
    echo "   $ code-guardian health-server --port 8080 --detach"
    echo
    echo "3. Export metrics:"
    echo "   $ code-guardian metrics ./src --output metrics.json --verbose"
    echo
    echo "4. CI gate with thresholds:"
    echo "   $ code-guardian ci-gate ./src --max-critical 0 --max-high 5"
    echo
    echo "5. Pre-commit scanning:"
    echo "   $ code-guardian pre-commit ./src --staged-only --fast"
    echo
    echo "6. Production readiness check:"
    echo "   $ code-guardian production-check ./src --format html --output report.html"
    echo
    
    echo "✅ Demo completed successfully!"
    echo
    echo "🔗 Next Steps:"
    echo "   1. Build Code Guardian: cargo build --release"
    echo "   2. Start health server: ./target/release/code-guardian health-server"
    echo "   3. Test endpoints: curl http://localhost:8080/health"
    echo "   4. Integrate with monitoring: See examples/health_monitoring_demo.md"
    echo
    echo "📚 Documentation:"
    echo "   - Health Monitoring Guide: examples/health_monitoring_demo.md"
    echo "   - Kubernetes Integration: examples/health_monitoring_demo.md#kubernetes-integration"
    echo "   - Prometheus Setup: examples/health_monitoring_demo.md#prometheus-integration"
}

# Run the demo
main "$@"