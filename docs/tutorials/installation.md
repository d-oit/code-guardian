# Installation Guide

This guide covers installing Code-Guardian for different use cases and environments.

## Prerequisites

- **Rust**: 1.70+ (for building from source)
- **Cargo**: Latest stable version
- **Git**: For cloning the repository

## Installation Methods

### Method 1: Pre-built Binaries (Recommended)

#### Linux/macOS
```bash
# Download latest release
curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
sudo mv code-guardian /usr/local/bin/

# Verify installation
code-guardian --version
```

#### Windows
```powershell
# Download latest release
Invoke-WebRequest -Uri "https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-windows-x64.zip" -OutFile "code-guardian.zip"
Expand-Archive code-guardian.zip .
Move-Item code-guardian.exe $env:USERPROFILE\bin\

# Add to PATH and verify
code-guardian --version
```

### Method 2: Cargo Install

```bash
# Install from crates.io
cargo install code-guardian-cli

# Or install from git for latest development version
cargo install --git https://github.com/d-oit/code-guardian.git
```

### Method 3: Build from Source

```bash
# Clone repository
git clone https://github.com/d-oit/code-guardian.git
cd code-guardian

# Build release version
cargo build --release

# Install locally
cargo install --path crates/cli

# Verify installation
code-guardian --version
```

## Docker Installation

### Using Pre-built Image
```bash
# Pull official image
docker pull d-oit/code-guardian:latest

# Run container
docker run --rm -v $(pwd):/workspace d-oit/code-guardian scan /workspace
```

### Building Custom Docker Image
```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin code-guardian-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/code-guardian-cli /usr/local/bin/code-guardian
ENTRYPOINT ["code-guardian"]
```

## CI/CD Integration

### GitHub Actions
```yaml
- name: Install Code-Guardian
  run: |
    curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
    sudo mv code-guardian /usr/local/bin/

- name: Run Security Scan
  run: code-guardian scan . --format json --output security-report.json
```

### GitLab CI
```yaml
code_guardian_scan:
  image: d-oit/code-guardian:latest
  script:
    - code-guardian scan . --fail-on-critical --format junit --output report.xml
  artifacts:
    reports:
      junit: report.xml
```

### Jenkins Pipeline
```groovy
pipeline {
    agent any
    stages {
        stage('Security Scan') {
            steps {
                sh '''
                    curl -L https://github.com/d-oit/code-guardian/releases/latest/download/code-guardian-linux-x64.tar.gz | tar xz
                    ./code-guardian scan . --format json --output security-report.json
                '''
                archiveArtifacts artifacts: 'security-report.json'
            }
        }
    }
}
```

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores
- **Memory**: 2GB RAM
- **Storage**: 100MB free space
- **OS**: Linux, macOS, Windows 10+

### Recommended Requirements
- **CPU**: 4+ cores
- **Memory**: 4GB+ RAM
- **Storage**: 500MB free space
- **OS**: Linux/macOS (better performance)

## Post-Installation Setup

### 1. Verify Installation
```bash
code-guardian --version
code-guardian --help
```

### 2. Run Test Scan
```bash
# Create test file
echo 'TODO: fix this
password = "secret123"
console.log("debug")' > test.js

# Run scan
code-guardian scan test.js

# Clean up
rm test.js
```

### 3. Configure Shell Completion (Optional)
```bash
# Bash
echo 'source <(code-guardian completions bash)' >> ~/.bashrc

# Zsh
echo 'source <(code-guardian completions zsh)' >> ~/.zshrc

# Fish
code-guardian completions fish > ~/.config/fish/completions/code-guardian.fish
```

## Troubleshooting

### Common Issues

#### "command not found"
- Ensure installation directory is in PATH
- Try `which code-guardian` to check location
- On Linux/macOS: `export PATH=$PATH:/usr/local/bin`

#### "Permission denied"
- On Linux/macOS: `chmod +x /usr/local/bin/code-guardian`
- Or reinstall with proper permissions

#### "Library not found" (macOS)
```bash
# Install required dependencies
brew install openssl
```

#### Slow Performance
- Check available memory: `free -h` (Linux) or `system_profiler SPHardwareDataType` (macOS)
- Reduce thread count: `code-guardian scan . --max-threads 2`
- Use incremental scanning: `code-guardian scan . --incremental`

## Updating

### Automatic Updates
```bash
# Using cargo (if installed via cargo)
cargo install code-guardian-cli --force

# Or use the update script
curl -fsSL https://raw.githubusercontent.com/d-oit/code-guardian/main/scripts/update.sh | bash
```

### Manual Updates
1. Download latest release from GitHub
2. Replace existing binary
3. Verify with `code-guardian --version`

## Uninstalling

### Cargo Install
```bash
cargo uninstall code-guardian-cli
```

### Manual Install
```bash
# Remove binary
sudo rm /usr/local/bin/code-guardian

# Remove completions (if installed)
rm ~/.config/fish/completions/code-guardian.fish  # Fish
# Edit ~/.bashrc or ~/.zshrc to remove completion source lines
```

### Docker
```bash
# Remove image
docker rmi d-oit/code-guardian:latest

# Remove containers
docker rm $(docker ps -a -q --filter ancestor=d-oit/code-guardian)
```