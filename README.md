# `no_drop`

[![Build](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/no_drop/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/no_drop/no_drop/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/no_drop/status.svg)](https://deps.rs/repo/github/MaxMahem/no_drop)
[![codecov](https://codecov.io/github/MaxMahem/no_drop/graph/badge.svg?token=VqGfOfh0vp)](https://codecov.io/github/MaxMahem/no_drop)
![GitHub License](https://img.shields.io/github/license/MaxMahem/no_drop)

A selection of guard types that guard against values being automatically dropped, ensuring a value is explicitly consumed.

## Features

### `NoDrop` and `NoDropMsg`

Wraps a value in a guard type to ensure it is explicitly consumed before the guard is dropped.

- **Debug-Only Checks**: Use the `dbg` module for zero-cost release builds with panic checks only in debug mode
- **Always-Checked Mode**: Use the `rls` module for panic checks in all build configurations
- **Custom Messages**: Use the `NoDropMsg` variant to provide custom panic messages

### `DropGuard` and `DropGuardMsg`

A mutable drop guard that can be dynamically armed and disarmed. 

- **Debug-Only Checks**: Use the `dbg` module for zero-cost release builds with panic checks only in debug mode. Nearly zero-cost in release builds (one `bool`).
- **Always-Checked Mode**: Use the `rls` module for panic checks in all build configurations
- **Custom Messages**: Use the `DropGuardMsg` variant to provide custom panic messages

## Usage - `NoDrop` and `NoDropMsg`

### Debug-Only Protection (`dbg` module)

The `dbg` module provides panic protection in debug builds while being a zero-cost wrapper in release builds:

```rust
use no_drop::dbg::NoDrop;

let value = NoDrop::wrap(42);

// Extract the value safely
let inner = value.unwrap();
assert_eq!(inner, 42);

// This would panic in debug builds:
// let value = NoDrop::wrap(42);
// drop(value); // panic: "Value was dropped without being unwrapped"
```

Or use the convenient `.no_drop()` method via the `IntoNoDrop` trait:

```rust
use no_drop::dbg::IntoNoDrop;

let value = 42.no_drop();  // Wraps the value automatically
let inner = value.unwrap();
assert_eq!(inner, 42);
```

### Always-Panicking Protection (`rls` module)

The `rls` module provides panic protection in both debug and release builds:

```rust
use no_drop::rls::NoDrop;

let value = NoDrop::wrap("important data");

// Must consume the value
let inner = value.unwrap();
assert_eq!(inner, "important data");

// This would panic in ALL builds:
// let value = NoDrop::wrap("data");
// drop(value); // panic: "Value was dropped without being unwrapped"
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
assert_eq!(42, value.unwrap());
```

### Using as a Drop Guard

`NoDrop` and `NoDropMsg` supports using a unit type `()` instances, allowing you to use them as drop guards within another type, to ensure a specific method is called before the type is dropped. This can be useful to enforce a manual RAII pattern or to enforce a builder pattern.

```rust
use no_drop::dbg::NoDrop;

struct Transaction {
    guard: NoDrop,
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

## Usage - `DropGuard` and `DropGuardMsg`

Unlike `NoDrop` types which are consumed when unwrapped, **drop guards** can be dynamically armed and disarmed. This makes them ideal for protecting mutable state or critical sections that may be entered and exited multiple times.

### `DropGuard` - Custom Messages

`DropGuard` (an alias for `DropGuardMsg`) provides custom panic messages and can be toggled between armed and disarmed states:

```rust
use no_drop::dbg::DropGuard;

let mut guard = DropGuard::new_armed("critical section not exited properly");

// Check state
assert!(guard.armed());

// do work...

// Safely exit critical section
guard.disarm();

// Can rearm if needed
guard.arm();
guard.disarm(); // Message is retained across arm/disarm cycles
```

### `DropGuardEmpty` - No Messages

For cases where you don't need a custom panic message, use `DropGuardEmpty` which provides the same arm/disarm functionality with a default panic message

### Debug vs Release Variants

Both `DropGuard` and `DropGuardEmpty` have debug-only and always-panicking variants:

- **`dbg` module**: Nearly zero-cost in release builds (one `bool`), panics only in debug mode
- **`rls` module**: Always panics in both debug and release builds

```rust
// Debug-only guard (zero-cost in release)
use no_drop::dbg::DropGuard;
let mut guard = DropGuard::new_disarmed("debug only");

// Always-panicking guard (checks in all builds)
use no_drop::rls::DropGuard as DropGuardRls;
let mut guard = DropGuardRls::new_disarmed("always checked");
```

