use crate::element_attributes::ElementAttributes;
use proc_macro_error::abort;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub struct FallbackAttributes(pub syn::Block);
pub struct OpenTag {
    pub name: syn::Path,
    pub attributes: ElementAttributes,
    pub self_closing: bool,
    pub fallback_attributes: Option<FallbackAttributes>,
}

fn name_or_fragment(maybe_name: Result<syn::Path>) -> syn::Path {
    maybe_name.unwrap_or_else(|_| syn::parse_str::<syn::Path>("tide_jsx::Fragment").unwrap())
}

fn is_custom_element_name(path: &syn::Path) -> bool {
    match path.get_ident() {
        None => true,
        Some(ident) => {
            let name = ident.to_string();
            let first_letter = name.get(0..1).unwrap();
            first_letter.to_uppercase() == first_letter
        }
    }
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        let maybe_name = syn::Path::parse_mod_style(input);
        let name = name_or_fragment(maybe_name);
        let is_custom_element = is_custom_element_name(&name);
        let attributes = ElementAttributes::parse(input, is_custom_element)?;

        let fallback = if input.peek(syn::token::Brace) {
            let block = input.parse::<syn::Block>()?;
            if !is_custom_element {
                abort!(
                    block.span(),
                    "Only custom components can have fallback attributes",
                );
            }
            Some(FallbackAttributes(block))
        } else {
            None
        };

        if !is_custom_element && fallback.is_some() {}

        let self_closing = input.parse::<syn::Token![/]>().is_ok();
        input.parse::<syn::Token![>]>()?;

        Ok(Self {
            name,
            attributes,
            self_closing,
            fallback_attributes: fallback,
        })
    }
}

pub struct ClosingTag {
    name: syn::Path,
}

impl ClosingTag {
    pub fn validate(&self, open_tag: &OpenTag) {
        let open_tag_path = &open_tag.name;
        let open_tag_path_str = quote!(#open_tag_path).to_string();
        let self_path = &self.name;
        let self_path_str = quote!(#self_path).to_string();
        if self_path_str != open_tag_path_str {
            abort!(
                self.name.span(),
                "Expected closing tag for: <{}>",
                &open_tag_path_str
            );
        }
    }
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        let maybe_name = input.parse::<syn::Path>();
        input.parse::<syn::Token![>]>()?;
        Ok(Self {
            name: name_or_fragment(maybe_name),
        })
    }
}
