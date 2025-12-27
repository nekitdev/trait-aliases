//! Functionality related to the special `__T` identifier.

use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    Error, Ident, Path, PathArguments, PathSegment, PredicateType, Result, TraitBound,
    TraitBoundModifier, Type, TypeParam, TypeParamBound, TypePath,
    punctuated::Punctuated,
    token::{Colon, Plus, Question},
    visit::{Visit, visit_ident},
};

/// Represents checkers which traverse syntax trees searching for `__T` identifiers.
pub struct Checker {
    error: Option<Error>,
}

impl Checker {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self { error: None }
    }

    /// Constructs new [`Error`] spanned by the given tokens, then adds it to [`Self`].
    pub fn error<T: ToTokens>(&mut self, tokens: T) {
        let error = Error::new_spanned(tokens, MESSAGE);

        if let Some(ref mut combined) = self.error {
            combined.combine(error);
        } else {
            self.error = Some(error);
        }
    }

    /// Returns the (combined) error, if one was found.
    pub fn check_error(self) -> Option<Error> {
        self.error
    }

    /// Similar to [`check_error`], except [`Result`] is returned.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if `__T` was found in the syntax tree.
    ///
    /// [`check_error`]: Self::check_error
    pub fn check(self) -> Result<()> {
        self.check_error().map_or(Ok(()), Err)
    }
}

impl<'a> Visit<'a> for Checker {
    fn visit_ident(&mut self, identifier: &'a Ident) {
        if identifier == SPECIAL {
            self.error(identifier);
        }

        visit_ident(self, identifier);
    }
}

/// Literal [`Sized`] string.
pub const SIZED: &str = stringify!(Sized);

/// Literal `__T` string, used for blanket implementations.
pub const SPECIAL: &str = stringify!(__T);

/// The message used for errors on `__T` detected when parsing.
pub const MESSAGE: &str = "identifier `__T` is reserved for blanket implementations";

/// Returns the special `__T` identifier.
pub fn identifier() -> Ident {
    Ident::new(SPECIAL, Span::call_site())
}

/// Returns the [`Sized`] identifier.
pub fn sized_identifier() -> Ident {
    Ident::new(SIZED, Span::call_site())
}

/// Constructs [`Path`] for the given identifier.
pub fn path_for(identifier: Ident) -> Path {
    let mut segments = Punctuated::new();

    segments.push(PathSegment {
        ident: identifier,
        arguments: PathArguments::None,
    });

    Path {
        leading_colon: None,
        segments,
    }
}

/// Constructs [`Path`] for `__T`.
pub fn path() -> Path {
    path_for(identifier())
}

/// Constructs [`Path`] for [`Sized`].
pub fn sized_path() -> Path {
    path_for(sized_identifier())
}

/// Constructs [`TypePath`] for `__T`.
pub fn type_path() -> TypePath {
    TypePath {
        qself: None,
        path: path(),
    }
}

/// Represents type parameter bounds.
pub type Bounds = Punctuated<TypeParamBound, Plus>;

/// Constructs type parameter for `__T`.
pub fn type_parameter() -> TypeParam {
    TypeParam {
        // no attributes
        attrs: Vec::new(),
        ident: identifier(),
        // bounds are added in `where` predicate
        colon_token: None,
        bounds: Punctuated::new(),
        // no defaults
        eq_token: None,
        default: None,
    }
}

/// Constructs [`?Sized`] bound.
///
/// [`?Sized`]: Sized
pub fn maybe_sized_bound() -> TraitBound {
    TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::Maybe(Question::default()),
        lifetimes: None,
        path: sized_path(),
    }
}

/// Constructs predicate type for `__T`, constraining it with the provided bounds.
///
/// The [`?Sized`] bound is added in the process.
///
/// [`?Sized`]: Sized
pub fn predicate_type(mut bounds: Bounds) -> PredicateType {
    bounds.push(TypeParamBound::Trait(maybe_sized_bound()));

    PredicateType {
        lifetimes: None,
        bounded_ty: Type::Path(type_path()),
        colon_token: Colon::default(),
        bounds,
    }
}
