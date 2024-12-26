use std::{collections::HashSet, convert::Infallible, ops::Deref};

use darling::FromMeta;
use syn::{Ident, Item};

use super::{
    state::{State, StateArgs},
    state_array::{StateArray, StateArrayArgs},
    Args,
};
use crate::utils::{require_struct, Spanned, SynErrorCombinator, Width};
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
        let mut errors = SynErrorCombinator::new();

        let width = args.width;
        let mut states = Vec::new();
        let mut entitlement_fields = HashSet::new();

        let mut state_bits = 0u32;

        for item in items {
            let s = require_struct(item)?;

            let get_args = || {
                Ok::<_, syn::Error>((
                    StateArgs::get(s.attrs.iter())?,
                    StateArrayArgs::get(s.attrs.iter())?,
                ))
            };

            errors.try_maybe_then(get_args(), |arg_collection| {
                let entitlements = match arg_collection {
                    (Some(state_args), None) => {
                        let state = State::parse(s.ident.clone(), state_bits, state_args.clone())?;

                        state_bits = state.bits + 1;
                        states.push(state);

                        Ok(state_args.entitlements.elems.clone())
                    }
                    (None, Some(state_array_args)) => {
                        let state_array = StateArray::parse(
                            s.ident.clone(),
                            state_bits,
                            state_array_args.clone(),
                        )?;

                        state_bits = state_array.bits + state_array.count();
                        states.extend(state_array.to_states()?);

                        Ok(state_array_args.state.entitlements.elems.clone())
                    }
                    (None, None) => Err(syn::Error::new_spanned(s, "extraneous item")),
                    (Some(state_args), Some(state_array_args)) => {
                        let msg = "only one struct annotation is permitted";

                        let mut e = syn::Error::new(state_args.span(), msg);
                        e.combine(syn::Error::new(state_array_args.span(), msg));

                        Err(e)
                    }
                }?;

                // collect fields of state entitlements (specified in state args)
                for entitlement in &entitlements {
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

                Ok(())
            });
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
        let mut errors = SynErrorCombinator::new();

        for state in &spec.states {
            if state.args.bits.is_none() && !spec.args.auto_increment {
                errors.push(syn::Error::new(state.args.span(), "state bit value `bits` must be specified. to infer the bit value, use `auto_increment`"));
            }

            if state.bits >> spec.width != 0 {
                errors.push(syn::Error::new(
                    state.args.span(),
                    "state bit value does not fit within field width",
                ));
            }
        }

        errors.coalesce()?;

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
