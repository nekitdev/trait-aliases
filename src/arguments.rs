use proc_macro2::Span;
use syn::{
    Attribute, Error, Expr, ExprLit, Ident, Lit, LitStr, Meta, MetaNameValue, Result,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Comma, Eq},
};

use crate::{
    at_most_one::AtMostOne,
    context::{Context, OrReport},
    name::Name,
};

struct Documentation {
    expression: Expr,
}

impl Documentation {
    const fn new(expression: Expr) -> Self {
        Self { expression }
    }

    fn peek(input: ParseStream<'_>) -> bool {
        input.peek(Ident) && input.peek2(Eq)
    }
}

impl Parse for Documentation {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let identifier: Ident = input.parse()?;

        let doc = Name::DOC;

        if identifier != doc {
            let message = format!("expected `{doc}` identifier");

            return Err(Error::new_spanned(identifier, message));
        }

        // ensure the equality sign is present, ignoring it
        let _: Eq = input.parse()?;

        let expression = input.parse()?;

        let documentation = Self::new(expression);

        Ok(documentation)
    }
}

enum Item {
    Documentation(Documentation),
    Identifier(Ident),
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if Documentation::peek(input) {
            let documentation = input.parse()?;

            Ok(Self::Documentation(documentation))
        } else {
            let identifier = input.parse()?;

            Ok(Self::Identifier(identifier))
        }
    }
}

type Items = Punctuated<Item, Comma>;

pub struct Optional {
    pub docs: Vec<Expr>,
    pub type_option: Option<Ident>,
}

pub fn is_trait_alias_attribute(attribute: &Attribute) -> bool {
    Name::TRAIT_ALIAS.match_path(attribute.path())
}

impl Optional {
    pub const fn new() -> Self {
        Self {
            docs: Vec::new(),
            type_option: None,
        }
    }

    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `trait_alias`, nothing to parse
            Meta::Path(_) => Ok(Self::new()),
            // `trait_alias = ...` is not supported
            Meta::NameValue(name_value) => Err(Self::error_name_value(name_value)),
            // `trait_alias(...)`, parse arguments
            Meta::List(list) => list.parse_args(),
        }
    }

    fn from_option(option: Option<&Attribute>) -> Result<Self> {
        let Some(attribute) = option else {
            return Ok(Self::new());
        };

        Self::from_meta(&attribute.meta)
    }

    fn error_name_value(name_value: &MetaNameValue) -> Error {
        let message = format!(
            "expected either #[{trait_alias}] or #[{trait_alias}(...)] syntax",
            trait_alias = Name::TRAIT_ALIAS
        );

        Error::new_spanned(name_value.eq_token, message)
    }

    fn report_repeated<'a, A: IntoIterator<Item = &'a Attribute>>(
        context: &Context,
        attributes: A,
    ) {
        let message = format!(
            "repeated `{trait_alias}` attribute",
            trait_alias = Name::TRAIT_ALIAS
        );

        attributes.into_iter().for_each(|attribute| {
            context.error_spanned_by(attribute, message.as_str());
        });
    }

    pub fn from_attributes<'a, A: IntoIterator<Item = &'a Attribute>>(
        context: &Context,
        attributes: A,
    ) -> Option<Self> {
        let filtered = attributes
            .into_iter()
            .filter(|attribute| is_trait_alias_attribute(attribute));

        match filtered.at_most_one() {
            Ok(option) => Self::from_option(option).or_report(context),
            Err(repeated) => {
                Self::report_repeated(context, repeated);

                None
            }
        }
    }

    pub fn push_doc(&mut self, doc: Expr) {
        self.docs.push(doc);
    }

    pub fn set_type(&mut self, name: Ident) -> Result<()> {
        let Some(ref set) = self.type_option else {
            self.type_option = Some(name);

            return Ok(());
        };

        let message = format!("duplicate identifier argument `{name}`, already set to `{set}`");

        Err(Error::new_spanned(name, message))
    }

    fn set_item(&mut self, item: Item) -> Result<()> {
        match item {
            Item::Documentation(documentation) => {
                self.push_doc(documentation.expression);

                Ok(())
            }
            Item::Identifier(name) => self.set_type(name),
        }
    }

    fn set_with<I: IntoIterator<Item = Item>>(&mut self, context: &Context, items: I) {
        items.into_iter().for_each(|item| {
            // simply report all errors, without acting on them
            self.set_item(item).or_report(context);
        });
    }
}

impl Parse for Optional {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut optional = Self::new();

        let items = Items::parse_terminated(input)?;

        let context = Context::new();

        optional.set_with(&context, items);

        context.check()?;

        Ok(optional)
    }
}

pub struct Arguments {
    pub blanket_docs: Vec<Expr>,
    pub blanket_type: Ident,
}

pub fn default_blanket_type() -> Ident {
    Name::DEFAULT_TYPE.ident()
}

pub fn default_blanket_doc_string(name: &Ident) -> String {
    format!("Blanket implementation of [`{name}`] for all types satisfying its bounds.")
}

pub fn default_blanket_doc_literal(name: &Ident) -> LitStr {
    let string = default_blanket_doc_string(name);

    LitStr::new(string.as_str(), Span::call_site())
}

pub fn default_blanket_doc_expr(name: &Ident) -> ExprLit {
    let literal = default_blanket_doc_literal(name);

    ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(literal),
    }
}

pub fn default_blanket_doc(name: &Ident) -> Expr {
    Expr::Lit(default_blanket_doc_expr(name))
}

impl Arguments {
    pub fn from_optional(optional: Optional, name: &Ident) -> Self {
        let Optional { docs, type_option } = optional;

        let mut blanket_docs = docs;

        if blanket_docs.is_empty() {
            blanket_docs.push(default_blanket_doc(name));
        }

        let blanket_type = type_option.unwrap_or_else(default_blanket_type);

        Self {
            blanket_docs,
            blanket_type,
        }
    }
}
