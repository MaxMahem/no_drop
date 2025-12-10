# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2025-12-10

### Changed

- Minor changes to rexports.

[0.2.2]: https://github.com/MaxMahem/no_drop/releases/tag/v0.2.2

## [0.2.1] - 2025-12-10

### Added

- Conversion Methods
  - `into_guard` for `DropGuard` types.
  - `From` `NoDrop` -> `DropGuard` implementations
  - `TryFrom` `DropGuard` -> `NoDrop` implementations
  - `GuardNotArmed` error type for `DropGuardPass` -> `NoDrop` conversions

- `DEFAULT_DROP_PANIC_MSG` constant for default panic messages

[0.2.1]: https://github.com/MaxMahem/no_drop/releases/tag/v0.2.1

## [0.2.0] - 2025-12-10

### Added

- `DropGuard<'msg>` type for mutable drop guards with custom panic messages
  - Supports dynamic arm/disarm operations via `arm()` and `disarm()` methods
  - Retains panic message across state changes
  - Available in both `dbg` and `rls` modules with conditional compilation support

- `DropGuardEmpty` type for mutable drop guard with a default panic message.
  - Same api as `DropGuard<'msg>` but with a default panic message.
  - Available in both `dbg` and `rls` modules with conditional compilation support

- `NoDrop::new()` for unit `NoDrop` values, useful for creating empty drop guards for uses as fields in other structs
- `NoDropMsg<'msg, T>` type with custom panic messages that supports both borrowed and owned messages via `Cow<'msg, str>`
- `IntoNoDrop::expect_no_drop(msg)` allows creating `NoDropMsg` values with custom panic messages, similar to `IntoNoDrop::no_drop`
- Added `Clone` implementation for `NoDrop`.

### Changed

- **Breaking**: Removed `Consume` trait entirely
  - Implemented `wrap()`, `consume()`, and `forget()` methods directly on `NoDrop` and `NoDropPassthrough` types

- **Breaking**: Removed `Consume` exports from `dbg` and `rls` modules

- Added `#[must_use]` attribute to `NoDrop`
- Added `#[inline]` attribute to `NoDrop::consume`

- **Breaking**: Renamed `consume()` method to `unwrap()` across all types (`NoDrop`, `NoDropMsg`, `NoDropPass`)
  - Updated panic messages from "Value was dropped without being consumed" to "Value was dropped without being unwrapped"
  - This aligns the API more closely with Rust conventions (similar to `Option::unwrap()` and `Result::unwrap()`)

### Migration Guide

Replace `Consume` trait usage with direct method calls:
- `Consume::new(value)` → `NoDrop::wrap(value)` or `NoDropPassthrough::wrap(value)`
- `value.consume()` remains the same
- `value.forget()` remains the same
- Remove `use no_drop::{dbg,rls}::Consume` imports

Replace `consume()` calls with `unwrap()`:
- `value.consume()` → `value.unwrap()`

[0.2.0]: https://github.com/MaxMahem/no_drop/releases/tag/v0.2.0

## [0.1.3] - 2025-12-09

### Added

- **Breaking**: Added `#[must_use]` attribute to `NoDrop` and `NoDropPassthrough` types
- Added `forget()` method to `NoDrop` and `NoDropPassthrough` types

### Changed

- Removed the `Size` bound from the `Consume` trait
- **Breaking**: Changed `Consume::new` to `Consume::wrap`

## [0.1.3] - 2025-12-09

### Added

- `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` `derive` implementations for both `NoDrop` and `NoDropPassthrough` types.
- Added `Consume::forget` method allowing a `NoDrop` value to be dropped without consuming it

[0.1.3]: https://github.com/MaxMahem/no_drop/releases/tag/v0.1.3

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
