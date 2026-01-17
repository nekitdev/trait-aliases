use std::fmt;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Path};

pub type StaticStr = &'static str;

#[derive(Clone, Copy)]
pub struct Name {
    string: StaticStr,
}

impl Name {
    pub const fn new(string: StaticStr) -> Self {
        Self { string }
    }

    pub const fn get(self) -> StaticStr {
        self.string
    }

    pub const DOC: Self = Self::new(stringify!(doc));

    pub const TRAIT_ALIAS: Self = Self::new(stringify!(trait_alias));

    pub const DEFAULT_TYPE: Self = Self::new(stringify!(__T));

    pub fn ident(self) -> Ident {
        Ident::new(self.get(), Span::call_site())
    }

    pub fn match_path(self, path: &Path) -> bool {
        path.is_ident(self.get())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.ident());
    }
}
