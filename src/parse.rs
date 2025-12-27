//! Parsing functionality.

use syn::{
    ItemTraitAlias, Result,
    parse::{Parse, ParseStream},
    visit::Visit,
};

use crate::special::Checker;

/// Represents the input to [`trait_aliases!`].
///
/// [`trait_aliases!`]: crate::trait_aliases
pub struct TraitAliases {
    /// The trait alias items.
    pub items: Vec<ItemTraitAlias>,
}

impl TraitAliases {
    /// Constructs [`Self`].
    pub const fn new(items: Vec<ItemTraitAlias>) -> Self {
        Self { items }
    }
}

impl Parse for TraitAliases {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut checker = Checker::new();

        let mut items = Vec::new();

        while !input.is_empty() {
            // fail immediately on syntax errors
            let item = input.parse()?;

            // visit the parsed trait alias, but check for errors only after parsing
            checker.visit_item_trait_alias(&item);

            items.push(item);
        }

        checker.check()?;

        let trait_aliases = Self::new(items);

        Ok(trait_aliases)
    }
}
