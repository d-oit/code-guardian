#!/usr/bin/env python3
"""
Performance Dashboard Server - Real-time web dashboard for Code Guardian performance monitoring
Serves interactive dashboard with live updates and historical data visualization
"""

import json
import os
import time
import threading
from datetime import datetime, timedelta
from pathlib import Path
from flask import Flask, render_template_string, jsonify, request
import subprocess

app = Flask(__name__)

# Configuration
PERFORMANCE_DIR = Path("performance")
STATE_FILE = PERFORMANCE_DIR / "monitor_state.json"
DASHBOARD_DIR = PERFORMANCE_DIR / "dashboard"
UPDATE_INTERVAL = 30  # seconds

class PerformanceData:
    def __init__(self):
        self.data = {}
        self.load_data()
    
    def load_data(self):
        """Load performance data from files"""
        try:
            if STATE_FILE.exists():
                with open(STATE_FILE) as f:
                    self.data = json.load(f)
            else:
                self.data = {
                    "consecutive_failures": 0,
                    "last_successful_run": "",
                    "metrics_history": []
                }
        except Exception as e:
            print(f"Error loading data: {e}")
            self.data = {"consecutive_failures": 0, "last_successful_run": "", "metrics_history": []}
    
    def get_latest_metrics(self):
        """Get the most recent performance metrics"""
        if self.data.get("metrics_history"):
            return self.data["metrics_history"][-1]
        return {}
    
    def get_trend_data(self, metric, hours=24):
        """Get trend data for a specific metric over the last N hours"""
        cutoff_time = datetime.now() - timedelta(hours=hours)
        
        trends = []
        for entry in self.data.get("metrics_history", []):
            try:
                entry_time = datetime.fromisoformat(entry.get("timestamp", "").replace("Z", "+00:00"))
                if entry_time >= cutoff_time:
                    trends.append({
                        "timestamp": entry["timestamp"],
                        "value": entry.get(metric, 0)
                    })
            except Exception:
                continue
        
        return trends

# Global performance data instance
perf_data = PerformanceData()

# Background thread to refresh data
def refresh_data():
    while True:
        perf_data.load_data()
        time.sleep(UPDATE_INTERVAL)

refresh_thread = threading.Thread(target=refresh_data, daemon=True)
refresh_thread.start()

# Dashboard HTML template
DASHBOARD_HTML = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Code Guardian - Live Performance Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f7fa;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            border-radius: 12px;
            margin-bottom: 30px;
            text-align: center;
            box-shadow: 0 8px 32px rgba(0,0,0,0.1);
        }
        .status-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .status-card {
            background: white;
            padding: 25px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
            text-align: center;
            position: relative;
            transition: transform 0.2s;
        }
        .status-card:hover {
            transform: translateY(-2px);
        }
        .metric-value {
            font-size: 2.5em;
            font-weight: 700;
            margin: 10px 0;
        }
        .metric-label {
            color: #666;
            font-size: 0.9em;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 15px;
        }
        .status-indicator {
            position: absolute;
            top: 15px;
            right: 15px;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }
        .status-good { background-color: #10b981; }
        .status-warning { background-color: #f59e0b; }
        .status-critical { background-color: #ef4444; }
        
        .good { color: #10b981; }
        .warning { color: #f59e0b; }
        .critical { color: #ef4444; }
        
        .charts-section {
            background: white;
            padding: 30px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
            margin-bottom: 30px;
        }
        .charts-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
            gap: 30px;
        }
        .chart-container {
            position: relative;
            height: 300px;
        }
        .alerts-section {
            background: white;
            padding: 25px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
        }
        .alert-item {
            padding: 12px;
            margin: 8px 0;
            border-radius: 8px;
            font-family: monospace;
            font-size: 0.9em;
        }
        .alert-warning { background-color: #fef3c7; color: #92400e; }
        .alert-critical { background-color: #fee2e2; color: #991b1b; }
        .alert-info { background-color: #dbeafe; color: #1e40af; }
        
        .refresh-info {
            position: fixed;
            top: 20px;
            right: 20px;
            background: rgba(0,0,0,0.7);
            color: white;
            padding: 8px 15px;
            border-radius: 20px;
            font-size: 0.8em;
        }
        .controls {
            margin-bottom: 20px;
            text-align: center;
        }
        .btn {
            background: #667eea;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            margin: 0 5px;
            transition: background 0.2s;
        }
        .btn:hover { background: #5a67d8; }
    </style>
</head>
<body>
    <div class="refresh-info">
        🔄 Auto-refresh: <span id="refresh-countdown">30</span>s
    </div>

    <div class="container">
        <div class="header">
            <h1>⚡ Code Guardian Live Performance Dashboard</h1>
            <p>Real-time performance monitoring with trend analysis</p>
            <div class="controls">
                <button class="btn" onclick="refreshData()">🔄 Refresh Now</button>
                <button class="btn" onclick="runPerformanceCheck()">🧪 Run Check</button>
                <button class="btn" onclick="toggleAutoRefresh()">⏸️ <span id="auto-refresh-status">Pause</span></button>
            </div>
        </div>

        <div class="status-grid" id="status-grid">
            <!-- Status cards will be populated by JavaScript -->
        </div>

        <div class="charts-section">
            <h2>📈 Performance Trends (Last 24 Hours)</h2>
            <div class="charts-grid">
                <div class="chart-container">
                    <canvas id="buildTimeChart"></canvas>
                </div>
                <div class="chart-container">
                    <canvas id="testTimeChart"></canvas>
                </div>
                <div class="chart-container">
                    <canvas id="coverageChart"></canvas>
                </div>
                <div class="chart-container">
                    <canvas id="memoryChart"></canvas>
                </div>
            </div>
        </div>

        <div class="alerts-section">
            <h2>🚨 Recent Alerts & Events</h2>
            <div id="alerts-container">
                <!-- Alerts will be populated by JavaScript -->
            </div>
        </div>
    </div>

    <script>
        let autoRefresh = true;
        let refreshCountdown = 30;
        let charts = {};

        function updateCountdown() {
            if (autoRefresh && refreshCountdown > 0) {
                document.getElementById('refresh-countdown').textContent = refreshCountdown;
                refreshCountdown--;
            } else if (autoRefresh) {
                refreshData();
                refreshCountdown = 30;
            }
        }

        function toggleAutoRefresh() {
            autoRefresh = !autoRefresh;
            const status = document.getElementById('auto-refresh-status');
            status.textContent = autoRefresh ? 'Pause' : 'Resume';
            if (autoRefresh) refreshCountdown = 30;
        }

        function getStatusClass(metric, value, threshold, comparison) {
            if (comparison === 'gt') {
                return value > threshold ? 'critical' : 'good';
            } else {
                return value < threshold ? 'critical' : 'good';
            }
        }

        function refreshData() {
            fetch('/api/metrics')
                .then(response => response.json())
                .then(data => {
                    updateStatusCards(data.latest);
                    updateCharts(data.trends);
                    updateAlerts(data.alerts);
                    refreshCountdown = 30;
                })
                .catch(error => console.error('Error fetching data:', error));
        }

        function updateStatusCards(metrics) {
            const grid = document.getElementById('status-grid');
            const buildStatus = getStatusClass('build_time', metrics.build_time || 0, 180, 'gt');
            const testStatus = getStatusClass('test_time', metrics.test_time || 0, 120, 'gt');
            const coverageStatus = getStatusClass('coverage', metrics.coverage || 0, 82, 'lt');
            const memoryStatus = getStatusClass('memory_mb', metrics.memory_mb || 0, 200, 'gt');

            grid.innerHTML = `
                <div class="status-card">
                    <div class="status-indicator status-${buildStatus}"></div>
                    <div class="metric-label">Build Time</div>
                    <div class="metric-value ${buildStatus}">${(metrics.build_time || 0).toFixed(1)}s</div>
                    <div>Threshold: <180s</div>
                </div>
                <div class="status-card">
                    <div class="status-indicator status-${testStatus}"></div>
                    <div class="metric-label">Test Time</div>
                    <div class="metric-value ${testStatus}">${(metrics.test_time || 0).toFixed(1)}s</div>
                    <div>Threshold: <120s</div>
                </div>
                <div class="status-card">
                    <div class="status-indicator status-${coverageStatus}"></div>
                    <div class="metric-label">Test Coverage</div>
                    <div class="metric-value ${coverageStatus}">${(metrics.coverage || 0).toFixed(1)}%</div>
                    <div>Threshold: >82%</div>
                </div>
                <div class="status-card">
                    <div class="status-indicator status-${memoryStatus}"></div>
                    <div class="metric-label">Memory Usage</div>
                    <div class="metric-value ${memoryStatus}">${(metrics.memory_mb || 0).toFixed(1)}MB</div>
                    <div>Threshold: <200MB</div>
                </div>
            `;
        }

        function updateCharts(trends) {
            updateChart('buildTimeChart', 'Build Time (seconds)', trends.build_time, '#667eea');
            updateChart('testTimeChart', 'Test Time (seconds)', trends.test_time, '#f093fb');
            updateChart('coverageChart', 'Coverage (%)', trends.coverage, '#4facfe');
            updateChart('memoryChart', 'Memory Usage (MB)', trends.memory_mb, '#43e97b');
        }

        function updateChart(canvasId, label, data, color) {
            const ctx = document.getElementById(canvasId).getContext('2d');
            
            if (charts[canvasId]) {
                charts[canvasId].destroy();
            }

            charts[canvasId] = new Chart(ctx, {
                type: 'line',
                data: {
                    labels: data.map(d => new Date(d.timestamp).toLocaleTimeString()),
                    datasets: [{
                        label: label,
                        data: data.map(d => d.value),
                        borderColor: color,
                        backgroundColor: color + '20',
                        fill: true,
                        tension: 0.4
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        y: {
                            beginAtZero: true
                        }
                    },
                    plugins: {
                        legend: {
                            display: true,
                            position: 'top'
                        }
                    }
                }
            });
        }

        function updateAlerts(alerts) {
            const container = document.getElementById('alerts-container');
            if (alerts.length === 0) {
                container.innerHTML = '<p>No recent alerts 🎉</p>';
                return;
            }

            container.innerHTML = alerts.map(alert => {
                const severity = alert.includes('CRITICAL') ? 'critical' : 
                               alert.includes('WARNING') ? 'warning' : 'info';
                return `<div class="alert-item alert-${severity}">${alert}</div>`;
            }).join('');
        }

        function runPerformanceCheck() {
            fetch('/api/run-check', { method: 'POST' })
                .then(response => response.json())
                .then(data => {
                    alert(data.message);
                    setTimeout(refreshData, 2000); // Refresh after 2 seconds
                })
                .catch(error => console.error('Error running check:', error));
        }

        // Initialize
        setInterval(updateCountdown, 1000);
        refreshData();
    </script>
</body>
</html>
"""

@app.route('/')
def dashboard():
    return render_template_string(DASHBOARD_HTML)

@app.route('/api/metrics')
def get_metrics():
    latest = perf_data.get_latest_metrics()
    
    trends = {
        'build_time': perf_data.get_trend_data('build_time'),
        'test_time': perf_data.get_trend_data('test_time'),
        'coverage': perf_data.get_trend_data('coverage'),
        'memory_mb': perf_data.get_trend_data('memory_mb')
    }
    
    # Get recent alerts (mock for now)
    alerts = []
    try:
        alert_file = PERFORMANCE_DIR / "alerts.log"
        if alert_file.exists():
            with open(alert_file) as f:
                alerts = f.readlines()[-10:]  # Last 10 alerts
                alerts = [line.strip() for line in alerts if line.strip()]
    except Exception:
        pass
    
    return jsonify({
        'latest': latest,
        'trends': trends,
        'alerts': alerts,
        'status': perf_data.data.get('consecutive_failures', 0)
    })

@app.route('/api/run-check', methods=['POST'])
def run_check():
    try:
        result = subprocess.run(['./scripts/continuous-monitor.sh', 'check'], 
                              capture_output=True, text=True, timeout=300)
        if result.returncode == 0:
            return jsonify({'message': 'Performance check completed successfully!'})
        else:
            return jsonify({'message': f'Performance check failed: {result.stderr}'})
    except subprocess.TimeoutExpired:
        return jsonify({'message': 'Performance check timed out'})
    except Exception as e:
        return jsonify({'message': f'Error running check: {str(e)}'})

if __name__ == '__main__':
    print("🚀 Starting Code Guardian Performance Dashboard Server")
    print("📊 Dashboard will be available at: http://localhost:8080")
    print("🔄 Auto-refresh interval: 30 seconds")
    print("💡 Use Ctrl+C to stop the server")
    
    # Ensure performance directory exists
    PERFORMANCE_DIR.mkdir(exist_ok=True)
    DASHBOARD_DIR.mkdir(exist_ok=True)
    
    app.run(host='0.0.0.0', port=8080, debug=False)