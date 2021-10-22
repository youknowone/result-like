#![recursion_limit = "512"]

extern crate proc_macro;

use inflector::Inflector;
use pmutil::{smart_quote, Quote, ToTokensExt};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    token::Comma, Data, DataEnum, DeriveInput, Field, Generics, Ident, ItemMod, WhereClause,
    WherePredicate,
};

#[proc_macro_derive(OptionLike)]
pub fn option_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).expect("failed to parse derive input");

    let data = match input.data {
        Data::Enum(ref data) => data,
        _ => panic!("`OptionLike` can be applied only on enums"),
    };

    let item_mod = expand(&input, OptionLike, data);
    let items = item_mod.content.unwrap().1;
    quote!(#(#items)*).into()
}

#[proc_macro_derive(ResultLike)]
pub fn result_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).expect("failed to parse derive input");

    let data = match input.data {
        Data::Enum(ref data) => data,
        _ => panic!("`ResultLike` can be applied only on enums"),
    };

    let item_mod = expand(&input, ResultLike, data);
    let items = item_mod.content.unwrap().1;
    quote!(#(#items)*).into()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VariantFieldsType {
    Unnamed,
    Unit,
}

impl VariantFieldsType {
    fn matches(self, fields: &syn::Fields) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, fields) {
            (VariantFieldsType::Unnamed, syn::Fields::Unnamed(_)) => true,
            (VariantFieldsType::Unit, syn::Fields::Unit) => true,
            _ => false,
        }
    }
}

trait LikeTrait {
    fn data(&self) -> LikeData;
    fn quote_impl(&self, args: ImplArgs) -> Quote;
}

struct ImplArgs<'a> {
    typ: &'a Ident,
    generics: &'a Generics,
    primary: &'a Ident,
    secondary: &'a Ident,
    primary_inner: &'a Punctuated<Field, Comma>,
    secondary_inner: Option<&'a Punctuated<Field, Comma>>,
}

impl<'a> ImplArgs<'a> {
    fn split_for_impl(
        &self,
    ) -> (
        Generics,
        Generics,
        Option<&WhereClause>,
        Punctuated<WherePredicate, Comma>,
    ) {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let impl_generics =
            syn::parse2::<Generics>(impl_generics.dump()).expect("generics to generics");
        let ty_generics =
            syn::parse2::<Generics>(ty_generics.dump()).expect("generics to generics");
        let where_predicates = where_clause.map_or(
            WhereClause {
                where_token: Default::default(),
                predicates: Default::default(),
            }
            .predicates,
            |w| w.predicates.clone(),
        );
        (impl_generics, ty_generics, where_clause, where_predicates)
    }
}

struct LikeData {
    name: String,
    fields: (VariantFieldsType, VariantFieldsType),
}

fn expand(input: &DeriveInput, like_trait: impl LikeTrait, data: &DataEnum) -> ItemMod {
    let typ = &input.ident;
    let like = like_trait.data();

    assert_eq!(
        data.variants.len(),
        2,
        "{} expects 2 variants but {} variant(s) given",
        &like.name,
        data.variants.len()
    );

    let (primary_variant, secondary_variant) = {
        let mut iter = data.variants.iter();
        (iter.next().unwrap(), iter.next().unwrap())
    };

    assert!(like.fields.0.matches(&primary_variant.fields));
    assert!(like.fields.1.matches(&secondary_variant.fields));

    let primary = &primary_variant.ident;
    let secondary = &secondary_variant.ident;

    let primary_inner = match &primary_variant.fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => unnamed,
        _ => unreachable!(),
    };
    let secondary_inner = match &secondary_variant.fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => Some(unnamed),
        _ => None,
    };

    let mod_name = Ident::new(
        &format!(
            "_impl_{}_{}",
            like.name.to_snake_case(),
            typ.to_string().to_snake_case()
        ),
        typ.span(),
    );

    let like_impl = like_trait.quote_impl(ImplArgs {
        typ,
        generics: &input.generics,
        primary,
        secondary,
        primary_inner,
        secondary_inner,
    });

    syn::parse2(quote! {
        mod #mod_name {
            #like_impl
        }
    })
    .expect("This is a bug. Please report the input used to show this error.")
}

struct OptionLike;

impl LikeTrait for OptionLike {
    fn data(&self) -> LikeData {
        LikeData {
            name: "OptionLike".to_owned(),
            fields: (VariantFieldsType::Unnamed, VariantFieldsType::Unit),
        }
    }

    fn quote_impl(&self, args: ImplArgs) -> Quote {
        let ImplArgs {
            typ,
            primary,
            secondary,
            primary_inner,
            ..
        } = args;
        let (impl_generics, ty_generics, where_clause, where_predicates) = args.split_for_impl();
        let mut option_impl = Quote::new_call_site().quote_with(smart_quote!(
            Vars {
                Type: &typ,
                impl_generics: &impl_generics,
                ty_generics: &ty_generics,
                where_predicates: &where_predicates,
                where_clause: &where_clause,
                Primary: primary,
                Secondary: secondary,
                PrimaryValue: primary_inner,
            },
            {
                impl impl_generics result_like::OptionLike for Type ty_generics where_clause {
                    type SomeType = PrimaryValue;
                }
                impl impl_generics Type ty_generics where_clause {
                    #[inline]
                    pub fn from_option(option: Option<PrimaryValue>) -> Self {
                        match option {
                            Some(v) => Type::Primary(v),
                            None => Type::Secondary,
                        }
                    }

                    #[inline]
                    pub fn into_option(self) -> Option<PrimaryValue> {
                        match self {
                            Type::Primary(v) => Some(v),
                            Type::Secondary => None,
                        }
                    }

                    #[inline]
                    pub fn as_option(&self) -> Option<&PrimaryValue> {
                        match self {
                            Type::Primary(ref v) => Some(v),
                            Type::Secondary => None,
                        }
                    }

                    #[inline]
                    pub fn as_option_mut(&mut self) -> Option<&mut PrimaryValue> {
                        match self {
                            Type::Primary(ref mut v) => Some(v),
                            Type::Secondary => None,
                        }
                    }

                    #[inline]
                    pub fn expect(self, msg: &str) -> PrimaryValue where {
                        self.into_option().expect(msg)
                    }

                    #[inline]
                    pub fn unwrap(self) -> PrimaryValue {
                        self.into_option().unwrap()
                    }

                    #[inline]
                    pub fn unwrap_or(self, default: PrimaryValue) -> PrimaryValue {
                        self.into_option().unwrap_or(default)
                    }

                    #[inline]
                    pub fn unwrap_or_else<_Function: FnOnce() -> PrimaryValue>(self, f: _Function) -> PrimaryValue {
                        self.into_option().unwrap_or_else(f)
                    }

                    #[inline]
                    pub fn ok_or<_Error>(self, err: _Error) -> Result<PrimaryValue, _Error> {
                        self.into_option().ok_or(err)
                    }

                    #[inline]
                    pub fn ok_or_else<_Error, _Function: FnOnce() -> _Error>(self, err: _Function) -> Result<PrimaryValue, _Error> {
                        self.into_option().ok_or_else(err)
                    }

                    #[inline]
                    pub fn filter<P: FnOnce(&PrimaryValue) -> bool>(self, predicate: P) -> Self {
                        Self::from_option(self.into_option().filter(predicate))
                    }

                    #[inline]
                    pub fn or(self, optb: Self) -> Self {
                        Self::from_option(self.into_option().or(optb.into_option()))
                    }

                    #[inline]
                    pub fn or_else<_Function: FnOnce() -> Self>(self, f: _Function) -> Self {
                        Self::from_option(self.into_option().or_else(|| f().into_option()))
                    }

                    #[inline]
                    pub fn map_or<_Other, _Function: FnOnce(PrimaryValue) -> _Other>(
                        self,
                        default: _Other,
                        f: _Function,
                    ) -> _Other {
                        self.into_option().map_or(default, f)
                    }

                    #[inline]
                    pub fn xor(self, optb: Self) -> Self {
                        Self::from_option(self.into_option().xor(optb.into_option()))
                    }

                    #[inline]
                    pub fn get_or_insert(&mut self, v: PrimaryValue) -> &mut PrimaryValue {
                        self.get_or_insert_with(|| v)
                    }

                    #[inline]
                    pub fn get_or_insert_with<_Function: FnOnce() -> PrimaryValue>(&mut self, f: _Function) -> &mut PrimaryValue {
                        if let Type::Secondary = *self {
                            *self = Type::Primary(f());
                        }

                        match *self {
                            Type::Primary(ref mut v) => v,
                            Type::Secondary => unsafe { std::hint::unreachable_unchecked() },
                        }
                    }

                    #[inline]
                    pub fn take(&mut self) -> Self where where_predicates PrimaryValue: Default {
                        std::mem::take(self)
                    }

                    #[inline]
                    pub fn replace(&mut self, value: PrimaryValue) -> Self {
                        std::mem::replace(self, Type::Primary(value))
                    }
                }

                impl impl_generics Type ty_generics where where_predicates PrimaryValue: Default {
                    #[inline]
                    pub fn unwrap_or_default(self) -> PrimaryValue {
                        self.into_option().unwrap_or_default()
                    }
                }

                impl impl_generics Copy for Type ty_generics where where_predicates PrimaryValue: Copy {}

                impl impl_generics Clone for Type ty_generics where where_predicates PrimaryValue: Clone {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Type::Primary(x) => Type::Primary(x.clone()),
                            Type::Secondary => Type::Secondary,
                        }
                    }

                    #[inline]
                    fn clone_from(&mut self, source: &Self) {
                        match (self, source) {
                            (Type::Primary(to), Type::Primary(from)) => to.clone_from(from),
                            (to, from) => *to = from.clone(),
                        }
                    }
                }

                impl impl_generics From<PrimaryValue> for Type ty_generics where_clause {
                    #[inline]
                    fn from(value: PrimaryValue) -> Self {
                        Type::Primary(value)
                    }
                }

                impl impl_generics Default for Type ty_generics {
                    #[inline]
                    fn default() -> Self {
                        Type::Secondary
                    }
                }

                impl impl_generics IntoIterator for Type ty_generics where_clause {
                    type Item = PrimaryValue;
                    type IntoIter = std::option::IntoIter<PrimaryValue>;

                    #[inline]
                    fn into_iter(self) -> std::option::IntoIter<PrimaryValue> {
                        self.into_option().into_iter()
                    }
                }
            }
        ));
        if !ty_generics.params.is_empty() {
            option_impl = option_impl.quote_with(smart_quote!(
                Vars {
                    Type: &typ,
                    impl_generics: &impl_generics,
                    ty_generics: &ty_generics,
                    where_predicates: &where_predicates,
                    where_clause: &where_clause,
                    Primary: primary,
                    Secondary: secondary,
                    PrimaryValue: primary_inner,
                },
                {
                    impl impl_generics Type ty_generics where_clause {
                        #[inline]
                        pub fn as_ref(&self) -> Type<&PrimaryValue> {
                            match *self {
                                Type::Primary(ref x) => Type::Primary(x),
                                Type::Secondary => Type::Secondary,
                            }
                        }

                        #[inline]
                        pub fn as_mut(&mut self) -> Type<&mut PrimaryValue> {
                            match *self {
                                Type::Primary(ref mut x) => Type::Primary(x),
                                Type::Secondary => Type::Secondary,
                            }
                        }

                        // as_pin_ref
                        // as_pin_mut

                        #[inline]
                        pub fn map<_Other, _Function: FnOnce(PrimaryValue) -> _Other>(self, f: _Function) -> Type<_Other> {
                            match self {
                                Type::Primary(x) => Type::Primary(f(x)),
                                Type::Secondary => Type::Secondary,
                            }
                        }

                        #[inline]
                        pub fn map_or_else<_Other, _Default: FnOnce() -> _Other, _Function: FnOnce(PrimaryValue) -> _Other>(
                            self,
                            default: _Default,
                            f: _Function,
                        ) -> _Other {
                            self.into_option().map_or_else(default, f)
                        }

                        // iter
                        // iter_mut

                        #[inline]
                        pub fn and<_Other>(self, optb: Type<_Other>) -> Type<_Other> {
                            match self {
                                Type::Primary(_) => optb,
                                Type::Secondary => Type::Secondary,
                            }
                        }

                        #[inline]
                        pub fn and_then<_Other, _Function: FnOnce(PrimaryValue) -> Type<_Other>>(self, f: _Function) -> Type<_Other> {
                            match self {
                                Type::Primary(x) => f(x),
                                Type::Secondary => Type::Secondary,
                            }
                        }

                        pub fn zip<_Other>(self, other: Type<_Other>) -> Type<(PrimaryValue, _Other)> {
                            Type::from_option(self.into_option().zip(other.into_option()))
                        }

                        // pub fn zip_with<_Other, _Function, _Result>(self, other: Type<_Other>, f: _Function) -> Type<_Result>
                        // where
                        //     _Function: FnOnce(PrimaryValue, _Other) -> _Result,
                        // {
                        //     Type::from_option(self.into_option().zip_with(other.into_option(), f))
                        // }
                    }

                    impl impl_generics Type<&PrimaryValue> where where_predicates PrimaryValue: Copy {
                        pub fn copied(self) -> Type<PrimaryValue> {
                            self.map(|&t| t)
                        }
                    }

                    impl impl_generics Type<&mut PrimaryValue> where where_predicates PrimaryValue: Copy {
                        pub fn copied(self) -> Type<PrimaryValue> {
                            self.map(|&mut t| t)
                        }
                    }

                    impl impl_generics Type<&PrimaryValue> where where_predicates PrimaryValue: Clone {
                        pub fn cloned(self) -> Type<PrimaryValue> {
                            self.map(|t| t.clone())
                        }
                    }

                    impl impl_generics Type<&mut PrimaryValue> where where_predicates PrimaryValue: Clone {
                        pub fn cloned(self) -> Type<PrimaryValue> {
                            self.map(|t| t.clone())
                        }
                    }

                    // impl<T: fmt::Debug> Type<T>
                    // expect_none
                    // unwrap_none

                    impl impl_generics Type<PrimaryValue> where where_predicates PrimaryValue: std::ops::Deref {
                        pub fn as_deref(&self) -> Type<&PrimaryValue::Target> {
                            self.as_ref().map(|t| t.deref())
                        }
                    }

                    impl<PrimaryValue: std::ops::DerefMut> Type<PrimaryValue> {
                        pub fn as_deref_mut(&mut self) -> Type<&mut PrimaryValue::Target> {
                            self.as_mut().map(|t| t.deref_mut())
                        }
                    }

                    impl<PrimaryValue, _Error> Type<Result<PrimaryValue, _Error>> {
                        #[inline]
                        pub fn transpose(self) -> Result<Type<PrimaryValue>, _Error> {
                            self.into_option()
                                .transpose()
                                .map(|op| Type::from_option(op))
                        }
                    }

                    // TODO: result-like

                    // impl<'a, PrimaryValue> IntoIterator for &'a Type<PrimaryValue> {
                    //     type Item = &'a PrimaryValue;
                    //     type IntoIter = std::option::Iter<'a, PrimaryValue>;

                    //     fn into_iter(self) -> std::option::Iter<'a, PrimaryValue> {
                    //         self.iter()
                    //     }
                    // }

                    // impl<'a, T> IntoIterator for &'a mut Self {
                    //     type Item = &'a mut T;
                    //     type IntoIter = std::option::IterMut<'a, T>;

                    //     fn into_iter(self) -> std::option::IterMut<'a, T> {
                    //         self.iter_mut()
                    //     }
                    // }

                    // impl impl_generics From ty_generics  for Type ty_generics where_clause {
                    //     fn from(val: PrimaryValue) -> Self {
                    //         Type::Primary(val)
                    //     }
                    // }

                    // impl<'a, T> From<&'a Type<T>> for Type<&'a T> {
                    //     fn from(o: &'a Type<T>) -> Type<&'a T> {
                    //         o.as_ref()
                    //     }
                    // }

                    // impl<'a, T> From<&'a mut Type<T>> for Type<&'a mut T> {
                    //     fn from(o: &'a mut Type<T>) -> Type<&'a mut T> {
                    //         o.as_mut()
                    //     }
                    // }
                }
            ));
        }
        option_impl
    }
}

struct ResultLike;

impl LikeTrait for ResultLike {
    fn data(&self) -> LikeData {
        LikeData {
            name: "ResultLike".to_owned(),
            fields: (VariantFieldsType::Unnamed, VariantFieldsType::Unnamed),
        }
    }

    fn quote_impl(&self, args: ImplArgs) -> Quote {
        let ImplArgs {
            typ,
            primary,
            secondary,
            primary_inner,
            secondary_inner,
            ..
        } = args;
        let (impl_generics, ty_generics, where_clause, where_predicates) = args.split_for_impl();
        let mut result_impl = Quote::new_call_site().quote_with(smart_quote!(
            Vars {
                Type: &typ,
                impl_generics: &impl_generics,
                ty_generics: &ty_generics,
                where_predicates: &where_predicates,
                where_clause: &where_clause,
                Primary: primary,
                Secondary: secondary,
                T: primary_inner,
                E: secondary_inner,
            },
            {
                impl impl_generics Type ty_generics where_clause {
                    #[inline]
                    pub fn from_result(result: Result<T, E>) -> Self {
                        match result {
                            Ok(v) => Type::Primary(v),
                            Err(e) => Type::Secondary(e),
                        }
                    }

                    #[inline]
                    pub fn into_result(self) -> Result<T, E> {
                        match self {
                            Type::Primary(v) => Ok(v),
                            Type::Secondary(e) => Err(e),
                        }
                    }

                    #[inline]
                    pub fn as_result(&self) -> Result<&T, &E> {
                        match self {
                            Type::Primary(ref x) => Ok(x),
                            Type::Secondary(ref x) => Err(x),
                        }
                    }

                    #[inline]
                    pub fn as_result_mut(&mut self) -> Result<&mut T, &mut E> {
                        match self {
                            Type::Primary(ref mut x) => Ok(x),
                            Type::Secondary(ref mut x) => Err(x),
                        }
                    }


                    #[inline]
                    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
                        match self {
                            Type::Primary(t) => f(t),
                            Type::Secondary(_) => default,
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

                impl impl_generics Type ty_generics where where_predicates E: std::fmt::Debug {
                    #[inline]
                    pub fn expect(self, msg: &str) -> T {
                        self.into_result().expect(msg)
                    }

                    #[inline]
                    pub fn unwrap(self) -> T {
                        self.into_result().unwrap()
                    }
                }

                impl impl_generics Type ty_generics where where_predicates T: std::fmt::Debug {
                    // #[inline]
                    // pub fn expect_err(self, msg: &str) -> E {
                    //     self.into_result().expect_err(msg)
                    // }

                    #[inline]
                    pub fn unwrap_err(self) -> E {
                        self.into_result().unwrap_err()
                    }
                }

                impl impl_generics Type ty_generics where where_predicates T: Default {
                    #[inline]
                    pub fn unwrap_or_default(self) -> T {
                        self.into_result().unwrap_or_default()
                    }
                }

                // into_ok

                impl impl_generics Copy for Type ty_generics where where_predicates T: Copy, E: Copy { }
                impl impl_generics Clone for Type ty_generics where where_predicates T: Clone, E: Clone {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Type::Primary(x) => Type::Primary(x.clone()),
                            Type::Secondary(x) => Type::Secondary(x.clone()),
                        }
                    }

                    #[inline]
                    fn clone_from(&mut self, source: &Self) {
                        match (self, source) {
                            (Type::Primary(to), Type::Primary(from)) => to.clone_from(from),
                            (Type::Secondary(to), Type::Secondary(from)) => to.clone_from(from),
                            (to, from) => *to = from.clone(),
                        }
                    }
                }

            }
        ));
        if !ty_generics.params.is_empty() {
            result_impl = result_impl.quote_with(smart_quote!(
                Vars {
                    Type: &typ,
                    impl_generics: &impl_generics,
                    ty_generics: &ty_generics,
                    where_predicates: &where_predicates,
                    where_clause: &where_clause,
                    Primary: primary,
                    Secondary: secondary,
                    T: primary_inner,
                    E: secondary_inner,
                },
                {
                impl impl_generics result_like::ResultLike for Type ty_generics where_clause {
                    type OkType = T;
                    type ErrType = E;
                }
                impl impl_generics Type ty_generics where_clause {
                    // contains
                    // contains_err

                    #[inline]
                    pub fn as_ref(&self) -> Type<&T, &E> {
                        match self {
                            Type::Primary(ref x) => Type::Primary(x),
                            Type::Secondary(ref x) => Type::Secondary(x),
                        }
                    }

                    #[inline]
                    pub fn as_mut(&mut self) -> Type<&mut T, &mut E> {
                        match self {
                            Type::Primary(ref mut x) => Type::Primary(x),
                            Type::Secondary(ref mut x) => Type::Secondary(x),
                        }
                    }

                       #[inline]
                    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Type<U, E> {
                        match self {
                            Type::Primary(t) => Type::Primary(op(t)),
                            Type::Secondary(e) => Type::Secondary(e),
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
                    pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Type<T, F> {
                        match self {
                            Type::Primary(t) => Type::Primary(t),
                            Type::Secondary(e) => Type::Secondary(op(e)),
                        }
                    }

                    // iter
                    // iter_mut

                    #[inline]
                    pub fn and<U>(self, res: Type<U, E>) -> Type<U, E> {
                        match self {
                            Type::Primary(_) => res,
                            Type::Secondary(e) => Type::Secondary(e),
                        }
                    }

                    #[inline]
                    pub fn and_then<U, F: FnOnce(T) -> Type<U, E>>(self, op: F) -> Type<U, E> {
                        match self {
                            Type::Primary(t) => op(t),
                            Type::Secondary(e) => Type::Secondary(e),
                        }
                    }

                    #[inline]
                    pub fn or<F>(self, res: Type<T, F>) -> Type<T, F> {
                        match self {
                            Type::Primary(v) => Type::Primary(v),
                            Type::Secondary(_) => res,
                        }
                    }

                    #[inline]
                    pub fn or_else<F, O: FnOnce(E) -> Type<T, F>>(self, op: O) -> Type<T, F> {
                        match self {
                            Type::Primary(t) => Type::Primary(t),
                            Type::Secondary(e) => op(e),
                        }
                    }
                }

                impl<T: Copy, E> Type<&T, E> {
                    pub fn copied(self) -> Type<T, E> {
                        self.map(|&t| t)
                    }
                }

                impl<T: Copy, E> Type<&mut T, E> {
                    pub fn copied(self) -> Type<T, E> {
                        self.map(|&mut t| t)
                    }
                }

                impl<T: Clone, E> Type<&T, E> {
                    pub fn cloned(self) -> Type<T, E> {
                        self.map(|t| t.clone())
                    }
                }

                impl<T: Clone, E> Type<&mut T, E> {
                    pub fn cloned(self) -> Type<T, E> {
                        self.map(|t| t.clone())
                    }
                }

                // impl<T: std::ops::Deref, E> Type<T, E> {
                //     pub fn as_deref_ok(&self) -> Type<&T::Target, &E> {
                //         self.as_ref().map(|t| t.deref())
                //     }
                // }

                // impl<T, E: std::ops::Deref> Type<T, E> {
                //     pub fn as_deref_err(&self) -> Type<&T, &E::Target> {
                //         self.as_ref().map_err(|e| e.deref())
                //     }
                // }

                impl impl_generics Type ty_generics where where_predicates T: std::ops::Deref, E: std::ops::Deref {
                    pub fn as_deref(&self) -> Type<&T::Target, &E::Target> {
                        self.as_ref().map(|t| t.deref()).map_err(|e| e.deref())
                    }
                }

                // impl<T: std::ops::DerefMut, E> Type<T, E> {
                //     pub fn as_deref_mut_ok(&mut self) -> Type<&mut T::Target, &mut E> {
                //         self.as_mut().map(|t| t.deref_mut())
                //     }
                // }

                // impl<T, E: std::ops::DerefMut> Type<T, E> {
                //     pub fn as_deref_mut_err(&mut self) -> Type<&mut T, &mut E::Target> {
                //         self.as_mut().map_err(|e| e.deref_mut())
                //     }
                // }

                // impl<T: std::ops::DerefMut, E: std::ops::DerefMut> Type<T, E> {
                //     pub fn as_deref_mut(&mut self) -> Type<&mut T::Target, &mut E::Target> {
                //         self.as_mut()
                //             .map(|t| t.deref_mut())
                //             .map_err(|e| e.deref_mut())
                //     }
                // }

                impl<T, E> Type<Option<T>, E> {
                    #[inline]
                    pub fn transpose(self) -> Option<Type<T, E>> {
                        self.into_result()
                            .transpose()
                            .map(|r| Type::from_result(r))
                    }
                }

                // flatten

                impl<T, E> IntoIterator for Type<T, E> {
                    type Item = T;
                    type IntoIter = std::result::IntoIter<T>;

                    #[inline]
                    fn into_iter(self) -> std::result::IntoIter<T> {
                        self.into_result().into_iter()
                    }
                }

                // impl<'a, T, E> IntoIterator for &'a Type<T, E> {
                //     type Item = &'a T;
                //     type IntoIter = std::result::Iter<'a, T>;

                //     fn into_iter(self) -> std::result::Iter<'a, T> {
                //         self.into_result().iter()
                //     }
                // }

                // impl<'a, T, E> IntoIterator for &'a mut Type<T, E> {
                //     type Item = &'a mut T;
                //     type IntoIter = IterMut<'a, T>;

                //     fn into_iter(self) -> IterMut<'a, T> {
                //         self.iter_mut()
                //     }
                // }
                }
            ));
        }
        result_impl
    }
}
