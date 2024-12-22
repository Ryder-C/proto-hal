use darling::FromMeta;
use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute};

pub mod block;
pub mod field;
pub mod field_array;
pub mod register;
pub mod schema;
pub mod state;

pub trait Args: FromMeta + Sized {
    const NAME: &str;

    fn attach_span(self, span: Span) -> Self;

    fn get<'a>(attrs: impl Iterator<Item = &'a Attribute>) -> syn::Result<Option<Self>> {
        let mut found = None;

        for attr in attrs {
            if attr.path().is_ident(Self::NAME) {
                if found.is_none() {
                    found.replace(Self::from_meta(&attr.meta)?.attach_span(attr.span()));
                } else {
                    Err(syn::Error::new_spanned(
                        attr,
                        format!(
                            "the '{}' attribute can only be used once per item",
                            Self::NAME
                        ),
                    ))?
                }
            }
        }

        Ok(found)
    }
}
