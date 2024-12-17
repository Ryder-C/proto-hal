use darling::FromMeta;
use syn::{Ident, Item};

use crate::utils::{extract_items_from, require_module, Width};

use super::{
    state::{StateArgs, StateSpec},
    Args, Spec,
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

#[derive(Debug)]
pub struct SchemaSpec {
    pub ident: Ident,
    pub width: Width,
    pub states: Vec<StateSpec>,
}

impl Spec for SchemaSpec {
    type Inherited = Ident;
    type Args = SchemaArgs;

    fn parse<'a>(
        ident: Self::Inherited,
        args: Self::Args,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut schema = Self {
            ident,
            width: args.width,
            states: Vec::new(),
        };

        for item in items {
            let module = require_module(item)?;

            if let Some(state_args) = StateArgs::get(module.attrs.iter())? {
                let state = StateSpec::parse((), state_args, extract_items_from(module)?.iter())?;
            }
        }

        Ok(schema)
    }
}
