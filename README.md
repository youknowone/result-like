# OptionLike and ResultLike

Install: [https://crates.io/crates/result-like](https://crates.io/crates/result-like)

Define your own Option-like and Result-like enum types.
Avoid to reimplement everything of option and result for your own enums.

Option example
```rust
use result_like::OptionLike;

// Simple case with single argument name to use Some and None
#[derive(OptionLike)]
enum MyOption<T> {
    Some(T),
    None,
}

let v = MyOption::Some(1);
// every option utilities are possible including unwrap, map, and, or etc.
assert_eq!(v.unwrap(), 1);

// convertable to option
let opt = v.into_option();
assert_eq!(opt, Some(1));

// enum with custom names instead of Some and None
#[derive(OptionLike)]
enum Number {
    Value(i64),
    Nan,
}

let v = Number::Value(10);
assert_ne!(v, Number::Nan);
```

Result example in same way
```rust
use result_like::ResultLike;

// typical
#[derive(ResultLike)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// value-only
#[derive(ResultLike)]
enum Trial {
    Success(String),
    Failure(String),
}
```
