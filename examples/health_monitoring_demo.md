# Health Monitoring and Metrics Demo

This demo shows how to use Code Guardian's production-ready health monitoring and metrics collection features for enterprise deployment.

## Health Check Server

Code Guardian includes a comprehensive health check server with Kubernetes-compatible endpoints:

### Starting the Health Server

```bash
# Start health server on default port 8080
code-guardian health-server

# Start on custom port
code-guardian health-server --port 3000

# Start in detached mode (background)
code-guardian health-server --port 8080 --detach
```

### Available Endpoints

#### 1. Health Check Endpoint (`/health`)
Comprehensive health check with detailed system status:

```bash
curl http://localhost:8080/health
```

**Response:**
```json
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
```

#### 2. Readiness Probe (`/ready`)
Kubernetes readiness probe for load balancer integration:

```bash
curl http://localhost:8080/ready
```

**Response:**
```json
{
  "status": "ready",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

#### 3. Liveness Probe (`/live`)
Basic liveness check for container orchestration:

```bash
curl http://localhost:8080/live
```

**Response:**
```json
{
  "status": "alive",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

#### 4. Prometheus Metrics (`/metrics`)
Comprehensive metrics in Prometheus format:

```bash
curl http://localhost:8080/metrics
```

**Response:**
```
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
code_guardian_scan_duration_seconds_sum 180.5
code_guardian_scan_duration_seconds_count 150

# HELP code_guardian_memory_usage_bytes Current memory usage in bytes
# TYPE code_guardian_memory_usage_bytes gauge
code_guardian_memory_usage_bytes 89456640

# HELP code_guardian_llm_detections_total Total number of LLM-specific detections
# TYPE code_guardian_llm_detections_total counter
code_guardian_llm_detections_total 23
```

## Health Status Levels

The health check system uses three status levels:

- **healthy**: All systems operational
- **degraded**: Minor issues but service still functional
- **unhealthy**: Critical issues requiring immediate attention

### Health Check Components

1. **Database**: Database connectivity and performance
2. **Scanner**: Core scanning engine functionality
3. **Memory**: System memory usage (warnings at 80%, critical at 90%)
4. **Disk**: Disk space usage (warnings at 85%, critical at 95%)

## Kubernetes Integration

### Deployment Configuration

```yaml
# kubernetes/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: code-guardian
spec:
  replicas: 3
  selector:
    matchLabels:
      app: code-guardian
  template:
    metadata:
      labels:
        app: code-guardian
    spec:
      containers:
      - name: code-guardian
        image: code-guardian:latest
        ports:
        - containerPort: 8080
          name: health
        - containerPort: 9090
          name: metrics
        
        # Health checks
        livenessProbe:
          httpGet:
            path: /live
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          
        # Resource limits
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

### Service Configuration

```yaml
# kubernetes/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: code-guardian-service
  labels:
    app: code-guardian
spec:
  selector:
    app: code-guardian
  ports:
  - name: health
    port: 8080
    targetPort: 8080
  - name: metrics
    port: 9090
    targetPort: 8080
  type: ClusterIP
```

## Prometheus Integration

### Prometheus Configuration

```yaml
# prometheus/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'code-guardian'
    static_configs:
      - targets: ['code-guardian-service:9090']
    scrape_interval: 30s
    metrics_path: /metrics
    
rule_files:
  - "code_guardian_alerts.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093
```

### Alert Rules

```yaml
# prometheus/code_guardian_alerts.yml
groups:
- name: code-guardian
  rules:
  - alert: CodeGuardianDown
    expr: up{job="code-guardian"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Code Guardian instance is down"
      
  - alert: HighMemoryUsage
    expr: code_guardian_memory_usage_bytes > 400000000
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "Code Guardian memory usage is high"
      
  - alert: SlowScans
    expr: rate(code_guardian_scan_duration_seconds_sum[5m]) > 10
    for: 2m
    labels:
      severity: warning
    annotations:
      summary: "Code Guardian scans are taking too long"
```

## Grafana Dashboard

### Example Dashboard Queries

**Scan Rate:**
```promql
rate(code_guardian_scans_total[5m])
```

**Average Scan Duration:**
```promql
rate(code_guardian_scan_duration_seconds_sum[5m]) / rate(code_guardian_scan_duration_seconds_count[5m])
```

**Memory Usage:**
```promql
code_guardian_memory_usage_bytes
```

**Issues Found Rate:**
```promql
rate(code_guardian_issues_found_total[5m])
```

**LLM Detection Rate:**
```promql
rate(code_guardian_llm_detections_total[5m])
```

## CI/CD Integration

### GitHub Actions with Health Checks

```yaml
# .github/workflows/deploy.yml
name: Deploy with Health Checks

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Deploy to staging
      run: |
        kubectl apply -f kubernetes/
        
    - name: Wait for deployment
      run: |
        kubectl rollout status deployment/code-guardian --timeout=300s
        
    - name: Health check
      run: |
        # Wait for health endpoint to be ready
        timeout 60 bash -c 'until curl -f http://staging.example.com/health; do sleep 2; done'
        
    - name: Smoke test
      run: |
        # Run basic functionality test
        curl -f http://staging.example.com/ready
        curl -f http://staging.example.com/metrics
```

## Production Monitoring Commands

### Metrics Collection

```bash
# Export current metrics to file
code-guardian metrics ./src --output metrics.txt

# Verbose metrics with detailed logging
code-guardian metrics ./src --verbose

# Monitor specific path
code-guardian metrics /app/source --output /monitoring/metrics.json
```

### CI Gate Integration

```bash
# Fail build if more than 0 critical or 5 high severity issues
code-guardian ci-gate ./src --max-critical 0 --max-high 5 --output ci-report.json

# With custom configuration
code-guardian ci-gate ./src --config production.toml --max-critical 0 --max-high 3
```

### Pre-commit Hooks

```bash
# Fast pre-commit check (staged files only)
code-guardian pre-commit ./src --staged-only --fast

# Full pre-commit scan
code-guardian pre-commit ./src
```

## Troubleshooting

### Common Issues

1. **Health check returns 503**
   - Check memory and disk usage
   - Verify scanner initialization
   - Review application logs

2. **Metrics endpoint not responding**
   - Ensure metrics system is initialized
   - Check Prometheus configuration
   - Verify network connectivity

3. **High memory usage alerts**
   - Review scan configuration
   - Check for memory leaks
   - Consider reducing parallel workers

### Diagnostic Commands

```bash
# Check health status
curl -s http://localhost:8080/health | jq '.checks'

# Get current metrics
curl -s http://localhost:8080/metrics | grep memory

# Monitor scan performance
watch -n 5 'curl -s http://localhost:8080/metrics | grep scan_duration'
```

## Best Practices

1. **Always use readiness and liveness probes** in Kubernetes deployments
2. **Set appropriate resource limits** based on your workload
3. **Monitor key metrics** like scan duration and memory usage
4. **Set up alerting** for critical health check failures
5. **Use staged deployments** with health check validation
6. **Regular health endpoint testing** in CI/CD pipelines

This health monitoring system provides enterprise-grade observability for Code Guardian deployments, ensuring reliable operation in production environments.