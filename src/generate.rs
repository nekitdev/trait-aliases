//! Code generation functionality.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, GenericParam, Ident, ItemTraitAlias, WherePredicate};

use crate::{
    parse::TraitAliases,
    special::{predicate_type, type_parameter},
};

/// Generates trait aliases for the input.
///
/// This function calls [`trait_alias`] for each item in [`TraitAliases`] and combines the outputs.
pub fn trait_aliases(input: &TraitAliases) -> TokenStream {
    let aliases = input.items.iter().map(trait_alias);

    quote! {
        #(#aliases)*
    }
}

/// The literal `doc` identifier.
pub const DOC: &str = stringify!(doc);

/// Checks whether the given attribute is [`DOC`].
pub fn is_doc_attribute(attribute: &Attribute) -> bool {
    attribute.meta.path().is_ident(DOC)
}

/// Returns the blanket implementation documentation.
pub fn blanket_impl_doc(name: &Ident) -> String {
    format!("Blanket implementation of [`{name}`] for all types satisfying its bounds.")
}

/// Generates the trait definition and blanket implementation for the given trait alias.
pub fn trait_alias(alias: &ItemTraitAlias) -> TokenStream {
    let (docs, attributes): (Vec<_>, Vec<_>) =
        alias.attrs.iter().cloned().partition(is_doc_attribute);

    let visibility = &alias.vis;
    let name = &alias.ident;

    let generics = &alias.generics;

    let bounds = &alias.bounds;

    // `generics` are used for the trait definition, so skip `impl_generics`
    let (_, type_generics, where_clause) = generics.split_for_impl();

    let mut derived = generics.clone();

    // add `__T` parameter
    derived.params.push(GenericParam::Type(type_parameter()));

    // and restrict it with `bounds`, adding `?Sized`
    derived
        .make_where_clause()
        .predicates
        .push(WherePredicate::Type(predicate_type(bounds.clone())));

    // `type_generics` are reused, so skip them here
    let (impl_generics, _, where_derived) = derived.split_for_impl();

    let blanket_impl = blanket_impl_doc(name);

    // output trait definition, then its blanket implementation
    quote! {
        #(#docs)*
        #(#attributes)*
        #visibility trait #name #generics: #bounds #where_clause {}

        #[doc = #blanket_impl]
        #(#attributes)*
        impl #impl_generics #name #type_generics for __T #where_derived {}
    }
}
