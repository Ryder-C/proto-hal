use std::{collections::HashSet, convert::Infallible, ops::Deref};

use darling::FromMeta;
use syn::{Ident, Item};

use super::{
    state::{State, StateArgs},
    Args,
};
use crate::utils::{require_struct, Spanned, Width};
use tiva::{Validate, Validator};

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
    pub args: Spanned<SchemaArgs>,
    pub ident: Ident,
    pub width: Width,
    pub states: Vec<State>,
    pub entitlement_fields: HashSet<Ident>,
}

#[derive(Debug, Clone)]
pub struct StatefulSchema {
    spec: StatefulSchemaSpec,
}

#[derive(Debug, Clone)]
pub struct StatelessSchemaSpec {
    pub args: Spanned<SchemaArgs>,
    pub ident: Ident,
    pub width: Width,
}

#[derive(Debug, Clone)]
pub struct StatelessSchema {
    spec: StatelessSchemaSpec,
}

#[derive(Debug, Clone)]
pub enum SchemaSpec {
    Stateful(StatefulSchemaSpec),
    Stateless(StatelessSchemaSpec),
}

#[derive(Debug, Clone)]
pub enum Schema {
    Stateful(StatefulSchema),
    Stateless(StatelessSchema),
}

impl Schema {
    pub fn args(&self) -> &Spanned<SchemaArgs> {
        match self {
            Self::Stateful(schema) => &schema.args,
            Self::Stateless(schema) => &schema.args,
        }
    }

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

impl Deref for StatefulSchema {
    type Target = StatefulSchemaSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl Deref for StatelessSchema {
    type Target = StatelessSchemaSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
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

                let state = State::parse(s.ident.clone(), state_bits, state_args)?;

                state_bits = state.bits + 1;

                states.push(state);
            }
        }

        Ok(if states.is_empty() {
            Self::Stateless(StatelessSchemaSpec { args, ident, width })
        } else {
            Self::Stateful(StatefulSchemaSpec {
                args,
                ident,
                width,
                states,
                entitlement_fields,
            })
        })
    }
}

impl Validator<StatefulSchemaSpec> for StatefulSchema {
    type Error = syn::Error;

    fn validate(spec: StatefulSchemaSpec) -> Result<Self, Self::Error> {
        for state in &spec.states {
            if state.args.bits.is_none() && !spec.args.auto_increment {
                Err(syn::Error::new(state.args.span(), "state bit value `bits` must be specified. to infer the bit value, use `auto_increment`"))?
            }

            if state.bits >> spec.width != 0 {
                Err(syn::Error::new(
                    state.args.span(),
                    "state bit value does not fit within field width",
                ))?
            }
        }

        Ok(Self { spec })
    }
}

impl Validator<StatelessSchemaSpec> for StatelessSchema {
    type Error = Infallible;

    fn validate(spec: StatelessSchemaSpec) -> Result<Self, Self::Error> {
        Ok(Self { spec })
    }
}

impl Validator<SchemaSpec> for Schema {
    type Error = syn::Error;

    fn validate(src: SchemaSpec) -> Result<Self, Self::Error> {
        Ok(match src {
            SchemaSpec::Stateful(spec) => Self::Stateful(spec.validate()?),
            SchemaSpec::Stateless(spec) => Self::Stateless(match spec.validate() {
                Ok(v) => v,
                Err(_infalllible) => unreachable!(),
            }),
        })
    }
}
