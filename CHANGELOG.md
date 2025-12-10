# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` `derive` implementations for both `NoDrop` and `NoDropPassthrough` types.

## [0.1.2] - 2025-12-09

### Added
- `IntoNoDrop` extension trait providing `.no_drop()` method for wrapping values
- Trait is conditionally aliased: `IntoNoDropRls` in debug mode, `IntoNoDropDbg` in release mode for `dbg` module
- `IntoNoDropDbg` trait for zero-cost wrapper creation
- `IntoNoDropRls` trait for always-panicking wrapper creation

[0.1.2]: https://github.com/MaxMahem/no_drop/releases/tag/v0.1.2

## [0.1.1] - 2025-12-08

### Added
- `Deref`, `DerefMut`, `AsRef`, and `AsMut` trait implementations (via `derive_more`) for both `NoDrop` and `NoDropPassthrough` types for improved ergonomics
- `derive_more` dependency with `as_ref`, `deref`, and `deref_mut` features

[0.1.1]: https://github.com/MaxMahem/no_drop/releases/tag/v0.1.1

## [0.1.0] - 2025-12-08

### Added
- Initial release of `no_drop` crate
- `NoDrop<T>` wrapper type that panics if dropped without being consumed
- `NoDropPassthrough<T>` zero-cost wrapper for release builds
- `Consume` trait with `new()` and `consume()` methods for safe value extraction
- `dbg` module: panic protection in debug builds, zero-cost in release builds
- `rls` module: panic protection in all build configurations

[0.1.0]: https://github.com/MaxMahem/no_drop/releases/tag/v0.1.0
