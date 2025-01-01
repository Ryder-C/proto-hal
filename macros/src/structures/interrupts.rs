use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::{parse2, spanned::Spanned, Attribute, Ident, Index, ItemEnum, Visibility};

use crate::utils::SynErrorCombinator;

struct Vector {
    attrs: Vec<Attribute>,
    ident: Ident,
    position: Index,
}

impl ToTokens for Vector {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let ident = &self.ident;
        let position = &self.position;

        tokens.extend(quote! {
            #(
                #attrs
            )*
            #ident = #position
        });
    }
}

pub struct InterruptsSpec {
    attrs: Vec<Attribute>,
    vectors: HashMap<u32, Vector>,
}

impl InterruptsSpec {
    pub fn parse(e: &ItemEnum) -> syn::Result<Self> {
        let mut errors = SynErrorCombinator::new();

        if !matches!(e.vis, Visibility::Public(_)) {
            errors.push(syn::Error::new(
                e.ident.span(),
                "interrupt table enum must be public",
            ));
        }

        let mut interrupts = Self {
            attrs: e.attrs.clone(),
            vectors: HashMap::new(),
        };

        let mut position = 0;

        if e.variants.is_empty() {
            Err(syn::Error::new_spanned(
                e,
                "interrupt table must be of non-zero size",
            ))?
        }

        for variant in &e.variants {
            errors.maybe(|| {
                if !variant.fields.is_empty() {
                    Err(syn::Error::new(
                        variant.fields.span(),
                        "interrupt table entries must be unit variants",
                    ))?
                }

                if let Some((_, discriminant)) = &variant.discriminant {
                    let i = parse2::<Index>(discriminant.to_token_stream())?;
                    position = i.index;
                }

                interrupts.vectors.insert(
                    position,
                    Vector {
                        attrs: variant.attrs.clone(),
                        ident: variant.ident.clone(),
                        position: Index::from(position as usize),
                    },
                );

                position += 1;

                Ok(())
            });
        }

        Ok(interrupts)
    }
}

impl ToTokens for InterruptsSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let vectors = self.vectors.values();

        let enum_ = quote! {
            #(
                #attrs
            )*
            #[allow(non_camel_case_types)]
            pub enum interrupt {
                #(
                    #vectors,
                )*
            }
        };

        let vector_idents = self
            .vectors
            .values()
            .map(|vector| &vector.ident)
            .collect::<Vec<_>>();

        let vector_ident_strings = vector_idents.iter().map(|ident| ident.to_string());

        let functions = quote! {
            extern "C" {
                #(
                    fn #vector_idents();
                )*
            }
        };

        let table_length = (self.vectors.keys().max().unwrap() + 1) as usize;
        let table_entries = (0..table_length as u32).into_iter().map(|position| {
            if let Some(vector) = self.vectors.get(&position) {
                let ident = &vector.ident;
                quote! {
                    ::proto_hal::interrupt::Vector::handler(#ident)
                }
            } else {
                quote! {
                    ::proto_hal::interrupt::Vector::reserved()
                }
            }
        });

        let table = quote! {
            #[doc(hidden)]
            #[cfg_attr(target_arch = "arm", link_section = ".vector_table.interrupts")]
            #[no_mangle]
            pub static __INTERRUPTS: [::proto_hal::interrupt::Vector; #table_length] = [
                #(
                    #table_entries,
                )*
            ];
        };

        let build_export = quote! {
            pub static INTERRUPT_IDENTS: &[&str] = &[
                #(
                    #vector_ident_strings,
                )*
            ];
        };

        tokens.extend(quote! {
            pub use ::cortex_m_rt::interrupt;
            #enum_
            #functions
            #table

            #build_export
        });
    }
}
