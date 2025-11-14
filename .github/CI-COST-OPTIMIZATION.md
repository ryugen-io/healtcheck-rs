# CI/CD Cost Optimization

This document explains the CI/CD configuration optimizations to reduce costs.

## Current Configuration

### Active Workflows:
- ✅ **ci.yml** - Full CI pipeline (format, clippy, build, test, audit, deny)
  - Runs on: PR pushes + master/main pushes
  - Can be skipped with `.skip-ci` file
- ✅ **docs-check.yml** - Documentation validation (only on .md file changes)
  - Runs on: PR pushes + master/main pushes
  - Can be skipped with `.skip-ci` file
- ✅ **release.yml** - Release automation (only on tags)

### Disabled Workflows:
- ❌ **claude-code-review.yml** - Renamed to `.yml.disabled`

### Event-Triggered Workflows (no cost impact):
- ✅ **claude.yml** - Only runs when @claude is mentioned in issues/comments
- ✅ **release.yml** - Only runs on tag pushes

## Cost Control Strategy

**Default Behavior:**
- CI runs on every PR push and master/main push
- Provides full automated validation

**Cost Savings:**
- Create `.skip-ci` file when you want to skip ALL CI runs
- Perfect for experimental work, rapid iteration, or cost control
- Remove `.skip-ci` when you need CI validation again

## Skipping CI on Demand

You can temporarily disable ALL CI runs using a `.skip-ci` file:

### To Skip CI:
```bash
touch .skip-ci
git add .skip-ci
git commit -m "chore: skip CI temporarily"
git push
```

### To Re-enable CI:
```bash
rm .skip-ci
git add .skip-ci
git commit -m "chore: re-enable CI"
git push
```

**When to use:**
- During experimental work or refactoring
- When making multiple rapid commits
- When you need to reduce costs temporarily
- During documentation-only changes

**Important:** The `.skip-ci` file affects ALL branches, so remove it before merging important changes.

## Usage Examples

### Example 1: Normal Development (CI enabled)
```bash
# Work on feature branch
git checkout -b feature/new-thing

# Make changes, CI runs on PRs automatically
git add .
git commit -m "feat: add new thing"
git push

# PR gets full CI validation
# Merge when CI passes
```

### Example 2: Rapid Iteration (CI disabled)
```bash
# Skip CI for experimental work
touch .skip-ci
git add .skip-ci
git commit -m "chore: skip CI for experimental work"
git push

# Make rapid changes without CI overhead
git add .
git commit -m "wip: trying different approach"
git push  # No CI runs

# Re-enable CI when ready
rm .skip-ci
git add .skip-ci
git commit -m "chore: re-enable CI"
git push  # CI runs again
```

### Example 3: Documentation Changes (CI disabled)
```bash
# Skip CI for doc-only changes
touch .skip-ci
git add .skip-ci
git commit -m "docs: update README [skip ci]"
git push

# Remove .skip-ci when done with docs
rm .skip-ci
git add .skip-ci
git commit -m "chore: re-enable CI"
git push
```

## Recommended Workflow

1. **Normal PRs**: Let CI run automatically for validation
2. **Experimental work**: Create `.skip-ci` to save costs during iteration
3. **Before merge**: Remove `.skip-ci` to validate final state
4. **After merge**: CI runs on master/main automatically

## Benefits

**Flexibility:**
- Full CI validation when you need it
- Cost control when you don't
- Fine-grained control per commit

**Cost Savings:**
- Skip CI during experimental phases
- Avoid wasting CI minutes on WIP commits
- Still get full validation when ready

**Best Practices:**
- Always remove `.skip-ci` before merging important changes
- Use `.skip-ci` liberally during development
- Let CI run on final commits before merge
