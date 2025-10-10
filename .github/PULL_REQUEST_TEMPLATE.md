# Pull Request

## 📋 Description

<!-- Provide a clear and concise description of what this PR accomplishes -->

### 🎯 Type of Change

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New feature (non-breaking change which adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📚 Documentation update
- [ ] 🎨 Code style/formatting
- [ ] ♻️ Refactoring (no functional changes)
- [ ] ⚡ Performance improvement
- [ ] 🧪 Test improvements
- [ ] 🔧 Build/CI changes
- [ ] 🔒 Security improvement

## 🔗 Related Issues

<!-- Link to related issues using keywords like "Fixes #123" or "Closes #456" -->

- Fixes #
- Related to #

## 🧪 Testing

### Test Strategy
<!-- Describe how you tested your changes -->

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] Existing tests pass

### Test Coverage
<!-- If applicable, mention coverage changes -->

- Coverage maintained/improved: _%
- New code covered by tests: _%

### Manual Testing Steps
<!-- Provide steps for manual testing -->

1. 
2. 
3. 

## 📸 Screenshots/Examples

<!-- If applicable, add screenshots or code examples -->

### Before
```bash
# Example of previous behavior
```

### After
```bash
# Example of new behavior
```

## 🚀 Performance Impact

<!-- Describe any performance implications -->

- [ ] No performance impact
- [ ] Performance improvement (describe below)
- [ ] Potential performance regression (justify below)

**Details:**

## 🔄 Migration Guide

<!-- If this is a breaking change, provide migration instructions -->

**For users upgrading from previous versions:**

1. 
2. 
3. 

## ✅ Checklist

### Code Quality
- [ ] Code follows the project's style guidelines (`cargo fmt`)
- [ ] Self-review of code completed
- [ ] Code is self-documenting with clear variable/function names
- [ ] Complex logic is commented
- [ ] No debugging code left in (console.log, println!, etc.)

### Testing
- [ ] Tests added for new functionality
- [ ] All tests pass locally (`cargo test`)
- [ ] Tests cover edge cases and error conditions
- [ ] Integration tests updated if needed

### Documentation
- [ ] Documentation updated (if needed)
- [ ] API documentation updated (if applicable)
- [ ] Examples updated (if applicable)
- [ ] CHANGELOG.md updated

### Security & Best Practices
- [ ] No sensitive information in code/commits
- [ ] Error handling implemented appropriately
- [ ] Input validation added where needed
- [ ] Security implications considered

### CI/CD
- [ ] All CI checks pass
- [ ] Build succeeds on all platforms
- [ ] Clippy warnings addressed
- [ ] Security audit passes (`cargo audit`)

### Dependencies
- [ ] No unnecessary dependencies added
- [ ] Dependencies are well-maintained and secure
- [ ] Cargo.toml updated appropriately

## 🤔 Questions/Concerns

<!-- Any questions or concerns about the implementation -->

## 👥 Reviewers

<!-- Tag specific reviewers if needed -->

/cc @reviewer1 @reviewer2

---

**Conventional Commit Format:**
- feat: new feature
- fix: bug fix  
- docs: documentation
- style: formatting
- refactor: code refactoring
- perf: performance improvement
- test: testing
- chore: maintenance

**Agent Coordination:**
If this PR involves multiple components, consider using:
- `@goap-planner` for complex feature coordination
- `@hive-mind-orchestrator` for multi-agent workflows
- `@atomic-commit-creator` for commit organization