use std::collections::HashSet;

use darling::FromMeta;
use syn::{Ident, Item};

use crate::utils::{require_struct, Spanned, Width};

use super::{
    state::{StateArgs, StateSpec},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
pub struct SchemaArgs {
    #[darling(default)]
    pub auto_increment: bool,
    pub width: u8,
}

impl Args for SchemaArgs {
    const NAME: &str = "schema";
}

#[derive(Debug, Clone)]
pub struct StatefulSchemaSpec {
    pub ident: Ident,
    pub width: Width,
    pub states: Vec<StateSpec>,
    pub entitlement_fields: HashSet<Ident>,
}

#[derive(Debug, Clone)]
pub struct StatelessSchemaSpec {
    pub ident: Ident,
    pub width: Width,
}

#[derive(Debug, Clone)]
pub enum SchemaSpec {
    Stateful(StatefulSchemaSpec),
    Stateless(StatelessSchemaSpec),
}

impl SchemaSpec {
    pub fn ident(&self) -> &Ident {
        match self {
            Self::Stateful(schema) => &schema.ident,
            Self::Stateless(schema) => &schema.ident,
        }
    }

    pub fn width(&self) -> &Width {
        match self {
            Self::Stateful(schema) => &schema.width,
            Self::Stateless(schema) => &schema.width,
        }
    }
}

impl SchemaSpec {
    pub fn parse<'a>(
        ident: Ident,
        args: Spanned<SchemaArgs>,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let width = args.width;
        let mut states = Vec::new();
        let mut entitlement_fields = HashSet::new();

        let mut state_bits = 0u32;

        for item in items {
            let s = require_struct(item)?;

            if let Some(state_args) = StateArgs::get(s.attrs.iter())? {
                if state_args.bits.is_none() && !args.auto_increment {
                    Err(syn::Error::new(state_args.span.unwrap(), "state bit value `bits` must be specified. to infer the bit value, use `auto_increment`"))?
                }

                // collect fields of state entitlements (specified in state args)
                for entitlement in &state_args.entitlements.elems {
                    // TODO: this can't be correct
                    entitlement_fields.insert(
                        entitlement
                            .segments
                            .iter()
                            .nth_back(1)
                            .unwrap()
                            .ident
                            .clone(),
                    );
                }

                let state = StateSpec::parse(s.ident.clone(), state_bits, state_args)?;

                state_bits = state.bits + 1;

                states.push(state);
            }
        }

        Ok(if states.is_empty() {
            Self::Stateless(StatelessSchemaSpec { ident, width })
        } else {
            Self::Stateful(StatefulSchemaSpec {
                ident,
                width,
                states,
                entitlement_fields,
            })
        })
    }
}
