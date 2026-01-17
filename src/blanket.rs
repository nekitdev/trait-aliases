use proc_macro2::Span;
use syn::{
    Ident, Path, PathArguments, PathSegment, PredicateType, TraitBound, TraitBoundModifier, Type,
    TypeParam, TypeParamBound, TypePath,
    punctuated::Punctuated,
    token::{Colon, Plus, Question},
    visit::{Visit, visit_ident},
};

use crate::context::Context;

pub struct Checker<'b, 'c> {
    blanket: &'b Ident,
    context: &'c Context,
}

impl<'b, 'c> Checker<'b, 'c> {
    pub const fn new(blanket: &'b Ident, context: &'c Context) -> Self {
        Self { blanket, context }
    }

    pub fn message(&self) -> String {
        format!(
            "identifier `{blanket}` is reserved for blanket implementations",
            blanket = self.blanket
        )
    }

    pub fn visit(&self, identifier: &Ident) {
        if identifier == self.blanket {
            self.context.error_spanned_by(identifier, self.message());
        }
    }
}

impl<'a> Visit<'a> for Checker<'_, '_> {
    fn visit_ident(&mut self, identifier: &'a Ident) {
        self.visit(identifier);

        visit_ident(self, identifier);
    }
}

pub const SIZED: &str = stringify!(Sized);

pub fn sized_identifier() -> Ident {
    Ident::new(SIZED, Span::call_site())
}

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

pub fn sized_path() -> Path {
    path_for(sized_identifier())
}

pub const fn type_path(path: Path) -> TypePath {
    TypePath { qself: None, path }
}

pub type Bounds = Punctuated<TypeParamBound, Plus>;

pub const fn type_parameter(identifier: Ident) -> TypeParam {
    TypeParam {
        // no attributes
        attrs: Vec::new(),
        ident: identifier,
        // bounds are added in `where` predicate
        colon_token: None,
        bounds: Punctuated::new(),
        // no defaults
        eq_token: None,
        default: None,
    }
}

pub fn maybe_sized_bound() -> TraitBound {
    TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::Maybe(Question::default()),
        lifetimes: None,
        path: sized_path(),
    }
}

pub fn predicate_type(identifier: Ident, mut bounds: Bounds) -> PredicateType {
    bounds.push(TypeParamBound::Trait(maybe_sized_bound()));

    let path = path_for(identifier);

    PredicateType {
        lifetimes: None,
        bounded_ty: Type::Path(type_path(path)),
        colon_token: Colon::default(),
        bounds,
    }
}
