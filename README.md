# OptionLike and ResultLike

Install: [https://crates.io/crates/result-like](https://crates.io/crates/result-like)

Define your own Option-like and Result-like enum types.
Avoid to reimplement everything of option and result for your own enums.

Option example
```rust
use result_like::option_like;

// Simple case with single argument name to use Some and None
option_like!(MyOption);

let v = MyOption::Some(1);
// every option utilities are possible including unwrap, map, and, or etc.
assert!(v.unwrap() == 1);

// convertable to option
let opt = v.into_option();
assert!(opt == Some(1));

// pub enum with custom names instead of Some and None
option_like!(pub Number, Value, Nan);

let v = Number::Value(10);
assert!(v != Number::Nan);
```

Result example in same way
```rust
use result_like::result_like;

// simply,
result_like!(pub(crate) MyResult);

// customizing,
result_like!(Trial, Success, Failure);
```
