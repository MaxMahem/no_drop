# `no_drop`

[![Build](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/no_drop/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/no_drop/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/no_drop/no_drop/index.html)
[![Crates.io](https://img.shields.io/crates/v/no_drop)](https://crates.io/crates/no_drop)
[![dependency status](https://deps.rs/repo/github/MaxMahem/no_drop/status.svg)](https://deps.rs/repo/github/MaxMahem/no_drop)
[![codecov](https://codecov.io/github/MaxMahem/no_drop/graph/badge.svg?token=VqGfOfh0vp)](https://codecov.io/github/MaxMahem/no_drop)
![GitHub License](https://img.shields.io/github/license/MaxMahem/no_drop)

A selection of guard types that guard against values being automatically dropped, ensuring a value is explicitly consumed.

## Features

* [`NoDrop`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDrop.html) and [`NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html) - Wraps a value in a guard type to ensure it is explicitly consumed before the guard is dropped.
* [`DropGuard`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuard.html) and [`DropGuardMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuardMsg.html) - A mutable drop guard that can be dynamically armed and disarmed.
* [`dbg`](https://MaxMahem.github.io/no_drop/no_drop/dbg/index.html) and [`rls`](https://MaxMahem.github.io/no_drop/no_drop/rls/index.html) modules - identical API, provide panic protection in debug mode only or in all builds respectively.

## Install

It's on [crates.io](https://crates.io/crates/no_drop).

## Usage - [`NoDrop`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDrop.html) and [`NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html)

Wrap a value in a guard type to ensure it is explicitly consumed before the guard is dropped.

```rust,no_run
use no_drop::rls::NoDrop;

let value = NoDrop::wrap(42);

// Extract the value safely
let inner = value.unwrap();
assert_eq!(inner, 42);

let value = NoDrop::wrap(43);
drop(value); // panic: "Value was dropped without being unwrapped"
```

For more descriptive error messages, use [`NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html) with custom panic messages:

```rust,no_run
use no_drop::rls::NoDropMsg;
// msg can be an owned or borrowed value
let value = NoDropMsg::wrap(42, "forgot to process the answer");

drop(value); // panic: "forgot to process the answer"
```

### Using as a Drop Guard

[`NoDrop`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDrop.html) and [`NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html) support using unit type `()` instances, allowing you to use them as drop guards within another type, to ensure a specific method is called before the type is dropped. This can be useful to enforce a manual RAII pattern or to enforce a builder pattern.

```rust
use no_drop::rls::NoDrop;

#[derive(Debug, Default)] // NoDrop implements Default and Debug
struct Transaction {
    guard: NoDrop,
    other_data: i32,
}

impl Transaction {
    fn new(x: i32) -> Self {
        Self { guard: NoDrop::guard(), other_data: x }
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

For custom panic messages with drop guards, use [`NoDropMsg::guard`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html#method.guard)

## Usage - [`DropGuard`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuard.html) and [`DropGuardMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuardMsg.html)

Unlike `NoDrop` types, which are consumed when unwrapped, `DropGuard`s can be dynamically armed and disarmed. This makes them ideal for protecting mutable state or critical sections that may be entered and exited, possibly multiple times.

```rust
use no_drop::rls::DropGuard;

let mut guard = DropGuard::new_armed("critical section not exited properly");

// Check state
assert!(guard.is_armed());

// do work...

// Safely exit critical section
guard.disarm();

// Can rearm if needed
guard.arm();
guard.disarm(); // Message is retained across arm/disarm cycles
```

For cases where you don't need a custom panic message, use [`DropGuardEmpty`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuardEmpty.html), which provides the same arm/disarm functionality with a default panic message

## Debug vs Release Variants

`NoDrop`, `NoDropMsg`, `DropGuard`, `DropGuardEmpty` have debug-only and always-panicking variants:

* [`dbg::NoDrop`](https://docs.rs/no_drop/latest/no_drop/dbg/struct.NoDrop.html): Zero-cost in release builds, panics only in debug mode
* [`rls::NoDrop`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDrop.html): Always panics in both debug and release builds
* [`dbg::NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/dbg/struct.NoDropMsg.html): Zero-cost in release builds, panics only in debug mode
* [`rls::NoDropMsg`](https://docs.rs/no_drop/latest/no_drop/rls/struct.NoDropMsg.html): Always panics in both debug and release builds
* [`dbg::DropGuard`](https://docs.rs/no_drop/latest/no_drop/dbg/struct.DropGuard.html): Nearly zero-cost in release builds (one `bool`), panics only in debug mode
* [`rls::DropGuard`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuard.html): Always panics in both debug and release builds
* [`dbg::DropGuardEmpty`](https://docs.rs/no_drop/latest/no_drop/dbg/struct.DropGuardEmpty.html): Nearly zero-cost in release builds (one `bool`), panics only in debug mode
* [`rls::DropGuardEmpty`](https://docs.rs/no_drop/latest/no_drop/rls/struct.DropGuardEmpty.html): Always panics in both debug and release builds
