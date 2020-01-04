/// ResultLike macro

pub trait ResultLike<T, E> {}

#[macro_export]
macro_rules! result_like {
    (pub $(($pub:ident))? $Result:ident, $Ok:ident, $Err:ident) => {
        #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, is_macro::Is)]
        pub$(($pub))? enum $Result<T, E> {
            $Ok(T),
            $Err(E),
        }
        result_like::impl_result_like!($Result, $Ok, $Err);
    };
    ($Result:ident, $Ok:ident, $Err:ident) => {
        #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, is_macro::Is)]
        enum $Result<T, E> {
            $Ok(T),
            $Err(E),
        }
        result_like::impl_result_like!($Result, $Ok, $Err);
    };
    (pub $(($pub:ident))? $Result:ident) => {
        result_like::result_like!(pub $(($pub))? $Result, Ok, Err);
    };
    ($Result:ident) => {
        result_like::result_like!($Result, Ok, Err);
    };
}

#[macro_export]
macro_rules! impl_result_like {
    ($Result:ident, $Ok:ident, $Err:ident) => {
        impl<T, E> result_like::ResultLike<T, E> for $Result<T, E> {}

        impl<T, E> $Result<T, E> {
            pub fn from_result(result: Result<T, E>) -> Self {
                match result {
                    Ok(v) => $Result::$Ok(v),
                    Err(e) => $Result::$Err(e),
                }
            }

            pub fn into_result(self) -> Result<T, E> {
                match self {
                    $Result::$Ok(v) => Ok(v),
                    $Result::$Err(e) => Err(e),
                }
            }

            pub fn as_result(&self) -> Result<&T, &E> {
                match self {
                    $Result::$Ok(ref x) => Ok(x),
                    $Result::$Err(ref x) => Err(x),
                }
            }

            pub fn as_result_mut(&mut self) -> Result<&mut T, &mut E> {
                match self {
                    $Result::$Ok(ref mut x) => Ok(x),
                    $Result::$Err(ref mut x) => Err(x),
                }
            }

            #[inline]
            pub fn as_ref(&self) -> $Result<&T, &E> {
                match self {
                    $Result::$Ok(ref x) => $Result::$Ok(x),
                    $Result::$Err(ref x) => $Result::$Err(x),
                }
            }

            #[inline]
            pub fn as_mut(&mut self) -> $Result<&mut T, &mut E> {
                match self {
                    $Result::$Ok(ref mut x) => $Result::$Ok(x),
                    $Result::$Err(ref mut x) => $Result::$Err(x),
                }
            }

            #[inline]
            pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> $Result<U, E> {
                match self {
                    $Result::$Ok(t) => $Result::$Ok(op(t)),
                    $Result::$Err(e) => $Result::$Err(e),
                }
            }

            #[inline]
            pub fn map_or_else<U, M: FnOnce(T) -> U, F: FnOnce(E) -> U>(
                self,
                fallback: F,
                map: M,
            ) -> U {
                self.map(map).unwrap_or_else(fallback)
            }

            #[inline]
            pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> $Result<T, F> {
                match self {
                    $Result::$Ok(t) => $Result::$Ok(t),
                    $Result::$Err(e) => $Result::$Err(op(e)),
                }
            }

            // iter
            // iter_mut

            #[inline]
            pub fn and<U>(self, res: $Result<U, E>) -> $Result<U, E> {
                match self {
                    $Result::$Ok(_) => res,
                    $Result::$Err(e) => $Result::$Err(e),
                }
            }

            #[inline]
            pub fn and_then<U, F: FnOnce(T) -> $Result<U, E>>(self, op: F) -> $Result<U, E> {
                match self {
                    $Result::$Ok(t) => op(t),
                    $Result::$Err(e) => $Result::$Err(e),
                }
            }

            #[inline]
            pub fn or<F>(self, res: $Result<T, F>) -> $Result<T, F> {
                match self {
                    $Result::$Ok(v) => $Result::$Ok(v),
                    $Result::$Err(_) => res,
                }
            }

            #[inline]
            pub fn or_else<F, O: FnOnce(E) -> $Result<T, F>>(self, op: O) -> $Result<T, F> {
                match self {
                    $Result::$Ok(t) => $Result::$Ok(t),
                    $Result::$Err(e) => op(e),
                }
            }

            #[inline]
            pub fn unwrap_or(self, optb: T) -> T {
                self.into_result().unwrap_or(optb)
            }

            #[inline]
            pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
                self.into_result().unwrap_or_else(op)
            }
        }

        impl<T: Copy, E> $Result<&T, E> {
            pub fn copied(self) -> $Result<T, E> {
                self.map(|&t| t)
            }
        }

        impl<T: Copy, E> $Result<&mut T, E> {
            pub fn copied(self) -> $Result<T, E> {
                self.map(|&mut t| t)
            }
        }

        impl<T: Clone, E> $Result<&T, E> {
            pub fn cloned(self) -> $Result<T, E> {
                self.map(|t| t.clone())
            }
        }

        impl<T: Clone, E> $Result<&mut T, E> {
            pub fn cloned(self) -> $Result<T, E> {
                self.map(|t| t.clone())
            }
        }

        impl<T, E: std::fmt::Debug> $Result<T, E> {
            #[inline]
            pub fn unwrap(self) -> T {
                self.into_result().unwrap()
            }

            #[inline]
            pub fn expect(self, msg: &str) -> T {
                self.into_result().expect(msg)
            }
        }

        impl<T: std::fmt::Debug, E> $Result<T, E> {
            #[inline]
            pub fn unwrap_err(self) -> E {
                self.into_result().unwrap_err()
            }
        }

        impl<T: Default, E> $Result<T, E> {
            #[inline]
            pub fn unwrap_or_default(self) -> T {
                self.into_result().unwrap_or_default()
            }
        }

        impl<T: std::ops::Deref, E> $Result<T, E> {
            pub fn as_deref_ok(&self) -> $Result<&T::Target, &E> {
                self.as_ref().map(|t| t.deref())
            }
        }

        impl<T, E: std::ops::Deref> $Result<T, E> {
            pub fn as_deref_err(&self) -> $Result<&T, &E::Target> {
                self.as_ref().map_err(|e| e.deref())
            }
        }

        impl<T: std::ops::Deref, E: std::ops::Deref> $Result<T, E> {
            pub fn as_deref(&self) -> $Result<&T::Target, &E::Target> {
                self.as_ref().map(|t| t.deref()).map_err(|e| e.deref())
            }
        }

        impl<T: std::ops::DerefMut, E> $Result<T, E> {
            pub fn as_deref_mut_ok(&mut self) -> $Result<&mut T::Target, &mut E> {
                self.as_mut().map(|t| t.deref_mut())
            }
        }

        impl<T, E: std::ops::DerefMut> $Result<T, E> {
            pub fn as_deref_mut_err(&mut self) -> $Result<&mut T, &mut E::Target> {
                self.as_mut().map_err(|e| e.deref_mut())
            }
        }

        impl<T: std::ops::DerefMut, E: std::ops::DerefMut> $Result<T, E> {
            pub fn as_deref_mut(&mut self) -> $Result<&mut T::Target, &mut E::Target> {
                self.as_mut()
                    .map(|t| t.deref_mut())
                    .map_err(|e| e.deref_mut())
            }
        }

        impl<T, E> $Result<Option<T>, E> {
            #[inline]
            pub fn transpose(self) -> Option<Result<T, E>> {
                self.into_result().transpose()
            }
        }

        impl<T: Clone, E: Clone> Clone for $Result<T, E> {
            #[inline]
            fn clone(&self) -> Self {
                match self {
                    $Result::$Ok(x) => $Result::$Ok(x.clone()),
                    $Result::$Err(x) => $Result::$Err(x.clone()),
                }
            }

            #[inline]
            fn clone_from(&mut self, source: &Self) {
                match (self, source) {
                    ($Result::$Ok(to), $Result::$Ok(from)) => to.clone_from(from),
                    ($Result::$Err(to), $Result::$Err(from)) => to.clone_from(from),
                    (to, from) => *to = from.clone(),
                }
            }
        }

        impl<T, E> IntoIterator for $Result<T, E> {
            type Item = T;
            type IntoIter = std::result::IntoIter<T>;

            #[inline]
            fn into_iter(self) -> std::result::IntoIter<T> {
                self.into_result().into_iter()
            }
        }

        // impl<'a, T, E> IntoIterator for &'a $Result<T, E> {
        //     type Item = &'a T;
        //     type IntoIter = std::result::Iter<'a, T>;

        //     fn into_iter(self) -> std::result::Iter<'a, T> {
        //         self.into_result().iter()
        //     }
        // }

        // impl<'a, T, E> IntoIterator for &'a mut Result<T, E> {
        //     type Item = &'a mut T;
        //     type IntoIter = IterMut<'a, T>;

        //     fn into_iter(self) -> IterMut<'a, T> {
        //         self.iter_mut()
        //     }
        // }
    };
    ($Result:ident) => {
        result_like::impl_result_like!($Result, Ok, Err);
    };
}
