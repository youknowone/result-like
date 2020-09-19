/// OptionLike macro

pub trait OptionLike<T> {}

#[macro_export]
macro_rules! option_like {
    (pub $(($pub:ident))? $Option:ident, $Some:ident, $None:ident) => {
        #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, is_macro::Is)]
        pub $(($pub))?  enum $Option<T> {
            $None,
            $Some(T),
        }

        result_like::impl_option_like!($Option, $Some, $None);
    };
    ( $Option:ident, $Some:ident, $None:ident) => {
        #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, is_macro::Is)]
         enum $Option<T> {
            $None,
            $Some(T),
        }

        result_like::impl_option_like!($Option, $Some, $None);
    };
    (pub $(($pub:ident))? $Option:ident) => {
        result_like::option_like!(pub $(($pub))? $Option, Some, None);
    };
    ($Option:ident) => {
        result_like::option_like!($Option, Some, None);
    };
}

#[macro_export]
macro_rules! impl_option_like {
    ($Option:ident, $Some:ident, $None:ident) => {
        impl<T> result_like::OptionLike<T> for $Option<T> {}

        impl<T> $Option<T> {
            #[inline]
            pub fn from_option(option: Option<T>) -> Self {
                match option {
                    Some(v) => $Option::$Some(v),
                    None => $Option::$None,
                }
            }

            #[inline]
            pub fn into_option(self) -> Option<T> {
                match self {
                    $Option::$Some(v) => Some(v),
                    $Option::$None => None,
                }
            }

            #[inline]
            pub fn as_option(&self) -> Option<&T> {
                match self {
                    $Option::$Some(ref v) => Some(v),
                    $Option::$None => None,
                }
            }

            #[inline]
            pub fn as_option_mut(&mut self) -> Option<&mut T> {
                match self {
                    $Option::$Some(ref mut v) => Some(v),
                    $Option::$None => None,
                }
            }

            #[inline]
            pub fn as_ref(&self) -> $Option<&T> {
                match *self {
                    $Option::$Some(ref x) => $Option::$Some(x),
                    $Option::$None => $Option::$None,
                }
            }

            #[inline]
            pub fn as_mut(&mut self) -> $Option<&mut T> {
                match *self {
                    $Option::$Some(ref mut x) => $Option::$Some(x),
                    $Option::$None => $Option::$None,
                }
            }

            // as_pin_ref
            // as_pin_mut

            #[inline]
            pub fn expect(self, msg: &str) -> T where {
                self.into_option().expect(msg)
            }

            #[inline]
            pub fn unwrap(self) -> T {
                self.into_option().unwrap()
            }

            #[inline]
            pub fn unwrap_or(self, default: T) -> T {
                self.into_option().unwrap_or(default)
            }

            #[inline]
            pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
                self.into_option().unwrap_or_else(f)
            }

            #[inline]
            pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> $Option<U> {
                match self {
                    $Option::$Some(x) => $Option::$Some(f(x)),
                    $Option::$None => $Option::$None,
                }
            }

            #[inline]
            pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
                self.into_option().map_or(default, f)
            }

            #[inline]
            pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(
                self,
                default: D,
                f: F,
            ) -> U {
                self.into_option().map_or_else(default, f)
            }

            #[inline]
            pub fn ok_or<E>(self, err: E) -> Result<T, E> {
                self.into_option().ok_or(err)
            }

            #[inline]
            pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
                self.into_option().ok_or_else(err)
            }

            // iter
            // iter_mut

            #[inline]
            pub fn and<U>(self, optb: $Option<U>) -> $Option<U> {
                match self {
                    $Option::$Some(_) => optb,
                    $Option::$None => $Option::$None,
                }
            }

            #[inline]
            pub fn and_then<U, F: FnOnce(T) -> $Option<U>>(self, f: F) -> $Option<U> {
                match self {
                    $Option::$Some(x) => f(x),
                    $Option::$None => $Option::$None,
                }
            }

            #[inline]
            pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
                if let $Option::$Some(x) = self {
                    if predicate(&x) {
                        return $Option::$Some(x);
                    }
                }
                $Option::$None
            }

            #[inline]
            pub fn or(self, optb: $Option<T>) -> $Option<T> {
                match self {
                    $Option::$Some(_) => self,
                    $Option::$None => optb,
                }
            }

            #[inline]
            pub fn or_else<F: FnOnce() -> $Option<T>>(self, f: F) -> $Option<T> {
                match self {
                    $Option::$Some(_) => self,
                    $Option::$None => f(),
                }
            }

            #[inline]
            pub fn xor(self, optb: $Option<T>) -> $Option<T> {
                match (self, optb) {
                    ($Option::$Some(a), $Option::$None) => $Option::$Some(a),
                    ($Option::$None, $Option::$Some(b)) => $Option::$Some(b),
                    _ => $Option::$None,
                }
            }

            #[inline]
            pub fn get_or_insert(&mut self, v: T) -> &mut T {
                self.get_or_insert_with(|| v)
            }

            #[inline]
            pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
                if let $Option::$None = *self {
                    *self = $Option::$Some(f());
                }

                match *self {
                    $Option::$Some(ref mut v) => v,
                    $Option::$None => unsafe { std::hint::unreachable_unchecked() },
                }
            }

            #[inline]
            pub fn take(&mut self) -> Self {
                std::mem::take(self)
            }

            #[inline]
            pub fn replace(&mut self, value: T) -> Self {
                std::mem::replace(self, $Option::$Some(value))
            }

            pub fn zip<U>(self, other: $Option<U>) -> $Option<(T, U)> {
                $Option::from_option(self.into_option().zip(other.into_option()))
            }

            // pub fn zip_with<U, F, R>(self, other: $Option<U>, f: F) -> $Option<R>
            // where
            //     F: FnOnce(T, U) -> R,
            // {
            //     $Option::from_option(self.into_option().zip_with(other.into_option(), f))
            // }
        }

        impl<T: Copy> $Option<&T> {
            pub fn copied(self) -> $Option<T> {
                self.map(|&t| t)
            }
        }

        impl<T: Copy> $Option<&mut T> {
            pub fn copied(self) -> $Option<T> {
                self.map(|&mut t| t)
            }
        }

        impl<T: Clone> $Option<&T> {
            pub fn cloned(self) -> $Option<T> {
                self.map(|t| t.clone())
            }
        }

        impl<T: Clone> $Option<&mut T> {
            pub fn cloned(self) -> $Option<T> {
                self.map(|t| t.clone())
            }
        }

        // impl<T: fmt::Debug> $Option<T>
        // expect_none
        // unwrap_none

        impl<T: Default> $Option<T> {
            #[inline]
            pub fn unwrap_or_default(self) -> T {
                match self {
                    $Option::$Some(x) => x,
                    $Option::$None => Default::default(),
                }
            }
        }

        impl<T: std::ops::Deref> $Option<T> {
            pub fn as_deref(&self) -> $Option<&T::Target> {
                self.as_ref().map(|t| t.deref())
            }
        }

        impl<T: std::ops::DerefMut> $Option<T> {
            pub fn as_deref_mut(&mut self) -> $Option<&mut T::Target> {
                self.as_mut().map(|t| t.deref_mut())
            }
        }

        impl<T, E> $Option<Result<T, E>> {
            #[inline]
            pub fn transpose(self) -> Result<$Option<T>, E> {
                self.into_option()
                    .transpose()
                    .map(|op| $Option::from_option(op))
            }
        }
        // TODO: result-like

        impl<T: Clone> Clone for $Option<T> {
            #[inline]
            fn clone(&self) -> Self {
                match self {
                    $Option::$Some(x) => $Option::$Some(x.clone()),
                    $Option::$None => $Option::$None,
                }
            }

            #[inline]
            fn clone_from(&mut self, source: &Self) {
                match (self, source) {
                    ($Option::$Some(to), $Option::$Some(from)) => to.clone_from(from),
                    (to, from) => *to = from.clone(),
                }
            }
        }

        impl<T> Default for $Option<T> {
            #[inline]
            fn default() -> $Option<T> {
                $Option::$None
            }
        }

        impl<T> IntoIterator for $Option<T> {
            type Item = T;
            type IntoIter = std::option::IntoIter<T>;

            #[inline]
            fn into_iter(self) -> std::option::IntoIter<T> {
                self.into_option().into_iter()
            }
        }

        // impl<'a, T> IntoIterator for &'a $Option<T> {
        //     type Item = &'a T;
        //     type IntoIter = std::option::Iter<'a, T>;

        //     fn into_iter(self) -> std::option::Iter<'a, T> {
        //         self.iter()
        //     }
        // }

        // impl<'a, T> IntoIterator for &'a mut $Option<T> {
        //     type Item = &'a mut T;
        //     type IntoIter = std::option::IterMut<'a, T>;

        //     fn into_iter(self) -> std::option::IterMut<'a, T> {
        //         self.iter_mut()
        //     }
        // }

        impl<T> From<T> for $Option<T> {
            fn from(val: T) -> $Option<T> {
                $Option::$Some(val)
            }
        }

        impl<'a, T> From<&'a $Option<T>> for $Option<&'a T> {
            fn from(o: &'a $Option<T>) -> $Option<&'a T> {
                o.as_ref()
            }
        }

        impl<'a, T> From<&'a mut $Option<T>> for $Option<&'a mut T> {
            fn from(o: &'a mut $Option<T>) -> $Option<&'a mut T> {
                o.as_mut()
            }
        }

        // flatten
    };
    ($Option:ident) => {
        result_like::impl_option_like!($Option, Some, None);
    };
}
