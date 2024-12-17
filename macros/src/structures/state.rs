use std::collections::HashSet;

use darling::FromMeta;
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
}

impl Args for StateArgs {
    const NAME: &str = "state";
}

#[derive(Debug, Clone)]
pub struct StateSpec {
    ident: Ident,
    entitlements: HashSet<Path>,
}

impl StateSpec {
    pub fn parse<'a>(ident: Ident, state_args: StateArgs) -> syn::Result<Self> {
        let mut entitlements = HashSet::new();

        for entitlement in state_args.entitlements.elems.iter().cloned() {
            if !entitlements.insert(entitlement.clone()) {
                Err(syn::Error::new_spanned(
                    entitlement,
                    "entitlement already exists",
                ))?
            }
        }

        Ok(Self {
            ident,
            entitlements,
        })
    }
}
