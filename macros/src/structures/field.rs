use std::{collections::HashMap, ops::Deref};

use darling::FromMeta;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Expr, Ident, Item};
use tiva::{Validate, Validator};

use crate::{
    access::{Access, AccessArgs},
    utils::{get_access_from_split, get_schema_from_set, Offset, Spanned, Width},
};

use super::{
    schema::{Schema, SchemaArgs, SchemaSpec, StatefulSchema, StatelessSchema},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldArgs {
    pub offset: Option<Offset>,
    pub width: Option<Width>,
    pub read: Option<AccessArgs>,
    pub write: Option<AccessArgs>,
    pub reset: Option<Expr>,
    pub schema: Option<Ident>,

    #[darling(default)]
    pub auto_increment: bool,
}

impl Args for FieldArgs {
    const NAME: &str = "field";
}

#[derive(Debug)]
pub struct StatefulFieldSpec {
    pub args: Spanned<FieldArgs>,
    pub ident: Ident,
    pub offset: Offset,
    pub schema: StatefulSchema,
    pub access: Access,
    pub reset: Expr,
}

#[derive(Debug)]
pub struct StatefulField {
    spec: StatefulFieldSpec,
}

#[derive(Debug)]
pub struct StatelessFieldSpec {
    pub args: Spanned<FieldArgs>,
    pub ident: Ident,
    pub offset: Offset,
    pub schema: StatelessSchema,
    pub access: Access,
    pub reset: Option<Expr>,
}

#[derive(Debug)]
pub struct StatelessField {
    spec: StatelessFieldSpec,
}

#[derive(Debug)]
pub enum FieldSpec {
    Stateful(StatefulFieldSpec),
    Stateless(StatelessFieldSpec),
}

#[derive(Debug)]
pub enum Field {
    Stateful(StatefulField),
    Stateless(StatelessField),
}

impl Field {
    pub fn args(&self) -> &Spanned<FieldArgs> {
        match self {
            Self::Stateful(field) => &field.args,
            Self::Stateless(field) => &field.args,
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            Self::Stateful(field) => &field.ident,
            Self::Stateless(field) => &field.ident,
        }
    }

    pub fn offset(&self) -> &Offset {
        match self {
            Self::Stateful(field) => &field.offset,
            Self::Stateless(field) => &field.offset,
        }
    }

    pub fn schema(&self) -> Schema {
        match self {
            Self::Stateful(field) => Schema::Stateful(field.schema.clone()),
            Self::Stateless(field) => Schema::Stateless(field.schema.clone()),
        }
    }
}

impl Deref for StatefulField {
    type Target = StatefulFieldSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl Deref for StatelessField {
    type Target = StatelessFieldSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl FieldSpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        schemas: &HashMap<Ident, Schema>,
        args: Spanned<FieldArgs>,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &args.schema {
            if items.next().is_some() {
                Err(syn::Error::new(
                    args.span(),
                    "fields with imported schemas must be empty",
                ))?
            }

            get_schema_from_set(schema, schemas)?
        } else {
            // the schema will be derived from the module contents
            SchemaSpec::parse(
                ident.clone(),
                SchemaArgs {
                    auto_increment: args.auto_increment,
                    width: args.width.ok_or(syn::Error::new_spanned(
                        ident.clone(),
                        "width must be specified",
                    ))?,
                }
                .with_span(args.span()),
                items,
            )?
            .validate()?
        };

        let offset = args.offset.unwrap_or(offset);
        let access = get_access_from_split(&args.read, &args.write, args.span())?;

        Ok(match schema {
            Schema::Stateful(schema) => {
                let reset = args.reset.clone().ok_or(syn::Error::new(
                    args.span(),
                    "stateful fields must have a reset specified",
                ))?;

                Self::Stateful(StatefulFieldSpec {
                    args,
                    ident,
                    offset,
                    schema,
                    access,
                    reset,
                })
            }
            Schema::Stateless(schema) => {
                let reset = args.reset.clone();

                Self::Stateless(StatelessFieldSpec {
                    args,
                    ident,
                    offset,
                    schema,
                    access,
                    reset,
                })
            }
        })
    }
}

impl Field {
    pub fn is_stateful(&self) -> bool {
        matches!(self, Self::Stateful(_))
    }
}

impl Validator<StatefulFieldSpec> for StatefulField {
    type Error = syn::Error;

    fn validate(spec: StatefulFieldSpec) -> Result<Self, Self::Error> {
        if spec.offset + spec.schema.width > 32 {
            let msg = format!(
                "field domain exceeds register domain. {{ domain: {}..{} }}",
                spec.offset,
                spec.offset + spec.schema.width
            );

            Err(Self::Error::new(spec.args.span(), msg))
        } else {
            Ok(Self { spec })
        }
    }
}

impl Validator<StatelessFieldSpec> for StatelessField {
    type Error = syn::Error;

    fn validate(spec: StatelessFieldSpec) -> Result<Self, Self::Error> {
        if spec.offset + spec.schema.width > 32 {
            Err(Self::Error::new(
                spec.args.span(),
                "field domain exceeds register domain",
            ))
        } else {
            Ok(Self { spec })
        }
    }
}

impl Validator<FieldSpec> for Field {
    type Error = syn::Error;

    fn validate(src: FieldSpec) -> Result<Self, Self::Error> {
        Ok(match src {
            FieldSpec::Stateful(spec) => Self::Stateful(spec.validate()?),
            FieldSpec::Stateless(spec) => Self::Stateless(spec.validate()?),
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident();
        let offset = self.offset();
        let width = *self.schema().width();

        let span = self.schema().args().span();

        let mut body = quote_spanned! { span =>
            pub const OFFSET: u8 = #offset;
            pub const WIDTH: u8 = #width;
        };

        match self {
            Self::Stateful(field) => {
                let spec = &field.spec;

                let reset_state = &spec.reset;

                let state_idents = spec.schema.states.iter().map(|state| state.ident.clone());
                let state_bits = spec.schema.states.iter().map(|state| state.bits);
                let state_bodies = spec.schema.states.iter().map(|state| quote! { #state });

                body.extend(quote_spanned! { span =>
                    #(
                        #state_bodies
                    )*

                    pub struct Any {
                        state: States,
                    }

                    pub type Reset = #reset_state;
                    pub const RESET: u32 = Reset::RAW as u32;

                    #[repr(u32)]
                    pub enum States {
                        #(
                            #state_idents = #state_bits,
                        )*
                    }

                    pub trait State {
                        const RAW: States;

                        unsafe fn conjure() -> Self;
                    }
                });
            }
            Self::Stateless(field) => {
                let spec = &field.spec;

                if let Some(reset) = &spec.reset {
                    body.extend(quote! {
                        pub const RESET: u32 = #reset;
                    });
                }
            }
        }

        tokens.extend(quote_spanned! { span =>
            pub mod #ident {
                #body
            }
        });
    }
}
