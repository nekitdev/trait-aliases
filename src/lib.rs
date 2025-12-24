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
//! trait SSS: Send + Sync + 'static {}
//!
//! /// Blanket implementation of [`SSS`] for all types satisfying its bounds.
//! impl<__T> SSS for __T where __T: Send + Sync + 'static + ?Sized {}
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
//!     pub trait NumOps<R = Self, T = Self> =
//!         Add<R, Output = T>
//!         + Sub<R, Output = T>
//!         + Mul<R, Output = T>
//!         + Div<R, Output = T>
//!         + Rem<R, Output = T>;
//!
//!     /// Represents types implementing numeric assignment operations.
//!     pub trait NumAssignOps<R = Self> =
//!         AddAssign<R> + SubAssign<R> + MulAssign<R> + DivAssign<R> + RemAssign<R>;
//!
//!     /// Represents numeric types that have `0` and `1` values, can be compared for equality
//!     /// and operated on.
//!     pub trait Num = PartialEq + Zero + One + NumOps;
//!
//!     /// Represents [`Num`] types which also implement assignment operations.
//!     pub trait NumAssign = Num + NumAssignOps;
//!
//!     /// Represents [`Num`] types which also implement numeric operations taking
//!     /// the right-hand side operand by reference.
//!     pub trait NumRef = Num + for<'r> NumOps<&'r Self>;
//!
//!     /// Represents [`NumAssign`] types which also implement numeric assignment by reference.
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
//! So, for instance, using `#[cfg]` attributes:
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
//!     pub trait BaseId = Copy + Ord + Hash;
//!
//!     /// Represents types that can be serialized and deserialized.
//!     #[cfg(feature = "serde")]
//!     pub trait Serializable = Serialize + for<'de> Deserialize<'de>;
//!
//!     /// Represents identifier types.
//!     #[cfg(feature = "serde")]
//!     pub trait Id = BaseId + Serializable;
//!
//!     /// Represents identifier types.
//!     #[cfg(not(feature = "serde"))]
//!     pub trait Id = BaseId;
//! }
//! ```
//!
//! Which will generate the following code with `serde` enabled:
//!
//! ```ignore
//! use core::hash::Hash;
//!
//! use serde::{Deserialize, Serialize};
//!
//! /// Represents base identifier bounds.
//! trait BaseId: Copy + Ord + Hash {}
//!
//! /// Blanket implementation of [`BaseId`] for all types satisfying its bounds.
//! impl<__T> BaseId for __T where __T: Copy + Ord + Hash + ?Sized {}
//!
//! /// Represents types that can be serialized and deserialized.
//! trait Serializable: Serialize + for<'de> Deserialize<'de> {}
//!
//! /// Blanket implementation of [`Serializable`] for all types satisfying its bounds.
//! impl<__T> Serializable for __T where __T: Serialize + for<'de> Deserialize<'de> + ?Sized {}
//!
//! /// Represents identifier types.
//! trait Id: BaseId + Serializable {}
//!
//! /// Blanket implementation of [`Id`] for all types satisfying its bounds.
//! impl<__T> Id for __T where __T: BaseId + Serializable + ?Sized {}
//! ```
//!
//! And without it:
//!
//! ```
//! use core::hash::Hash;
//!
//! /// Represents base identifier bounds.
//! trait BaseId: Copy + Ord + Hash {}
//!
//! /// Blanket implementation of [`BaseId`] for all types satisfying its bounds.
//! impl<__T> BaseId for __T where __T: Copy + Ord + Hash + ?Sized {}
//!
//! /// Represents identifier types.
//! trait Id: BaseId {}
//!
//! /// Blanket implementation of [`Id`] for all types satisfying its bounds.
//! impl<__T> Id for __T where __T: BaseId + ?Sized {}
//! ```
//!
//! # Note
//!
//! Please *never* use `__T` in your generic parameters, as it is reserved for the
//! blanket implementation.
//!
//! Failing to do so will result in collisions at best, and hard-to-debug errors,
//! migraines or even spontaneous combustion at worst.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod parse;

/// Defines trait aliases with blanket implementations.
///
/// See the [crate-level] documentation for more details.
///
/// [crate-level]: crate
#[proc_macro]
pub fn trait_aliases(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::TraitAliases);

    generate::trait_aliases(&input).into()
}
