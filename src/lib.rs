//! Trait aliases.
//!
//! The core functionality is provided by the [`trait_aliases!`] procedural macro.
//!
//! # Example
//!
//! Ever felt tired of writing `T: Send + Sync + 'static` over and over when working with `async`
//! in multi-threaded scenarios? Simply define an alias without blanket implementation boilerplate!
//!
//! ```
//! use trait_aliases::trait_aliases;
//!
//! trait_aliases! {
//!     /// Working in multi-threaded `async` contexts often requires these.
//!     pub trait SSS = Send + Sync + 'static;
//! }
//! ```
//!
//! This crate will generate the `SSS` trait with the provided bounds, and implement it for any type
//! satisfying them:
//!
//! ```
//! /// Working in multi-threaded `async` contexts often requires these.
//! pub trait SSS: Send + Sync + 'static {}
//!
//! /// Blanket implementation of [`SSS`] for all types satisfying its bounds.
//! impl<__T> SSS for __T where __T: Send + Sync + 'static + ?Sized {}
//! ```
//!
//! # Attribute
//!
//! The [`trait_alias`] attribute can be attached to any trait definition within the
//! input to [`trait_aliases!`]. See the attribute [documentation](trait_alias) for full description.
//!
//! ```
//! use trait_aliases::trait_aliases;
//!
//! trait_aliases! {
//!     /// Working in multi-threaded `async` contexts often requires these.
//!     #[trait_alias(
//!         T,
//!         doc = "Implemented for any type that is [`Send`], [`Sync`] and `'static`",
//!         doc = "(meaning it does not contain non-static lifetimes)."
//!     )]
//!     pub trait SSS = Send + Sync + 'static;
//! }
//! ```
//!
//! Gets expanded to:
//!
//! ```
//! /// Working in multi-threaded `async` contexts often requires these.
//! pub trait SSS: Send + Sync + 'static {}
//!
//! /// Implemented for any type that is [`Send`], [`Sync`] and `'static`
//! /// (meaning it does not contain non-static lifetimes).
//! impl<T> SSS for T where T: Send + Sync + 'static + ?Sized {}
//! ```
//!
//! # Generic
//!
//! Defining generic trait aliases is also supported:
//!
//! ```
//! use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
//!
//! use trait_aliases::trait_aliases;
//!
//! /// Defines an additive identity element for [`Self`].
//! pub trait Zero: Add<Output = Self> + Sized {
//!     /// The identity element of [`Self`], `0`.
//!     const ZERO: Self;
//!
//!     /// Returns [`true`] if `self` is equal to the additive identity.
//!     fn is_zero(&self) -> bool;
//! }
//!
//! /// Defines a multiplicative identity element for [`Self`].
//! pub trait One: Mul<Output = Self> + Sized {
//!     /// The multiplicative identity of [`Self`], `1`.
//!     const ONE: Self;
//!
//!     /// Returns [`true`] if `self` is equal to the multiplicative identity.
//!     fn is_one(&self) -> bool;
//! }
//!
//! trait_aliases! {
//!     /// Represents types implementing basic numeric operations.
//!     #[trait_alias(N)]
//!     pub trait NumOps<R = Self, T = Self> =
//!         Add<R, Output = T>
//!         + Sub<R, Output = T>
//!         + Mul<R, Output = T>
//!         + Div<R, Output = T>
//!         + Rem<R, Output = T>;
//!
//!     /// Represents types implementing numeric assignment operations.
//!     #[trait_alias(N)]
//!     pub trait NumAssignOps<R = Self> =
//!         AddAssign<R> + SubAssign<R> + MulAssign<R> + DivAssign<R> + RemAssign<R>;
//!
//!     /// Represents numeric types that have `0` and `1` values, can be compared for equality
//!     /// and operated on.
//!     #[trait_alias(N)]
//!     pub trait Num = PartialEq + Zero + One + NumOps;
//!
//!     /// Represents [`Num`] types which also implement assignment operations.
//!     #[trait_alias(N)]
//!     pub trait NumAssign = Num + NumAssignOps;
//!
//!     /// Represents [`Num`] types which also implement numeric operations taking
//!     /// the right-hand side operand by reference.
//!     #[trait_alias(N)]
//!     pub trait NumRef = Num + for<'r> NumOps<&'r Self>;
//!
//!     /// Represents [`NumAssign`] types which also implement numeric assignment by reference.
//!     #[trait_alias(N)]
//!     pub trait NumAssignRef = NumAssign + for<'r> NumAssignOps<&'r Self>;
//! }
//! ```
//!
//! # Attributes
//!
//! Any attributes applied to the trait alias will be copied to both the generated trait definition
//! and its blanket implementation, except for documentation comments which are only applied to the
//! trait definition.
//!
//! So, for instance, using `#[cfg]` attributes for conditional compilation:
//!
//! ```
//! use core::hash::Hash;
//!
//! #[cfg(feature = "serde")]
//! use serde::{Deserialize, Serialize};
//!
//! use trait_aliases::trait_aliases;
//!
//! trait_aliases! {
//!     /// Represents base identifier bounds.
//!     #[trait_alias(T)]
//!     pub trait BaseId = Copy + Ord + Hash;
//!
//!     /// Represents types that can be serialized and deserialized.
//!     #[cfg(feature = "serde")]
//!     #[trait_alias(T)]
//!     pub trait Serializable = Serialize + for<'de> Deserialize<'de>;
//!
//!     /// Represents identifier types.
//!     #[cfg(feature = "serde")]
//!     #[trait_alias(T)]
//!     pub trait Id = BaseId + Serializable;
//!
//!     /// Represents identifier types.
//!     #[cfg(not(feature = "serde"))]
//!     #[trait_alias(T)]
//!     pub trait Id = BaseId;
//! }
//! ```
//!
//! Which will generate the following code with `serde` enabled:
//!
//! ```
//! use core::hash::Hash;
//!
//! use serde::{Deserialize, Serialize};
//!
//! /// Represents base identifier bounds.
//! pub trait BaseId: Copy + Ord + Hash {}
//!
//! /// Blanket implementation of [`BaseId`] for all types satisfying its bounds.
//! impl<T> BaseId for T where T: Copy + Ord + Hash + ?Sized {}
//!
//! /// Represents types that can be serialized and deserialized.
//! pub trait Serializable: Serialize + for<'de> Deserialize<'de> {}
//!
//! /// Blanket implementation of [`Serializable`] for all types satisfying its bounds.
//! impl<T> Serializable for T where T: Serialize + for<'de> Deserialize<'de> + ?Sized {}
//!
//! /// Represents identifier types.
//! pub trait Id: BaseId + Serializable {}
//!
//! /// Blanket implementation of [`Id`] for all types satisfying its bounds.
//! impl<T> Id for T where T: BaseId + Serializable + ?Sized {}
//! ```
//!
//! And without it:
//!
//! ```
//! use core::hash::Hash;
//!
//! /// Represents base identifier bounds.
//! pub trait BaseId: Copy + Ord + Hash {}
//!
//! /// Blanket implementation of [`BaseId`] for all types satisfying its bounds.
//! impl<T> BaseId for T where T: Copy + Ord + Hash + ?Sized {}
//!
//! /// Represents identifier types.
//! pub trait Id: BaseId {}
//!
//! /// Blanket implementation of [`Id`] for all types satisfying its bounds.
//! impl<T> Id for T where T: BaseId + ?Sized {}
//! ```
//!
//! # Note
//!
//! The blanket identifier is essential to correct code generation, therefore *any* occurrences
//! of the selected identifier will result in compilation errors.
//!
//! When the identifier is supplied to [`trait_alias`], for instance:
//!
//! ```compile_fail
//! use trait_aliases::trait_aliases;
//!
//! trait_aliases! {
//!     #[trait_alias(T)]
//!     trait Convertible<T> = From<T> + Into<T>;
//! }
//! ```
//!
//! will cause compilation to fail with several errors like:
//!
//! ```text
//! identifier `T` is reserved for blanket implementations
//! ```
//!
//! pointing to every occurrence of `T` within the trait alias definition.
//!
//! Otherwise, the default `__T` is used, therefore examples like:
//!
//! ```compile_fail
//! use trait_aliases::trait_aliases;
//!
//! trait_aliases! {
//!     trait __T = Sized;
//! }
//! ```
//!
//! fail with the following error:
//!
//! ```text
//! error: identifier `__T` is reserved for blanket implementations
//!  --> src/lib.rs
//!   |
//!   |     trait __T = Sized;
//!   |           ^^^
//! ```

use proc_macro::TokenStream;
use syn::{Error, ItemTraitAlias, parse_macro_input};

mod arguments;
mod at_most_one;
mod blanket;
mod context;
mod generate;
mod input;
mod name;
mod parse;

use arguments::Optional;
use input::TraitAliasInput;
use parse::TraitAliases;

/// Defines trait aliases with blanket implementations.
///
/// See [crate-level] documentation for more information.
///
/// [crate-level]: crate
#[proc_macro]
pub fn trait_aliases(tokens: TokenStream) -> TokenStream {
    let mut aliases = parse_macro_input!(tokens as TraitAliases);

    generate::extract_trait_aliases(&mut aliases)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Defines trait aliases with blanket implementations (attribute).
///
/// This attribute is exported mainly for documentation purposes. It *does* work on recent versions
/// of Rust, however the compiler is going to issue warnings since the feature[^trait_alias] is
/// **unstable** and the syntax can theoretically change, causing breakage.
///
/// # Arguments
///
/// The following sections describe all supported arguments.
///
/// ## Blanket type
///
/// The *blanket type* is the identifier used in blanket implementations of generated traits:
///
/// ```
/// use trait_aliases::trait_alias;
///
/// /// Represents types that are [`Send`], [`Sync`] and `'static`.
/// #[trait_alias(T)]
/// pub trait SSS = Send + Sync + 'static;
/// ```
///
/// Expands to:
///
/// ```
/// /// Represents types that are [`Send`], [`Sync`] and `'static`.
/// pub trait SSS: Send + Sync + 'static {}
///
/// /// Blanket implementation of [`SSS`] for all types satisfying its bounds.
/// impl<T> SSS for T where T: Send + Sync + 'static + ?Sized {}
/// ```
///
/// This argument is *optional*, with `__T` being the default blanket type.
///
/// Repeating the argument will cause compilation to fail:
///
/// ```compile_fail
/// use trait_aliases::trait_alias;
///
/// #[trait_alias(T, U)]
/// pub trait S = Sized;
/// ```
///
/// ```text
/// error: duplicate identifier argument `U`, already set to `T`
///  --> src/lib.rs
///   |
///   | #[trait_alias(T, U)]
///   |                  ^
/// ```
///
/// ## Blanket documentation
///
/// The *blanket documentation* is the expression passed as documentation of blanket
/// implementations:
///
/// ```
/// use core::fmt::{Debug, Display};
///
/// use trait_aliases::trait_alias;
///
/// /// Represents types that are both [`Debug`] and [`Display`].
/// #[trait_alias(doc = "Implemented for any type that is both [`Debug`] and [`Display`].")]
/// pub trait DD = Debug + Display;
/// ```
///
/// Expands to:
///
/// ```
/// use core::fmt::{Debug, Display};
///
/// /// Represents types that are both [`Debug`] and [`Display`].
/// pub trait DD: Debug + Display {}
///
/// /// Implemented for any type that is both [`Debug`] and [`Display`].
/// impl<__T> DD for __T where __T: Debug + Display + ?Sized {}
/// ```
///
/// The `doc` argument can be repeated. Each occurrence is going to add another `#[doc]` attribute
/// to the blanket implementation:
///
/// ```
/// use trait_aliases::trait_alias;
///
/// /// Represents types that are [`Send`], [`Sync`] and `'static`.
/// #[trait_alias(
///     doc = "Implemented for any type that is [`Send`], [`Sync`] and `'static`",
///     doc = "(meaning it does not contain non-static lifetimes)."
/// )]
/// pub trait SSS = Send + Sync + 'static;
/// ```
///
/// Expands to:
///
/// ```
/// /// Represents types that are [`Send`], [`Sync`] and `'static`.
/// pub trait SSS: Send + Sync + 'static {}
///
/// /// Implemented for any type that is [`Send`], [`Sync`] and `'static`
/// /// (meaning it does not contain non-static lifetimes).
/// impl<__T> SSS for __T where __T: Send + Sync + 'static + ?Sized {}
/// ```
///
/// This argument is optional; the default blanket documentation is generated as detailed below.
///
/// ### Generated
///
/// For some trait alias `Alias`, the default blanket documentation is:
///
/// ```text
/// Blanket implementation of [`Alias`] for all types satisfying its bounds.
/// ```
///
/// # Combined
///
/// ```
/// use trait_aliases::trait_alias;
///
/// /// Represents types that are convertible from and into `T`.
/// #[trait_alias(U, doc = "Implementation for all types convertible from and into `T`.")]
/// pub trait Convertible<T> = From<T> + Into<T>;
/// ```
///
/// Expands to:
///
/// ```
/// /// Represents types that are convertible from and into `T`.
/// trait Convertible<T>: From<T> + Into<T> {}
///
/// /// Implementation for all types convertible from and into `T`.
/// impl<T, U> Convertible<T> for U where U: From<T> + Into<T> + ?Sized {}
/// ```
///
/// # Syntax
///
/// Finally, the overall syntax is either `#[trait_alias]` or `#[trait_alias(...)]`, where the
/// arguments can be:
///
/// | `arguments`        | `blanket_type` | `blanket_docs`            |
/// |--------------------|----------------|---------------------------|
/// | `()`               | `__T`          | [*generated*](#generated) |
/// | `(T)`              | `T`            | [*generated*](#generated) |
/// | `(doc = "...")`    | `__T`          | `...`                     |
/// | `(T, doc = "...")` | `T`            | `...`                     |
/// | `(doc = "...", T)` | `T`            | `...`                     |
///
/// [^trait_alias]: [`trait_alias`](https://rust-lang.github.io/rfcs/1733-trait-alias.html) RFC.
#[proc_macro_attribute]
pub fn trait_alias(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let optional = parse_macro_input!(tokens as Optional);
    let alias = parse_macro_input!(item as ItemTraitAlias);

    let input = TraitAliasInput::new(optional, &alias);

    generate::trait_alias(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
