# `no_drop`

[![Build](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/no_drop/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/no_drop/no_drop/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/no_drop/status.svg)](https://deps.rs/repo/github/MaxMahem/no_drop)
[![codecov](https://codecov.io/github/MaxMahem/no_drop/graph/badge.svg?token=VqGfOfh0vp)](https://codecov.io/github/MaxMahem/no_drop)
![GitHub License](https://img.shields.io/github/license/MaxMahem/no_drop)

A simple wrapper type that guards against values being automatically dropped, ensuring a value is explicitly consumed.

The primary use case is for double-checking logic during development, ensuring values that are desired to be preserved/moved from are not accidentally dropped.

## Features

- **Explicit Consumption**: Values wrapped in `NoDrop` must be explicitly consumed via the `Consume` trait, preventing accidental drops
- **Debug-Only Checks**: Use the `dbg` module for zero-cost release builds with panic checks only in debug mode
- **Always-Checked Mode**: Use the `rls` module for panic checks in all build configurations

## Usage

### Debug-Only Protection (`dbg` module)

The `dbg` module provides panic protection in debug builds while being a zero-cost wrapper in release builds:

```rust
use no_drop::dbg::{NoDrop, Consume};

let value = NoDrop::new(42);

// Extract the value safely
let inner = value.consume();
assert_eq!(inner, 42);

// This would panic in debug builds:
// let value = NoDrop::new(42);
// drop(value); // panic: "Value was dropped without being consumed"
```

Or use the convenient `.no_drop()` method via the `IntoNoDrop` trait:

```rust
use no_drop::dbg::{Consume, IntoNoDrop};

let value = 42.no_drop();  // Wraps the value automatically
let inner = value.consume();
assert_eq!(inner, 42);
```

### Always-Panicking Protection (`rls` module)

The `rls` module provides panic protection in both debug and release builds:

```rust
use no_drop::rls::{NoDrop, Consume};

let value = NoDrop::new("important data");

// Must consume the value
let inner = value.consume();
assert_eq!(inner, "important data");

// This would panic in ALL builds:
// let value = NoDrop::new("data");
// drop(value); // panic: "Value was dropped without being consumed"
```
