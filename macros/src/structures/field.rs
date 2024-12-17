use std::collections::{HashMap, HashSet};

use darling::FromMeta;
use syn::{Expr, Ident, Item};

use crate::{
    access::{Access, AccessArgs, Read, ReadWrite, Write},
    utils::{Offset, Width},
};

use super::{
    schema::{SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldArgs {
    #[darling(default)]
    pub auto_increment: bool,
    pub offset: Option<Offset>,
    pub width: Option<Width>,
    pub read: Option<AccessArgs>,
    pub write: Option<AccessArgs>,
    pub reset: Option<Expr>,
    pub schema: Option<Ident>,
}

impl Args for FieldArgs {
    const NAME: &str = "field";
}

#[derive(Debug)]
pub struct FieldSpec {
    pub ident: Ident,
    pub offset: Offset,
    pub schema: SchemaSpec,
    pub access: Access,
}

impl FieldSpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        schemas: &HashMap<Ident, SchemaSpec>,
        field_args: FieldArgs,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = field_args.schema {
            if items.next().is_some() {
                Err(syn::Error::new_spanned(
                    ident.clone(),
                    "fields with imported schemas should be empty",
                ))?
            }

            schemas
                .get(&schema)
                .cloned()
                .ok_or(syn::Error::new_spanned(schema, "schema does not exist"))?
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
                },
                items,
            )?
        };

        let mut access_entitlements = HashSet::new();

        for access_arg in [&field_args.read, &field_args.write] {
            if let Some(read) = access_arg {
                for entitlement in read.entitlements.elems.iter().cloned() {
                    if !access_entitlements.insert(entitlement.clone()) {
                        Err(syn::Error::new_spanned(
                            entitlement,
                            "entitlement exists already",
                        ))?
                    }
                }
            }
        }

        let access = match (field_args.read, field_args.write) {
            (Some(_), Some(_)) => Access::ReadWrite(ReadWrite {
                entitlements: access_entitlements,
                effects: (),
            }),
            (Some(_), None) => Access::Read(Read {
                entitlements: access_entitlements,
                effects: (),
            }),
            (None, Some(_)) => Access::Write(Write {
                entitlements: access_entitlements,
                effects: (),
            }),
            (None, None) => Err(syn::Error::new_spanned(
                ident.clone(),
                "fields must be readable or writable",
            ))?,
        };

        Ok(Self {
            ident,
            offset,
            schema,
            access,
        })
    }

    pub fn stateful(&self) -> bool {
        self.schema.stateful()
    }
}
