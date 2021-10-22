//! # OptionLike and ResultLike
//!
//! Define your own Option-like and Result-like enum types.
//! Avoid to reimplement everything of [std::option::Option] and [std::result::Result] for your own enums.
//!
//! Option example
//! ```rust
//! use result_like::OptionLike;
//!
//! // Simple case with single argument name to use Some and None
//! #[derive(OptionLike)]
//! enum MyOption {
//!     Some(u32),
//!     None
//! }
//!
//! let v = MyOption::Some(1);
//! // every option utilities are possible including unwrap, map, and, or etc.
//! assert_eq!(v.unwrap(), 1);
//!
//! // convertable to option
//! let opt = v.into_option();
//! assert_eq!(opt, Some(1));
//! ```
//!
//! Result example in same way
//! ```rust
//! use result_like::ResultLike;
//!
//! #[derive(ResultLike)]
//! enum Trial<T, E> {
//!     Success(T),
//!     Failure(E),
//! }
//! ```

extern crate result_like_derive;

pub use result_like_derive::*;

// mod option;
// pub use option::*;

// mod result;
// pub use result::*;
