# `no_drop`

[![Build](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/no_drop/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/no_drop/no_drop/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/no_drop/status.svg)](https://deps.rs/repo/github/MaxMahem/no_drop)
[![codecov](https://codecov.io/github/MaxMahem/no_drop/graph/badge.svg?token=VqGfOfh0vp)](https://codecov.io/github/MaxMahem/no_drop)
![GitHub License](https://img.shields.io/github/license/MaxMahem/no_drop)

A simple wrapper type that guards against values being automatically dropped, ensuring a value is explicitly consumed.

The primary use case is for double-checking logic during development, ensuring values that are desired to be preserved/moved from are not accidentally dropped.

## Features

- **Explicit Consumption**: Values wrapped in `NoDrop` must be explicitly consumed via the `consume()` method, preventing accidental drops
- **Debug-Only Checks**: Use the `dbg` module for zero-cost release builds with panic checks only in debug mode
- **Always-Checked Mode**: Use the `rls` module for panic checks in all build configurations

## Usage

### Debug-Only Protection (`dbg` module)

The `dbg` module provides panic protection in debug builds while being a zero-cost wrapper in release builds:

```rust
use no_drop::dbg::NoDrop;

let value = NoDrop::wrap(42);

// Extract the value safely
let inner = value.consume();
assert_eq!(inner, 42);

// This would panic in debug builds:
// let value = NoDrop::wrap(42);
// drop(value); // panic: "Value was dropped without being consumed"
```

Or use the convenient `.no_drop()` method via the `IntoNoDrop` trait:

```rust
use no_drop::dbg::IntoNoDrop;

let value = 42.no_drop();  // Wraps the value automatically
let inner = value.consume();
assert_eq!(inner, 42);
```

### Always-Panicking Protection (`rls` module)

The `rls` module provides panic protection in both debug and release builds:

```rust
use no_drop::rls::NoDrop;

let value = NoDrop::wrap("important data");

// Must consume the value
let inner = value.consume();
assert_eq!(inner, "important data");

// This would panic in ALL builds:
// let value = NoDrop::wrap("data");
// drop(value); // panic: "Value was dropped without being consumed"
```

### Using as a Drop Guard

`NoDrop` supports a unit type `()` instances, allowing you to use them as drop guards to ensure specific code paths are taken:

```rust
use no_drop::dbg::NoDrop;

fn important_operation() {
    let guard = NoDrop::new();  // or NoDrop::default()
    
    // Do important work here
    // If this function returns early without consuming the guard,
    // it will panic in debug builds
    
    guard.forget(); // Explicitly mark the operation as complete
}
```

This is useful for ensuring that cleanup code runs or that certain operations complete fully.
