use quote::quote;
use std::hash::{Hash, Hasher};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub type AttributeKey = syn::punctuated::Punctuated<syn::Ident, syn::Token![-]>;

pub enum ElementAttribute {
    Punned(AttributeKey),
    WithValueLit(AttributeKey, syn::LitStr),
    WithValue(AttributeKey, syn::Block),
    WithValueOpt(AttributeKey, syn::Block),
    WithValueBool(AttributeKey, syn::Block),
}

impl ElementAttribute {
    pub fn ident(&self) -> &AttributeKey {
        match self {
            Self::Punned(ident)
            | Self::WithValue(ident, _)
            | Self::WithValueOpt(ident, _)
            | Self::WithValueLit(ident, _)
            | Self::WithValueBool(ident, _) => ident,
        }
    }

    pub fn idents(&self) -> Vec<&syn::Ident> {
        self.ident().iter().collect::<Vec<_>>()
    }

    pub fn value_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::WithValue(_, value)
            | Self::WithValueOpt(_, value)
            | Self::WithValueBool(_, value) => {
                if value.stmts.len() == 1 {
                    let first = &value.stmts[0];
                    quote!(#first)
                } else {
                    quote!(#value)
                }
            }
            Self::WithValueLit(_, value) => {
                quote!(#value)
            }
            Self::Punned(ident) => quote!(#ident),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::WithValueOpt(_, _) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::WithValueBool(_, _) => true,
            _ => false,
        }
    }

    pub fn validate(self, is_custom_element: bool) -> Result<Self> {
        if is_custom_element {
            self.validate_for_custom_element()
        } else {
            self.validate_for_simple_element()
        }
    }

    pub fn validate_for_custom_element(self) -> Result<Self> {
        if self.is_optional() {
            let error_message =
                "Cannot use optional value syntax on custom components. Try to remove `?`";
            return Err(syn::Error::new(self.ident().span(), error_message));
        }

        if self.is_boolean() {
            let error_message =
                "Cannot use boolean value syntax on custom components. Try to remove `!`";
            return Err(syn::Error::new(self.ident().span(), error_message));
        }


        if self.idents().len() < 2 {
            Ok(self)
        } else {
            let alternative_name = self
                .idents()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("_");

            let error_message = format!(
                "Can't use dash-delimited values on custom components. Did you mean `{}`?",
                alternative_name
            );

            Err(syn::Error::new(self.ident().span(), error_message))
        }
    }

    pub fn validate_for_simple_element(self) -> Result<Self> {
        match (&self, self.idents().len()) {
            (Self::Punned(ref key), len) if len > 1 => {
                let error_message = "Can't use punning with dash-delimited values";
                Err(syn::Error::new(key.span(), error_message))
            }
            _ => Ok(self),
        }
    }
}

impl PartialEq for ElementAttribute {
    fn eq(&self, other: &Self) -> bool {
        let self_idents: Vec<_> = self.ident().iter().collect();
        let other_idents: Vec<_> = other.ident().iter().collect();
        self_idents == other_idents
    }
}

impl Eq for ElementAttribute {}

impl Hash for ElementAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ident = self.idents();
        Hash::hash(&ident, state)
    }
}

impl Parse for ElementAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = AttributeKey::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        let not_punned = input.peek(syn::Token![=]);

        if !not_punned {
            return Ok(Self::Punned(name));
        }

        input.parse::<syn::Token![=]>()?;
        let value = if input.peek(syn::token::Brace) {
            input.parse::<syn::Block>()?
        } else {
            let v = input.parse::<syn::LitStr>()?;
            return Ok(Self::WithValueLit(name, v));
        };

        if input.peek(syn::Token![?]) {
            input.parse::<syn::Token![?]>().unwrap();
            Ok(Self::WithValueOpt(name, value))
        } else if input.peek(syn::Token![!])  {
            input.parse::<syn::Token![!]>().unwrap();
            Ok(Self::WithValueBool(name, value))
        }
        else {
            Ok(Self::WithValue(name, value))
        }
    }
}
