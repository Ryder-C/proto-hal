use std::collections::HashSet;

use darling::FromMeta;
use proc_macro2::Span;
use syn::{Ident, Path};

use crate::utils::PathArray;

use super::Args;

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct StateArgs {
    #[darling(default)]
    pub bits: Option<u32>,
    pub reset: bool,
    pub entitlements: PathArray,

    #[darling(skip)]
    pub span: Option<Span>,
}

impl Args for StateArgs {
    const NAME: &str = "state";

    fn attach_span(mut self, span: proc_macro2::Span) -> Self {
        self.span.replace(span);

        self
    }
}

#[derive(Debug, Clone)]
pub struct StateSpec {
    pub ident: Ident,
    pub bits: u32,
    pub entitlements: HashSet<Path>,
    pub entitlement_fields: HashSet<Ident>,
}

impl StateSpec {
    pub fn parse<'a>(ident: Ident, bits: u32, state_args: StateArgs) -> syn::Result<Self> {
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
            ident,
            bits,
            entitlements,
            entitlement_fields,
        })
    }
}
