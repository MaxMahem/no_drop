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
- **Custom Messages**: Use the `NoDropMsg` variant to provide custom panic messages

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

### Custom Panic Messages (`NoDropMsg`)

For more descriptive error messages, use `NoDropMsg` with custom panic messages:

```rust,no_run
use no_drop::rls::NoDropMsg;
// msg can be an owned or borrowed value
let value = NoDropMsg::wrap(42, "forgot to process the answer");

drop(value); // panic: "forgot to process the answer"
```

To properly use the value:

```rust
use no_drop::rls::NoDropMsg;

let value = NoDropMsg::wrap(42, "forgot to process the answer");
assert_eq!(42, value.consume());
```

### Using as a Drop Guard

`NoDrop` supports a unit type `()` instances, allowing you to use them as drop guards within another type, to ensure a specific method is called before the type is dropped. This can be useful to enforce a manual RAII pattern or to enforce a builder pattern.

```rust
use no_drop::dbg::NoDrop;

struct Transaction {
    guard: NoDrop<()>,
    other_data: i32,
}

impl Transaction {
    fn new(x: i32) -> Self {
        Self { guard: NoDrop::new(), other_data: x }
    }

    fn finalize(self) {
        // do necessary finalization work
        self.guard.forget(); // Disarm the guard by consuming it
    }
}

let t = Transaction::new(10);
// do work.
t.finalize();
// Dropping without calling `finalize()` would panic.
```

For custom panic messages with drop guards, use `NoDropMsg::guard()`:

```rust
use no_drop::dbg::NoDropMsg;

struct Transaction {
    guard: NoDropMsg<'static, ()>,
    other_data: i32,
}

impl Transaction {
    fn new(x: i32) -> Self {
        Self { 
            guard: NoDropMsg::guard("Transaction was dropped without being finalized"), 
            other_data: x 
        }
    }

    fn finalize(self) {
        // do necessary finalization work
        self.guard.forget(); // Disarm the guard
    }
}

let t = Transaction::new(10);
t.finalize();
// Dropping without calling `finalize()` would panic with custom message.
```

### Mutable Drop Guards (`DropGuard` and `DropGuardMsg`)

For cases where you need to dynamically arm and disarm a guard, use `DropGuard` or `DropGuardMsg`:

```rust
use no_drop::dbg::DropGuardMsg;

let mut guard = DropGuardMsg::new_armed("critical section not exited properly");

// Check state
assert!(guard.armed());

// Safely exit critical section
guard.disarm();

// Can rearm if needed
guard.arm();
guard.disarm(); // Message is retained across arm/disarm cycles
```

