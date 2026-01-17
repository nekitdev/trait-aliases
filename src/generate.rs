use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, GenericParam, ItemTraitAlias, Result, WherePredicate, visit::Visit};

use crate::{
    arguments::Arguments,
    blanket::{Checker, predicate_type, type_parameter},
    context::Context,
    input::{TraitAlias, TraitAliasInput, TraitAliasesInput},
    name::Name,
    parse::TraitAliases,
};

pub fn is_doc_attribute(attribute: &Attribute) -> bool {
    Name::DOC.match_path(attribute.path())
}

pub fn split_attributes<'a, A: IntoIterator<Item = &'a Attribute>>(
    attributes: A,
) -> (Vec<&'a Attribute>, Vec<&'a Attribute>) {
    attributes
        .into_iter()
        .partition(|attribute| is_doc_attribute(attribute))
}

pub fn extract_trait_aliases(aliases: &mut TraitAliases) -> Result<TokenStream> {
    let input = TraitAliasesInput::extract(aliases)?;

    trait_aliases(input)
}

pub fn trait_aliases(input: TraitAliasesInput<'_>) -> Result<TokenStream> {
    let context = Context::new();

    let output = trait_aliases_with(&context, input);

    context.check()?;

    Ok(output)
}

pub fn trait_aliases_with(context: &Context, input: TraitAliasesInput<'_>) -> TokenStream {
    let generated = input
        .inputs
        .into_iter()
        .map(|input| trait_alias_with(context, input));

    let output = quote! {
        #(#generated)*
    };

    output
}

pub fn trait_alias(input: TraitAliasInput<'_>) -> Result<TokenStream> {
    let context = Context::new();

    let output = trait_alias_with(&context, input);

    context.check()?;

    Ok(output)
}

pub fn trait_alias_with(context: &Context, input: TraitAliasInput<'_>) -> TokenStream {
    let TraitAlias { arguments, item } = TraitAlias::from_input(input);

    let Arguments {
        blanket_docs,
        blanket_type,
    } = arguments;

    let ItemTraitAlias {
        attrs,
        vis: visibility,
        ident: name,
        generics,
        bounds,
        .. // skip over tokens
    } = item;

    let (docs, attributes) = split_attributes(attrs);

    // check the trait alias for occurrences of the `blanket_type` identifier
    let mut checker = Checker::new(&blanket_type, context);

    checker.visit_item_trait_alias(item);

    // `generics` are used for the trait definition, so skip `impl_generics`
    let (_, type_generics, where_clause) = generics.split_for_impl();

    // clone generics in order to add the blanket type parameter to them
    let mut derived = generics.clone();

    let blanket = type_parameter(blanket_type.clone());

    derived.params.push(GenericParam::Type(blanket));

    let predicate = predicate_type(blanket_type.clone(), bounds.clone());

    derived
        .make_where_clause()
        .predicates
        .push(WherePredicate::Type(predicate));

    // `type_generics` are reused, so skip them here
    let (impl_generics, _, where_derived) = derived.split_for_impl();

    // output trait definition, then its blanket implementation
    quote! {
        #(#docs)*
        #(#attributes)*
        #visibility trait #name #generics: #bounds #where_clause {}

        #(#[doc = #blanket_docs])*
        #(#attributes)*
        impl #impl_generics #name #type_generics for #blanket_type #where_derived {}
    }
}
