# OptionLike and ResultLike

Define your own Option-like and Result-like enum types.
Do not redefine option and result utilities to your own enums.

Option example
```rust
use result_like::option_like;

// Simple case with single argument name
option_like!(MyOption);

let v = MyOption::Some(1);
// every option utilities are possible including unwrap, map, and, or etc.
assert!(v.unwrap() == 1);

// convertable to option
let opt = v.into_option();
assert!(opt == Some(1));

// pub enum with custom names
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
