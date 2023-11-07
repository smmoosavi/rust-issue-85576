# rust lang issue 85576

this is a show case for rust lang issue [85576]

## when it happens

When type is a associated type of a trait, and the trait is implemented in another crate. if the trait is implemented in the same crate, it works fine.

```rust
type OtherDuration = <other::Systick as other::Monotonic>::Duration;
type OtherInstant = <other::Systick as other::Monotonic>::Instant;

impl FromTime for OtherDuration {}
impl FromTime for OtherInstant {}

```

if types used directly, it works fine.

```rust
    impl FromTime for other::Duration {}

    impl FromTime for other::Instant {}
```

- `other::Duration` and `OtherDuration` are the same type.
- `other::Instant` and `OtherInstant` are the same type.
- `OtherDuration` and `OtherInstant` aren't the same type.

```rust
assert_eq!(TypeId::of::<OtherDuration>(),TypeId::of::<other::Duration>());
assert_eq!(TypeId::of::<OtherInstant>(), TypeId::of::<other::Instant>());

assert_ne!(TypeId::of::<OtherDuration>(), TypeId::of::<OtherInstant>());
```

## how to reproduce

```bash
# ok
cargo clippy --features crate-direct
# ok
cargo clippy --features crate-systick
# ok
cargo clippy --features other-direct

# error
cargo clippy --features other-systick
```

error message:

```text
error[E0119]: conflicting implementations of trait `FromTime`
  --> src/lib.rs:48:5
   |
46 |     impl FromTime for OtherDuration {}
   |     ------------------------------- first implementation here
47 |
48 |     impl FromTime for OtherInstant {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation

For more information about this error, try `rustc --explain E0119`.
```

[85576]: https://github.com/rust-lang/rust/issues/85576
