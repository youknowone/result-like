# OptionLike and ResultLike

Define your own Option-like and Result-like enum types.
Do not redefine option and result utilities to your own enums.

Option example
```rust
use result_like::OptionLike;

// Simple case with single argument name
OptionLike!(MyOption);

let v = MyOption::Some(1);
// every option utilities are possible including unwrap, map, and, or etc.
assert!(v.unwrap() == 1);

// convertable to option
let opt = v.into_option();
assert!(opt == Some(1));

// With custom names
OptionLike!(Number, Value, Nan);

let v = Number::Value(10);
assert!(v != Number::Nan);
```

Result example in same way
```rust
use result_like::ResultLike;

// simply,
ResultLike!(MyResult);

// customizing,
ResultLike!(Trial, Success, Failure);
```
