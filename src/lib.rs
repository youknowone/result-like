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
//! #[derive(OptionLike, Clone, Copy)]
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

pub trait OptionLike
where
    Self: Sized,
{
    type SomeType;

    // fn from_value(value: Self::SomeType) -> Self;
    // fn from_option(option: Option<Self::SomeType>) -> Self;
    // fn into_option(self) -> Option<Self::SomeType>;
    // fn as_option(&self) -> Option<&Self::SomeType>;
    // fn as_option_mut(&mut self) -> Option<&mut Self::SomeType>;
    // fn get_or_insert_with<_Function: FnOnce() -> Self::SomeType>(
    //     &mut self,
    //     f: _Function,
    // ) -> &mut Self::SomeType;

    // #[inline]
    // fn expect(self, msg: &str) -> Self::SomeType where {
    //     self.into_option().expect(msg)
    // }

    // #[inline]
    // fn unwrap(self) -> Self::SomeType {
    //     self.into_option().unwrap()
    // }

    // #[inline]
    // fn unwrap_or(self, default: Self::SomeType) -> Self::SomeType {
    //     self.into_option().unwrap_or(default)
    // }

    // #[inline]
    // fn unwrap_or_else<_Function: FnOnce() -> Self::SomeType>(self, f: _Function) -> Self::SomeType {
    //     self.into_option().unwrap_or_else(f)
    // }

    // #[inline]
    // fn ok_or<_Error>(self, err: _Error) -> Result<Self::SomeType, _Error> {
    //     self.into_option().ok_or(err)
    // }

    // #[inline]
    // fn ok_or_else<_Error, _Function: FnOnce() -> _Error>(
    //     self,
    //     err: _Function,
    // ) -> Result<Self::SomeType, _Error> {
    //     self.into_option().ok_or_else(err)
    // }

    // #[inline]
    // fn filter<P: FnOnce(&Self::SomeType) -> bool>(self, predicate: P) -> Self {
    //     Self::from_option(self.into_option().filter(predicate))
    // }

    // #[inline]
    // fn or(self, optb: Self) -> Self {
    //     Self::from_option(self.into_option().or(optb.into_option()))
    // }

    // #[inline]
    // fn or_else<_Function: FnOnce() -> Self>(self, f: _Function) -> Self {
    //     Self::from_option(self.into_option().or_else(|| f().into_option()))
    // }

    // #[inline]
    // fn map_or<_Other, _Function: FnOnce(Self::SomeType) -> _Other>(
    //     self,
    //     default: _Other,
    //     f: _Function,
    // ) -> _Other {
    //     self.into_option().map_or(default, f)
    // }

    // #[inline]
    // fn xor(self, optb: Self) -> Self {
    //     Self::from_option(self.into_option().xor(optb.into_option()))
    // }

    // #[inline]
    // fn get_or_insert(&mut self, v: Self::SomeType) -> &mut Self::SomeType {
    //     self.get_or_insert_with(|| v)
    // }

    // #[inline]
    // fn take(&mut self) -> Self
    // where
    //     Self: Default,
    // {
    //     std::mem::take(self)
    // }

    // #[inline]
    // fn replace(&mut self, value: Self::SomeType) -> Self {
    //     std::mem::replace(self, Self::from_value(value))
    // }

    // #[inline]
    // fn unwrap_or_default(self) -> Self::SomeType
    // where
    //     Self::SomeType: Default,
    // {
    //     self.into_option().unwrap_or_default()
    // }
}

pub trait ResultLike {
    type OkType;
    type ErrType;
}
