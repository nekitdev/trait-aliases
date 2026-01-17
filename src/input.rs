use syn::{Attribute, ItemTraitAlias, Result};

use crate::{
    arguments::{Arguments, Optional, is_trait_alias_attribute},
    context::Context,
    parse::TraitAliases,
};

pub const NO_ERRORS: &str = "failed to extract input but no errors reported";

pub fn extract_attributes(attributes: &mut Vec<Attribute>) -> Vec<Attribute> {
    attributes
        .extract_if(.., |attribute| is_trait_alias_attribute(attribute))
        .collect()
}

pub struct TraitAliasInput<'i> {
    pub optional: Optional,
    pub item: &'i ItemTraitAlias,
}

impl<'i> TraitAliasInput<'i> {
    pub const fn new(optional: Optional, item: &'i ItemTraitAlias) -> Self {
        Self { optional, item }
    }

    pub fn with<'a, A: IntoIterator<Item = &'a Attribute>>(
        context: &Context,
        attributes: A,
        item: &'i ItemTraitAlias,
    ) -> Option<Self> {
        let optional = Optional::from_attributes(context, attributes)?;

        let input = Self::new(optional, item);

        Some(input)
    }
}

pub struct TraitAliasesInput<'i> {
    pub inputs: Vec<TraitAliasInput<'i>>,
}

impl<'i> TraitAliasesInput<'i> {
    pub const fn new(inputs: Vec<TraitAliasInput<'i>>) -> Self {
        Self { inputs }
    }

    pub fn with(context: &Context, aliases: &'i mut TraitAliases) -> Option<Self> {
        let option: Option<Vec<TraitAliasInput<'i>>> = aliases
            .items
            .iter_mut()
            .map(|item| {
                let attributes = extract_attributes(&mut item.attrs);

                TraitAliasInput::with(context, attributes.iter(), item)
            })
            .collect();

        let inputs = option?;

        let input = Self::new(inputs);

        Some(input)
    }

    pub fn extract(aliases: &'i mut TraitAliases) -> Result<Self> {
        let context = Context::new();

        let Some(input) = Self::with(&context, aliases) else {
            return Err(context.check().expect_err(NO_ERRORS));
        };

        context.check()?;

        Ok(input)
    }
}

pub struct TraitAlias<'a> {
    pub arguments: Arguments,
    pub item: &'a ItemTraitAlias,
}

impl<'a> TraitAlias<'a> {
    pub const fn new(arguments: Arguments, item: &'a ItemTraitAlias) -> Self {
        Self { arguments, item }
    }

    pub fn from_input(input: TraitAliasInput<'a>) -> Self {
        let TraitAliasInput { optional, item } = input;

        let arguments = Arguments::from_optional(optional, &item.ident);

        Self::new(arguments, item)
    }
}

impl<'a> From<TraitAliasInput<'a>> for TraitAlias<'a> {
    fn from(input: TraitAliasInput<'a>) -> Self {
        Self::from_input(input)
    }
}
