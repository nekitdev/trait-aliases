use syn::{
    ItemTraitAlias, Result,
    parse::{Parse, ParseStream},
};

pub struct TraitAliases {
    pub items: Vec<ItemTraitAlias>,
}

impl TraitAliases {
    pub const fn new(items: Vec<ItemTraitAlias>) -> Self {
        Self { items }
    }
}

impl Parse for TraitAliases {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            let item = input.parse()?;

            items.push(item);
        }

        let trait_aliases = Self::new(items);

        Ok(trait_aliases)
    }
}
