use std::collections::{HashMap, HashSet};

use darling::FromMeta;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Expr, Ident, Item};

use crate::{
    access::{Access, AccessArgs, Read, ReadWrite, Write},
    utils::{get_access_from_split, get_schema_from_set, Offset, Width},
};

use super::{
    schema::{SchemaArgs, SchemaSpec, StatefulSchemaSpec, StatelessSchemaSpec},
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

    #[darling(skip)]
    pub span: Option<Span>,
}

impl Args for FieldArgs {
    const NAME: &str = "field";

    fn attach_span(mut self, span: Span) -> Self {
        self.span.replace(span);

        self
    }
}

#[derive(Debug)]
pub struct StatefulFieldSpec {
    pub ident: Ident,
    pub offset: Offset,
    pub schema: StatefulSchemaSpec,
    pub access: Access,
    pub reset: Expr,
}

#[derive(Debug)]
pub struct StatelessFieldSpec {
    pub ident: Ident,
    pub offset: Offset,
    pub schema: StatelessSchemaSpec,
    pub access: Access,
    pub reset: Option<Expr>,
}

#[derive(Debug)]
pub enum FieldSpec {
    Stateful(StatefulFieldSpec),
    Stateless(StatelessFieldSpec),
}

impl FieldSpec {
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

    pub fn schema(&self) -> SchemaSpec {
        match self {
            Self::Stateful(field) => SchemaSpec::Stateful(field.schema.clone()),
            Self::Stateless(field) => SchemaSpec::Stateless(field.schema.clone()),
        }
    }

    pub fn access(&self) -> &Access {
        match self {
            Self::Stateful(s) => &s.access,
            Self::Stateless(s) => &s.access,
        }
    }
}

impl FieldSpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        schemas: &HashMap<Ident, SchemaSpec>,
        field_args: FieldArgs,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &field_args.schema {
            if items.next().is_some() {
                Err(syn::Error::new_spanned(
                    ident.clone(),
                    "fields with imported schemas should be empty",
                ))?
            }

            get_schema_from_set(schema, schemas)?
        } else {
            // the schema will be derived from the module contents
            SchemaSpec::parse(
                ident.clone(),
                SchemaArgs {
                    auto_increment: field_args.auto_increment,
                    width: field_args.width.ok_or(syn::Error::new_spanned(
                        ident.clone(),
                        "width must be specified",
                    ))?,
                    span: None,
                },
                items,
            )?
        };

        let offset = field_args.offset.unwrap_or(offset);
        let access = get_access_from_split(&field_args.read, &field_args.write, ident.span())?;

        Ok(match schema {
            SchemaSpec::Stateful(schema) => {
                let reset = field_args.reset.ok_or(syn::Error::new(
                    field_args.span.unwrap(),
                    "stateful fields must have a reset specified",
                ))?;

                Self::Stateful(StatefulFieldSpec {
                    ident,
                    offset,
                    schema,
                    access,
                    reset,
                })
            }
            SchemaSpec::Stateless(schema) => Self::Stateless(StatelessFieldSpec {
                ident,
                offset,
                schema,
                access,
                reset: field_args.reset,
            }),
        })
    }

    pub fn is_stateful(&self) -> bool {
        matches!(self, Self::Stateful(_))
    }
}

impl ToTokens for FieldSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident();
        let offset = self.offset();
        let width = *self.schema().width();

        let mut body = quote! {
            pub const OFFSET: u8 = #offset;
            pub const WIDTH: u8 = #width;
        };

        if let Self::Stateful(field) = self {
            let reset_state = &field.reset;

            let state_idents = field
                .schema
                .states
                .iter()
                .map(|state| state.ident.clone())
                .collect::<Vec<_>>();

            let state_bits = field
                .schema
                .states
                .iter()
                .map(|state| state.bits)
                .collect::<Vec<_>>();

            body.extend(quote! {
                pub struct Any {
                    state: States,
                }

                pub type Reset = #reset_state;
                pub const RESET: u32 = Reset::RAW as u32;

                #[repr(u32)]
                pub enum States {
                    #(
                        #state_idents #state_bits,
                    )*
                }
            });
        }

        tokens.extend(quote! {
            pub mod #ident {
                #body
            }
        });
    }
}
