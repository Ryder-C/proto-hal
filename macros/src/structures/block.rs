use std::collections::HashSet;

use darling::FromMeta;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Ident, Item, Path, Visibility};

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
    pub vis: Visibility,
}

impl BlockSpec {
    pub fn parse<'a>(
        ident: Ident,
        vis: Visibility,
        block_args: BlockArgs,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut block = Self {
            ident,
            base_addr: block_args.base_addr,
            entitlements: HashSet::new(),
            registers: Vec::new(),
            erase_mod: block_args.erase_mod,
            vis,
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

        let (stateful_registers, stateless_registers) = self
            .registers
            .iter()
            .partition::<Vec<_>, _>(|register| register.is_stateful());

        let stateful_register_idents = stateful_registers
            .iter()
            .map(|register| &register.ident)
            .collect::<Vec<_>>();

        let stateless_register_idents = stateless_registers
            .iter()
            .map(|register| &register.ident)
            .collect::<Vec<_>>();

        let stateful_register_tys = stateful_registers
            .iter()
            .map(|register| {
                Ident::new(
                    &inflector::cases::pascalcase::to_pascal_case(&register.ident.to_string()),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        let entitlement_idents = (0..self.entitlements.len())
            .map(|i| format_ident!("entitlement{}", i))
            .collect::<Vec<_>>();

        let entitlement_tys = (0..self.entitlements.len())
            .map(|i| format_ident!("Entitlement{}", i))
            .collect::<Vec<_>>();

        let reset_entitlement_tys = entitlement_tys
            .iter()
            .map(|_| {
                parse_quote! {
                    ::proto_hal::stasis::Unsatisfied
                }
            })
            .collect::<Vec<Path>>();

        let register_bodies = self.registers.iter().map(|register| quote! { #register });

        let mut body = quote! {
            #(
                #register_bodies
            )*

            const BASE_ADDR: u32 = #base_addr;

            pub struct Block<
                #(
                    #stateful_register_tys,
                )*

                #(
                    #entitlement_tys,
                )*
            > {
                #(
                    pub #stateful_register_idents: #stateful_register_tys,
                )*

                #(
                    pub #stateless_register_idents: #stateless_register_idents::Register,
                )*

                #(
                    pub #entitlement_idents: #entitlement_tys,
                )*
            }

            pub type Reset = Block<
                #(
                    #stateful_register_idents::Reset,
                )*

                #(
                    #reset_entitlement_tys,
                )*
            >;

            impl Reset {
                pub unsafe fn conjure() -> Self {
                    ::core::mem::transmute(())
                }
            }
        };

        let entitlements = self
            .entitlements
            .iter()
            .map(|path| {
                parse_quote! {
                    ::proto_hal::stasis::Entitlement<#path>
                }
            })
            .collect::<Vec<Path>>();

        for (i, (ident, ty)) in stateful_register_idents
            .iter()
            .zip(stateful_register_tys.iter())
            .enumerate()
        {
            let prev_register_idents = stateful_register_idents.get(..i).unwrap();
            let next_register_idents = stateful_register_idents.get(i + 1..).unwrap();

            let prev_register_tys = stateful_register_tys.get(..i).unwrap();
            let next_register_tys = stateful_register_tys.get(i + 1..).unwrap();

            body.extend(quote! {
                impl<#(#stateful_register_tys,)*> Block<#(#stateful_register_tys,)* #(#entitlements,)*>
                where
                    #ty: ::proto_hal::macro_utils::AsBuilder,
                {
                    pub fn #ident<R, B>(self, f: impl FnOnce(#ty::Builder) -> B) -> Block<#(#prev_register_tys,)* R, #(#next_register_tys,)* #(#entitlements,)*>
                    where
                        B: ::proto_hal::macro_utils::AsRegister<Register = R>,
                    {
                        Block {
                            #(
                                #prev_register_idents: self.#prev_register_idents,
                            )*

                            #ident: f(self.#ident.into()).into(),

                            #(
                                #next_register_idents: self.#next_register_idents,
                            )*

                            #(
                                #stateless_register_idents: self.#stateless_register_idents,
                            )*

                            #(
                                #entitlement_idents: self.#entitlement_idents,
                            )*
                        }
                    }
                }
            });
        }

        if !self.entitlements.is_empty() {
            body.extend(quote! {
                impl<#(#stateful_register_tys,)*> Block<#(#stateful_register_tys,)* #(#reset_entitlement_tys,)*> {
                    pub fn attach(self, #(#entitlement_idents: #entitlements,)*) -> Block<#(#stateful_register_tys,)* #(#entitlements,)*> {
                        Block {
                            #(
                                #stateful_register_idents: self.#stateful_register_idents,
                            )*

                            #(
                                #stateless_register_idents: self.#stateless_register_idents,
                            )*

                            #(
                                #entitlement_idents,
                            )*
                        }
                    }
                }
            });
        }

        let vis = &self.vis;

        tokens.extend(if self.erase_mod {
            body
        } else {
            quote! {
                #vis mod #ident {
                    #body
                }
            }
        })
    }
}
