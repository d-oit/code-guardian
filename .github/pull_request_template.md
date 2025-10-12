## Pull Request Checklist

### 🔍 Quality Gates
Please ensure all quality gates pass before requesting review:

- [ ] **Lint**: Code passes `cargo fmt` and `cargo clippy` checks
- [ ] **Build**: All crates build successfully (`cargo build --workspace`)
- [ ] **Test**: All tests pass (`cargo test --workspace`)
- [ ] **Security**: No security vulnerabilities detected
- [ ] **Performance**: Performance impact assessed (if applicable)
- [ ] **Documentation**: Code is properly documented

### 📝 Description
<!-- Provide a brief description of the changes -->

### 🎯 Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Security enhancement

### 🧪 Testing
<!-- Describe the tests you ran and how to reproduce them -->

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

### 📚 Documentation
- [ ] Code comments added/updated
- [ ] API documentation updated
- [ ] User documentation updated (if applicable)
- [ ] CHANGELOG.md updated

### 🔒 Security Considerations
- [ ] No sensitive data exposed
- [ ] Security implications reviewed
- [ ] Dependencies are secure and up-to-date

### 📋 Additional Notes
<!-- Any additional information, breaking changes, or considerations -->

---

**By submitting this PR, I confirm that:**
- [ ] I have read and followed the contributing guidelines
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my own code
- [ ] All quality gates are passing