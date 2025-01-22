use std::collections::{HashMap, HashSet};

use darling::{util::SpannedValue, FromMeta};
use syn::{Ident, Meta, Path};

use crate::{
    structures::schema::Schema,
    utils::{get_schema_from_set, PathArray, Spanned, SynErrorCombinator},
};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct AccessArgs {
    pub schema: Option<Ident>,
    pub entitlements: PathArray,
    pub effect: Option<Meta>,
}

#[derive(Debug, Clone)]
pub struct Read {
    pub schema: Schema,
    pub entitlements: HashSet<Path>,
    pub effects: (),
}

#[derive(Debug, Clone)]
pub struct Write {
    pub schema: Schema,
    pub entitlements: HashSet<Path>,
    pub effects: (),
}

#[derive(Debug, Clone)]
pub enum Access {
    Read(Read),
    Write(Write),
    ReadWrite { read: Read, write: Write },
}

impl Access {
    pub fn new(
        schemas: &HashMap<Ident, Schema>,
        read_args: Option<&SpannedValue<AccessArgs>>,
        write_args: Option<&SpannedValue<AccessArgs>>,
    ) -> syn::Result<Option<Self>> {
        let mut access_entitlements = HashSet::new();

        for access_arg in [read_args, write_args].into_iter().flatten() {
            for entitlement in access_arg.entitlements.elems.iter().cloned() {
                if !access_entitlements.insert(entitlement.clone()) {
                    Err(syn::Error::new_spanned(
                        entitlement,
                        "entitlement exists already",
                    ))?
                }
            }
        }

        let get_schema = |args: &SpannedValue<AccessArgs>| {
            get_schema_from_set(
                &args
                    .schema
                    .clone()
                    .ok_or(syn::Error::new(args.span(), "schema must be specified"))?,
                schemas,
            )
        };

        Ok(match (read_args, write_args) {
            (Some(read_args), Some(write_args)) => {
                let read_schema = get_schema(read_args)?;
                let write_schema = get_schema(write_args)?;

                if read_schema.width != write_schema.width {
                    Err(syn::Error::new(
                        read_args.span().join(write_args.span()).unwrap(),
                        "read and write schemas must be of equal width",
                    ))?
                }

                Some(Access::ReadWrite {
                    read: Read {
                        schema: read_schema,
                        entitlements: access_entitlements.clone(),
                        effects: (),
                    },
                    write: Write {
                        schema: write_schema,
                        entitlements: access_entitlements,
                        effects: (),
                    },
                })
            }
            (Some(args), None) => Some(Access::Read(Read {
                schema: get_schema(args)?,
                entitlements: access_entitlements,
                effects: (),
            })),
            (None, Some(args)) => Some(Access::Write(Write {
                schema: get_schema(args)?,
                entitlements: access_entitlements,
                effects: (),
            })),
            (None, None) => None,
        })
    }

    pub fn is_read(&self) -> bool {
        match self {
            Self::Read(_) | Self::ReadWrite { read: _, write: _ } => true,
            Self::Write(_) => false,
        }
    }

    pub fn is_write(&self) -> bool {
        match self {
            Self::Write(_) | Self::ReadWrite { read: _, write: _ } => true,
            Self::Read(_) => false,
        }
    }

    pub fn schemas(&self) -> impl Iterator<Item = &Schema> {
        match self {
            Access::Read(read) => vec![&read.schema].into_iter(),
            Access::Write(write) => vec![&write.schema].into_iter(),
            Access::ReadWrite { read, write } => vec![&read.schema, &write.schema].into_iter(),
        }
    }
}
