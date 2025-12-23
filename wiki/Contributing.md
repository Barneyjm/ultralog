# Contributing

Thank you for your interest in contributing to UltraLog!

## Quick Start

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit (`git commit -m 'feat: add amazing feature'`)
6. Push (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Full Contributing Guide

For complete contributing guidelines, see [CONTRIBUTING.md](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md) in the repository.

The full guide covers:

- [Setting up your development environment](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#getting-started)
- [Development workflow](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#development-workflow)
- [Pull request process](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#pull-request-process)
- [Coding standards](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#coding-standards)
- [Commit message guidelines](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#commit-guidelines)
- [Testing requirements](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#testing)
- [Adding new ECU support](https://github.com/SomethingNew71/UltraLog/blob/main/CONTRIBUTING.md#adding-ecu-support)

## Types of Contributions

| Type | Description |
|------|-------------|
| Bug Fixes | Fix reported issues |
| New Features | Add new functionality |
| ECU Support | Add support for new ECU formats |
| Documentation | Improve README, wiki, or code comments |
| Tests | Add or improve test coverage |
| Performance | Optimize parsing or rendering |

## Finding Issues

- [`good first issue`](https://github.com/SomethingNew71/UltraLog/labels/good%20first%20issue) - Great for first-time contributors
- [`help wanted`](https://github.com/SomethingNew71/UltraLog/labels/help%20wanted) - Issues where we need help

## Before You Start

1. **Check existing issues** - Someone may already be working on it
2. **Open an issue first** - Discuss proposed changes before starting significant work
3. **Get feedback** - Wait for maintainer input on large changes

## Commit Message Format

```
type(scope): short description

Longer description if needed.

Fixes #123
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

## Pull Request Checklist

- [ ] Code compiles (`cargo build`)
- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation updated if needed

## Code of Conduct

Be respectful, constructive, and welcoming to all contributors.

---

## Next Steps

- [[Development]] - Development setup and architecture
- [[FAQ]] - Common questions
- [GitHub Issues](https://github.com/SomethingNew71/UltraLog/issues) - Report bugs or request features
