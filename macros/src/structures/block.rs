use std::collections::HashSet;

use darling::FromMeta;
use quote::{quote, ToTokens};
use syn::{Ident, Item, Path};

use crate::utils::{extract_items_from, require_module, PathArray};

use super::{
    register::{RegisterArgs, RegisterSpec},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct BlockArgs {
    pub base_addr: u32,
    pub entitlements: PathArray,
    pub auto_increment: bool,
    pub erase_mod: bool,
}

#[derive(Debug)]
pub struct BlockSpec {
    pub ident: Ident,
    pub base_addr: u32,
    pub entitlements: HashSet<Path>,
    pub registers: Vec<RegisterSpec>,

    pub erase_mod: bool,
}

impl BlockSpec {
    pub fn parse<'a>(
        ident: Ident,
        block_args: BlockArgs,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut block = Self {
            ident,
            base_addr: block_args.base_addr,
            entitlements: HashSet::new(),
            registers: Vec::new(),
            erase_mod: block_args.erase_mod,
        };

        for entitlement in block_args.entitlements.elems {
            if !block.entitlements.insert(entitlement.clone()) {
                Err(syn::Error::new_spanned(
                    entitlement,
                    "entitlement exists already",
                ))?
            }
        }

        let mut register_offset = 0u8;

        for item in items {
            let module = require_module(item)?;

            if let Some(register_args) = RegisterArgs::get(module.attrs.iter())? {
                if !block_args.auto_increment && register_args.offset.is_none() {
                    // TODO: improve the span of this error
                    Err(syn::Error::new_spanned(block.ident.clone(), "register offset must be specified. to infer offsets, add the `auto_increment` argument to the block attribute macro"))?
                }

                let register = RegisterSpec::parse(
                    module.ident.clone(),
                    register_args.offset.unwrap_or(register_offset),
                    register_args.clone(),
                    extract_items_from(module)?.iter(),
                )?;

                register_offset = register_args.offset.unwrap_or(register_offset) + 0x4;

                block.registers.push(register);
            } else {
                Err(syn::Error::new_spanned(module, "erroneous module"))?
            }
        }

        Ok(block)
    }
}

impl ToTokens for BlockSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let base_addr = self.base_addr;

        let body = quote! {
            const BASE_ADDR: u32 = #base_addr;
        };

        tokens.extend(if self.erase_mod {
            body
        } else {
            quote! {
                mod #ident {
                    #body
                }
            }
        })
    }
}
