# Security Policy

## Supported Versions

We take security seriously and provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | ✅ Yes             |
| 0.1.x   | ✅ Yes (until 0.3.0) |
| < 0.1.0 | ❌ No              |

## Reporting a Vulnerability

**⚠️ Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in Code-Guardian, please report it responsibly:

### 🔒 Private Reporting (Preferred)

**GitHub Security Advisories**: Use GitHub's private vulnerability reporting
   - Go to: https://github.com/d-oit/code-guardian/security/advisories/new
   - This allows for private discussion and coordinated disclosure

### 📝 What to Include

Please include as much information as possible:

- **Type of vulnerability** (e.g., code injection, path traversal, etc.)
- **Component affected** (CLI, Core, Storage, Output)
- **Steps to reproduce** the vulnerability
- **Potential impact** and attack scenarios
- **Suggested fix** (if you have one)
- **Your contact information** for follow-up

### 🚀 Response Timeline

We are committed to addressing security issues promptly:

- **Initial Response**: Within 48 hours of report
- **Assessment**: Within 1 week of initial response
- **Fix Development**: Depends on complexity, typically 1-2 weeks
- **Coordinated Disclosure**: After fix is ready and tested

### 🛡️ Security Measures in Code-Guardian

Code-Guardian implements several security measures:

#### Input Validation
- File path validation to prevent directory traversal
- File size limits to prevent resource exhaustion
- Extension filtering to avoid processing malicious files

#### Safe Processing
- Sandboxed pattern matching
- Memory limits for large file processing
- Timeout protection for long-running operations

#### Dependency Security
- Regular dependency audits with `cargo audit`
- Automated dependency updates via Dependabot
- Minimal dependency surface area

#### Build Security
- Reproducible builds
- Multi-platform testing
- Signed releases (planned)

### 🔍 Security-Related Features

Code-Guardian can help detect security issues in your code:

- **Unsafe Rust Detection**: Identifies `unsafe` blocks
- **Panic Detection**: Finds `panic!` and `unwrap()` calls
- **Debug Code Detection**: Locates debugging code that shouldn't be in production
- **Credential Pattern Detection**: Identifies potential hardcoded secrets (planned)

### 🏆 Responsible Disclosure

We believe in responsible disclosure and will:

- **Acknowledge** your contribution to improving security
- **Work with you** to understand and resolve the issue
- **Provide credit** in release notes (if desired)
- **Consider monetary rewards** for significant findings (case-by-case basis)

### 📋 Security Checklist for Contributors

When contributing to Code-Guardian:

- [ ] Run `cargo audit` to check for known vulnerabilities
- [ ] Use `cargo clippy` to catch potential security issues
- [ ] Validate all external inputs
- [ ] Avoid `unwrap()` and `panic!` in production code paths
- [ ] Consider attack vectors for new features
- [ ] Document security implications of changes

### 🔗 Security Resources

- [Rust Security Guidelines](https://doc.rust-lang.org/nomicon/)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [CVE Database](https://cve.mitre.org/)

### 📞 Contact

For non-security related issues, please use:
- **Bug Reports**: GitHub Issues
- **Feature Requests**: GitHub Discussions
- **General Questions**: GitHub Discussions

For security-related concerns:
- **GitHub Security Advisories**: https://github.com/d-oit/code-guardian/security/advisories/new

---

**Thank you for helping keep Code-Guardian and its users safe!** 🛡️