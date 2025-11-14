# CI/CD Cost Optimization

This document explains the CI/CD configuration optimizations to reduce costs.

## Current Configuration

### Workflows that RUN on every master/main push:
- ✅ **ci.yml** - Full CI pipeline (format, clippy, build, test, audit, deny)
- ✅ **docs-check.yml** - Documentation validation (only on .md file changes)
- ✅ **release.yml** - Release automation (only on tags)

### Workflows that are DISABLED:
- ❌ **Pull Request CI** - Commented out in ci.yml (lines 12-19)
- ❌ **Pull Request Docs Check** - Commented out in docs-check.yml (lines 8-11)
- ❌ **claude-code-review.yml** - Renamed to `.yml.disabled`

### Workflows that are EVENT-TRIGGERED (no cost impact):
- ✅ **claude.yml** - Only runs when @claude is mentioned in issues/comments
- ✅ **release.yml** - Only runs on tag pushes

## Cost Savings

**Before:**
- CI runs on every PR push (sync)
- CI runs on every master/main push
- Docs check on every PR with .md changes
- Claude code review on every PR

**After:**
- CI runs ONLY on master/main pushes (after merge)
- Docs check ONLY on master/main pushes
- No automatic PR checks (saves ~70% of CI runs)

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

## Re-enabling PR Checks

If you want to re-enable PR checks for specific PRs or periods:

### Option 1: Re-enable PR CI for all PRs
Uncomment lines 12-19 in `.github/workflows/ci.yml`:
```yaml
pull_request:
  branches: [ master, main ]
  paths-ignore:
    - '**.md'
    - 'LICENSE*'
    - '.gitignore'
    - '.github/workflows/release.yml'
    - 'AGENTS.md'
```

### Option 2: Re-enable Claude Code Review
Rename `.github/workflows/claude-code-review.yml.disabled` back to `.yml`

### Option 3: Manual PR checks
Before merging important PRs, run locally:
```bash
./rebuild.sh  # Runs fmt, clippy, auditable build
cargo test    # Runs all tests
```

## Recommended Workflow

1. **Development**: Work on feature branches, test locally with `./rebuild.sh`
2. **Pre-merge**: Run `cargo test` and `cargo clippy -- -D warnings` locally
3. **Merge**: CI runs automatically on master/main after merge
4. **Issues**: If CI fails on master, fix immediately (or revert)

## Trade-offs

**Pros:**
- Significant cost savings (50-70% reduction in CI minutes)
- Still get CI validation on all code that lands in master
- Local testing encourages better development practices

**Cons:**
- No automated PR validation before merge
- Broken PRs can land in master (though CI will catch them post-merge)
- Requires discipline to test locally before merging

**Mitigation:**
- Use branch protection rules requiring local checks
- Only merge PRs you've tested locally
- Enable PR CI temporarily for high-risk changes
- Use `git revert` if something breaks master
