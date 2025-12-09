# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-12-08

### Added
- Initial release of `no_drop` crate
- `NoDrop<T>` wrapper type that panics if dropped without being consumed
- `NoDropPassthrough<T>` zero-cost wrapper for release builds
- `Consume` trait with `new()` and `consume()` methods for safe value extraction
- `dbg` module: panic protection in debug builds, zero-cost in release builds
- `rls` module: panic protection in all build configurations
- Comprehensive test coverage for both debug and release modes
- CI/CD pipeline with build, test, and documentation workflows
- Code coverage reporting via codecov

[0.1.0]: https://github.com/MaxMahem/no_drop/releases/tag/v0.1.0
