use std::collections::HashSet;

use darling::FromMeta;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Ident, Path};

use crate::utils::{PathArray, Spanned};

use super::Args;

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct StateArgs {
    #[darling(default)]
    pub bits: Option<u32>,
    pub entitlements: PathArray,

    #[darling(skip)]
    pub span: Option<Span>,
}

impl Args for StateArgs {
    const NAME: &str = "state";
}

#[derive(Debug, Clone)]
pub struct State {
    pub args: Spanned<StateArgs>,
    pub ident: Ident,
    pub bits: u32,
    pub entitlements: HashSet<Path>,
    pub entitlement_fields: HashSet<Ident>,
}

impl State {
    pub fn parse(ident: Ident, bits: u32, state_args: Spanned<StateArgs>) -> syn::Result<Self> {
        let bits = state_args.bits.unwrap_or(bits);
        let mut entitlements = HashSet::new();
        let mut entitlement_fields = HashSet::new();

        for entitlement in state_args.entitlements.elems.iter().cloned() {
            entitlement_fields.insert(
                entitlement
                    .segments
                    .iter()
                    .nth_back(1)
                    .unwrap()
                    .ident
                    .clone(),
            );

            if !entitlements.insert(entitlement.clone()) {
                Err(syn::Error::new_spanned(
                    entitlement,
                    "entitlement already exists",
                ))?
            }
        }

        Ok(Self {
            args: state_args,
            ident,
            bits,
            entitlements,
            entitlement_fields,
        })
    }
}

// no validation necessary...

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;

        tokens.extend(quote! {
            pub struct #ident {
                sealed: (),
            }

            impl State for #ident {
                const RAW: States = States::#ident;

                unsafe fn conjure() -> Self {
                    Self {
                        sealed: (),
                    }
                }
            }
        });

        if !self.entitlements.is_empty() {
            let entitlement_paths = self.entitlements.iter();

            tokens.extend(quote! {
                #(
                    unsafe impl ::proto_hal::stasis::Entitled<super::#entitlement_paths> for #ident {}
                )*
            });
        }
    }
}
