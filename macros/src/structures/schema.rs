use darling::FromMeta;
use syn::{Ident, Item};

use crate::utils::{extract_items_from, require_module, require_struct, Width};

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
pub struct SchemaSpec {
    pub ident: Ident,
    pub width: Width,
    pub states: Vec<StateSpec>,
}

impl SchemaSpec {
    pub fn parse<'a>(
        ident: Ident,
        args: SchemaArgs,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut schema = Self {
            ident,
            width: args.width,
            states: Vec::new(),
        };

        for item in items {
            let s = require_struct(item)?;

            if let Some(state_args) = StateArgs::get(s.attrs.iter())? {
                let state = StateSpec::parse(s.ident.clone(), state_args)?;

                schema.states.push(state);
            }
        }

        Ok(schema)
    }

    pub fn stateful(&self) -> bool {
        !self.states.is_empty()
    }
}
