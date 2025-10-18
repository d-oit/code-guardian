# CI/CD Integration Guide

This guide shows how to integrate Code-Guardian into your CI/CD pipelines for automated security scanning.

## Overview

Integrating Code-Guardian into CI/CD provides:
- **Automated scanning** on every commit/PR
- **Early detection** of security issues
- **Quality gates** to prevent problematic code
- **Historical tracking** of code quality trends

## Quick Start Examples

### GitHub Actions

#### Basic Security Scan
```yaml
name: Security Scan
on: [push, pull_request]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Code-Guardian
        run: |
          curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
          sudo mv code-guardian /usr/local/bin/

      - name: Run Security Scan
        run: |
          code-guardian scan . \
            --format json \
            --output security-report.json \
            --fail-on-critical \
            --fail-on-high

      - name: Upload Report
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: security-report
          path: security-report.json
```

#### Advanced Setup with Quality Gates
```yaml
name: Code Quality
on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Code-Guardian
        run: |
          curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
          sudo mv code-guardian /usr/local/bin/

      - name: Run Comprehensive Scan
        id: scan
        run: |
          code-guardian scan . \
            --profile comprehensive \
            --format json \
            --output quality-report.json \
            --max-critical 0 \
            --max-high 3 \
            --max-medium 10
        continue-on-error: true

      - name: Comment PR
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = JSON.parse(fs.readFileSync('quality-report.json', 'utf8'));

            const summary = `## 🔍 Code Quality Report
            **Issues Found:** ${report.summary.total_matches}
            - Critical: ${report.summary.critical}
            - High: ${report.summary.high}
            - Medium: ${report.summary.medium}
            - Low: ${report.summary.low}

            ${report.summary.critical > 0 || report.summary.high > 3 ? '⚠️  Quality gates failed' : '✅ Quality checks passed'}`;

            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: summary
            });

      - name: Fail on Quality Gate
        if: steps.scan.outcome == 'failure'
        run: |
          echo "❌ Quality gates failed. Check the report above."
          exit 1
```

### GitLab CI

#### Basic Pipeline
```yaml
stages:
  - test
  - security

security_scan:
  stage: security
  image: d-oit/code-guardian:latest
  script:
    - code-guardian scan . --format json --output security-report.json
  artifacts:
    reports:
      junit: security-report.json
  only:
    - merge_requests
    - main
```

#### Advanced Pipeline with Quality Gates
```yaml
stages:
  - build
  - test
  - security
  - deploy

security_scan:
  stage: security
  image: d-oit/code-guardian:latest
  script:
    - |
      code-guardian scan . \
        --profile security \
        --format junit \
        --output security-report.xml \
        --fail-on-critical \
        --fail-on-high
  artifacts:
    reports:
      junit: security-report.xml
  coverage: '/Security Coverage: \d+\.\d+%/'
  only:
    - merge_requests
    - main

quality_gate:
  stage: security
  image: d-oit/code-guardian:latest
  script:
    - |
      code-guardian production-check . \
        --max-critical 0 \
        --max-high 5 \
        --max-medium 20 \
        --format json \
        --output quality-gate.json
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: quality-gate.json
  only:
    - main
```

### Jenkins Pipeline

#### Declarative Pipeline
```groovy
pipeline {
    agent any

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Security Scan') {
            steps {
                sh '''
                    # Install Code-Guardian
                    curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
                    chmod +x code-guardian

                    # Run scan
                    ./code-guardian scan . \
                        --format json \
                        --output security-report.json \
                        --fail-on-critical
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'security-report.json', fingerprint: true
                    publishHTML target: [
                        allowMissing: true,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: '.',
                        reportFiles: 'security-report.json',
                        reportName: 'Security Report'
                    ]
                }
                failure {
                    script {
                        def report = readJSON file: 'security-report.json'
                        echo "Security scan failed: ${report.summary.critical} critical, ${report.summary.high} high severity issues"
                    }
                }
            }
        }
    }
}
```

#### Scripted Pipeline
```groovy
node {
    try {
        stage('Checkout') {
            checkout scm
        }

        stage('Install Code-Guardian') {
            sh '''
                curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
                chmod +x code-guardian
            '''
        }

        stage('Security Scan') {
            sh './code-guardian scan . --format json --output security-report.json'
        }

        stage('Quality Check') {
            def report = readJSON file: 'security-report.json'
            def critical = report.summary.critical
            def high = report.summary.high

            if (critical > 0) {
                error("❌ ${critical} critical security issues found!")
            }
            if (high > 5) {
                error("⚠️ ${high} high severity issues found (max allowed: 5)")
            }

            echo "✅ Quality checks passed: ${critical} critical, ${high} high severity issues"
        }

    } catch (Exception e) {
        currentBuild.result = 'FAILURE'
        throw e
    } finally {
        archiveArtifacts artifacts: 'security-report.json', allowEmptyArchive: true
    }
}
```

## CircleCI

```yaml
version: 2.1

jobs:
  security-scan:
    docker:
      - image: cimg/rust:1.75
    steps:
      - checkout
      - run:
          name: Install Code-Guardian
          command: |
            curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
            sudo mv code-guardian /usr/local/bin/
      - run:
          name: Run Security Scan
          command: |
            code-guardian scan . \
              --format json \
              --output security-report.json \
              --fail-on-critical
      - store_artifacts:
          path: security-report.json
      - run:
          name: Check Results
          command: |
            # Parse JSON and check for failures
            CRITICAL=$(jq '.summary.critical' security-report.json)
            HIGH=$(jq '.summary.high' security-report.json)

            if [ "$CRITICAL" -gt 0 ]; then
              echo "❌ $CRITICAL critical issues found"
              exit 1
            fi

            if [ "$HIGH" -gt 5 ]; then
              echo "⚠️ $HIGH high severity issues found (max: 5)"
              exit 1
            fi

            echo "✅ Security checks passed"

workflows:
  version: 2
  build-and-scan:
    jobs:
      - security-scan
```

## Azure DevOps

### YAML Pipeline
```yaml
trigger:
  - main
  - develop

pool:
  vmImage: 'ubuntu-latest'

steps:
  - checkout: self

  - bash: |
      # Install Code-Guardian
      curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
      sudo mv code-guardian /usr/local/bin/
    displayName: 'Install Code-Guardian'

  - bash: |
      # Run security scan
      code-guardian scan . \
        --format json \
        --output $(Build.ArtifactStagingDirectory)/security-report.json \
        --fail-on-critical \
        --fail-on-high
    displayName: 'Run Security Scan'

  - task: PublishBuildArtifacts@1
    displayName: 'Publish Security Report'
    inputs:
      pathToPublish: '$(Build.ArtifactStagingDirectory)/security-report.json'
      artifactName: 'SecurityReport'
    condition: always()

  - bash: |
      # Quality gate check
      REPORT="$(Build.ArtifactStagingDirectory)/security-report.json"
      CRITICAL=$(jq '.summary.critical' "$REPORT")
      HIGH=$(jq '.summary.high' "$REPORT")

      if [ "$CRITICAL" -gt 0 ]; then
        echo "##vso[task.logissue type=error]Critical security issues found: $CRITICAL"
        exit 1
      fi

      if [ "$HIGH" -gt 3 ]; then
        echo "##vso[task.logissue type=warning]High severity issues found: $HIGH (threshold: 3)"
        exit 1
      fi
    displayName: 'Quality Gate Check'
```

## Travis CI

```yaml
language: rust
rust:
  - stable

cache:
  cargo: true
  directories:
    - /usr/local/bin

install:
  - |
    # Install Code-Guardian if not cached
    if ! command -v code-guardian &> /dev/null; then
      curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
      sudo mv code-guardian /usr/local/bin/
    fi

script:
  - cargo test
  - |
    # Run security scan
    code-guardian scan . \
      --format json \
      --output security-report.json \
      --fail-on-critical

after_script:
  - |
    # Display results
    if [ -f security-report.json ]; then
      echo "Security Scan Results:"
      jq '.summary' security-report.json
    fi

deploy:
  - provider: releases
    api_key: $GITHUB_TOKEN
    file: security-report.json
    skip_cleanup: true
    on:
      tags: true
```

## Advanced Configuration

### Custom Quality Gates

```yaml
# .code-guardian-config.toml
[quality_gates]
max_critical = 0
max_high = 5
max_medium = 20
max_low = 100

# Allow specific patterns
[exceptions]
allowed_patterns = [
    "TODO.*production",  # Allow TODOs mentioning production
    "console\.log.*debug"  # Allow debug console.logs
]
```

### Incremental Scanning

```yaml
# Only scan changed files in PRs
- name: Run Incremental Scan
  if: github.event_name == 'pull_request'
  run: |
    # Get changed files
    CHANGED_FILES=$(git diff --name-only ${{ github.event.pull_request.base.sha }} ${{ github.sha }} | tr '\n' ' ')

    if [ -n "$CHANGED_FILES" ]; then
      code-guardian scan $CHANGED_FILES \
        --format json \
        --output pr-security-report.json
    else
      echo "No files changed, skipping scan"
    fi
```

### Parallel Scanning for Large Projects

```yaml
# Split scanning across multiple jobs
strategy:
  matrix:
    scan-type: [security, performance, quality]

jobs:
  scan-${{ matrix.scan-type }}:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Code-Guardian
        run: |
          curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
          sudo mv code-guardian /usr/local/bin/

      - name: Run ${{ matrix.scan-type }} Scan
        run: |
          code-guardian scan . \
            --profile ${{ matrix.scan-type }} \
            --format json \
            --output ${{ matrix.scan-type }}-report.json
```

## Best Practices

### 1. **Fail Fast on Critical Issues**
Always fail the build on critical security issues:
```bash
code-guardian scan . --fail-on-critical
```

### 2. **Use Appropriate Profiles**
Choose the right profile for your use case:
- `security`: For security-focused scans
- `comprehensive`: For full analysis
- `basic`: For quick checks

### 3. **Set Realistic Quality Gates**
Start with lenient gates and tighten over time:
```yaml
# Initial gates
max_critical: 0
max_high: 10
max_medium: 50

# Target gates (after improvements)
max_critical: 0
max_high: 3
max_medium: 15
```

### 4. **Cache Dependencies**
Speed up CI by caching the Code-Guardian binary:
```yaml
- name: Cache Code-Guardian
  uses: actions/cache@v3
  with:
    path: ~/code-guardian
    key: code-guardian-${{ runner.os }}-${{ hashFiles('.code-guardian-version') }}
```

### 5. **Monitor Trends**
Track code quality over time:
```yaml
- name: Upload metrics to monitoring
  run: |
    # Extract metrics
    CRITICAL=$(jq '.summary.critical' security-report.json)
    HIGH=$(jq '.summary.high' security-report.json)

    # Send to monitoring system
    curl -X POST $MONITORING_URL \
      -d "{\"project\":\"$GITHUB_REPOSITORY\",\"critical\":$CRITICAL,\"high\":$HIGH,\"timestamp\":\"$(date -Iseconds)\"}"
```

## Troubleshooting

### Common Issues

#### Scan Times Out
```yaml
# Increase timeout and reduce threads
- run: |
    code-guardian scan . \
      --max-threads 2 \
      --timeout 600 \
      --format json
```

#### False Positives
```toml
# .code-guardian-config.toml
[exceptions]
allowed_patterns = [
    "test.*password",  # Allow passwords in tests
    "example.*api_key"  # Allow API keys in examples
]
```

#### Large Codebases
```yaml
# Use incremental scanning
- run: |
    code-guardian scan . \
      --incremental \
      --cache-size 1000 \
      --streaming
```

#### Permission Issues
```yaml
# Ensure proper permissions
- run: |
    chmod +x code-guardian
    sudo mv code-guardian /usr/local/bin/ || mv code-guardian ~/bin/
```

## Integration Examples

### With SonarQube
```yaml
- name: Run Code-Guardian
  run: |
    code-guardian scan . \
      --format sonarqube \
      --output code-guardian-report.json

- name: Run SonarQube Scanner
  uses: sonarsource/sonarqube-scan-action@v2
  with:
    args: >
      -Dsonar.externalIssuesReportPaths=code-guardian-report.json
```

### With DefectDojo
```yaml
- name: Upload to DefectDojo
  run: |
    curl -X POST $DEFECTDOJO_URL/api/v2/import-scan/ \
      -H "Authorization: Token $DEFECTDOJO_TOKEN" \
      -F "scan_type=Code Guardian" \
      -F "file=@security-report.json" \
      -F "product_name=$GITHUB_REPOSITORY"
```

This comprehensive CI/CD integration guide should help you automate Code-Guardian scanning in your development workflow.