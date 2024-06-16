use crate::children::Children;
use crate::element_attribute::ElementAttribute;
use crate::tags::FallbackAttributes;
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

pub type Attributes = HashSet<ElementAttribute>;

#[derive(Default)]
pub struct ElementAttributes {
    pub attributes: Attributes,
}

impl ElementAttributes {
    pub fn new(attributes: Attributes) -> Self {
        Self { attributes }
    }

    pub fn for_custom_element<'f, 'c>(
        &self,
        fallback_attributes: Option<&'f FallbackAttributes>,
        children: &'c Children,
    ) -> CustomElementAttributes<'_, 'f, 'c> {
        CustomElementAttributes {
            attributes: &self.attributes,
            fallback_attributes,
            children,
        }
    }

    pub fn for_simple_element(&self) -> SimpleElementAttributes<'_> {
        SimpleElementAttributes {
            attributes: &self.attributes,
        }
    }

    pub fn parse(input: ParseStream, is_custom_element: bool) -> Result<Self> {
        let mut parsed_self = input.parse::<Self>()?;

        let new_attributes: Attributes = parsed_self
            .attributes
            .drain()
            .filter_map(|attribute| match attribute.validate(is_custom_element) {
                Ok(x) => Some(x),
                Err(err) => {
                    emit_error!(err.span(), "Invalid attribute: {}", err);
                    None
                }
            })
            .collect();

        Ok(ElementAttributes::new(new_attributes))
    }
}

impl Parse for ElementAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes: HashSet<ElementAttribute> = HashSet::new();
        while input.peek(syn::Ident::peek_any) {
            let attribute = input.parse::<ElementAttribute>()?;
            let ident = attribute.ident();
            if attributes.contains(&attribute) {
                emit_error!(
                    ident.span(),
                    "There is a previous definition of the {} attribute",
                    quote!(#ident)
                );
            }
            attributes.insert(attribute);
        }
        Ok(ElementAttributes::new(attributes))
    }
}

pub struct CustomElementAttributes<'a, 'f, 'c> {
    attributes: &'a Attributes,
    fallback_attributes: Option<&'f FallbackAttributes>,
    children: &'c Children,
}

impl<'a, 'f, 'c> ToTokens for CustomElementAttributes<'a,'f, 'c> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut attrs: Vec<_> = self
            .attributes
            .iter()
            .map(|attribute| {
                let ident = attribute.ident();
                let value = attribute.value_tokens();

                quote! {
                    #ident: #value
                }
            })
            .collect();

        if self.children.len() > 0 {
            let children_tuple = self.children.as_option_of_tuples_tokens();
            attrs.push(quote! {
                children: #children_tuple
            });
        }

        let quoted = if attrs.len() == 0 && !self.fallback_attributes.is_some() {
            quote!()
        } else if let Some(FallbackAttributes(block)) = self.fallback_attributes{
            let inner = &block.stmts[0];
            if attrs.len() > 0 {
                quote!({ #(#attrs),*,  #inner})
            }
            else {
                quote!({ #inner})
            }
        }
        else {
            quote!({ #(#attrs),* })
        }
        ;


        quoted.to_tokens(tokens);
    }
}

pub struct SimpleElementAttributes<'a> {
    attributes: &'a Attributes,
}

impl<'a> ToTokens for SimpleElementAttributes<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.attributes.is_empty() {
            quote!(None).to_tokens(tokens);
        } else {
            let attrs: Vec<_> = self
                .attributes
                .iter()
                .map(|attribute| {
                    let mut iter = attribute.ident().iter();
                    let first_word = iter.next().unwrap().unraw();
                    let ident = iter.fold(first_word.to_string(), |acc, curr| {
                        format!("{}-{}", acc, curr.unraw())
                    });
                    let value = attribute.value_tokens();

                    if attribute.is_optional() {
                        let mut iter = attribute.ident().iter();
                        let first_word = iter.next().unwrap().unraw();
                        let ident_underscored = iter.fold(first_word.to_string(), |acc, curr| {
                            format!("{}_{}", acc, curr.unraw())
                        });
                        let attr_ident = syn::Ident::new(&ident_underscored, attribute.ident().span());

                        quote! {
                            if let ::std::option::Option::Some(#attr_ident) = #value {
                                hm.insert(#ident, ::std::borrow::Cow::from(#attr_ident));
                            }
                        }
                    }
                    else if attribute.is_boolean() {
                        quote! {
                            if #value {
                                hm.insert(#ident, std::borrow::Cow::from(""));
                            }
                        }
                    }
                    else {
                        quote! {
                            hm.insert(#ident, ::std::borrow::Cow::from(#value));
                        }
                    }
                })
                .collect();

            let hashmap_declaration = quote! {{
                let mut hm = std::collections::HashMap::<&str, ::std::borrow::Cow<'_, str>>::new();
                #(#attrs)*
                Some(hm)
            }};

            hashmap_declaration.to_tokens(tokens);
        }
    }
}
