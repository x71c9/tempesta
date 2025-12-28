# Contributing to Tempesta

## Development Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes with conventional commit messages
4. Push and create a Pull Request

## Pull Request Requirements

### PR Title Format
PR titles must follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add bookmark synchronization
fix: resolve crash on empty bookmarks
docs: update installation guide
feat!: redesign bookmark storage
fix!: change API signature
```

**Commit types and version bumps:**
- `feat!:` or `fix!:` - Breaking change indicated by exclamation mark (triggers major version bump)
- `feat:` - New feature (triggers minor version bump)
- `fix:`, `perf:`, `security:` - Bug fix or improvement (triggers patch version bump)  
- `docs:`, `style:`, `refactor:`, `test:`, `chore:` - Documentation and maintenance (triggers patch version bump)

**IMPORTANT:** The exclamation mark (!) after the type indicates a breaking change and will trigger a major version release.

### Code Quality Requirements

All PRs must pass automated checks:
- Tests: `cargo test`
- Formatting: `cargo fmt --check`
- Linting: `cargo clippy -- -D warnings`

### Branch Configuration

- Target branch: `master`
- Merge strategy: Squash merging (PR title becomes commit message)
- Status checks must pass before merge approval

## Development Environment

```bash
# Clone repository
git clone https://github.com/YOUR_USERNAME/tempesta.git
cd tempesta

# Verify setup
cargo test
cargo fmt --check
cargo clippy
cargo build --release
```

## Automated Release Process

Upon merge to master:
1. System analyzes conventional commit message from squashed merge
2. Determines semantic version increment based on commit type
3. Executes `cargo release` with appropriate version bump
4. Creates git tag triggering release pipeline
5. Updates package distribution channels

## Issue Reporting

- Search existing issues before creating new reports
- Provide system information and reproduction steps
- Include relevant code samples or error messages
