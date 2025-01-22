use std::collections::HashSet;

use darling::{util::SpannedValue, FromMeta};
use syn::{Ident, Meta, Path};

use crate::{
    structures::schema::Schema,
    utils::PathArray,
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
        read_args: Option<&SpannedValue<AccessArgs>>,
        write_args: Option<&SpannedValue<AccessArgs>>,
        read_schema: Schema,
        write_schema: Schema,
    ) -> syn::Result<Option<Self>> {
        let get_access_entitlements = |args: &AccessArgs| {
            let mut access_entitlements = HashSet::new();

            for entitlement in args.entitlements.elems.iter().cloned() {
                if !access_entitlements.insert(entitlement.clone()) {
                    Err(syn::Error::new_spanned(
                        entitlement,
                        "entitlement exists already",
                    ))?
                }
            }

            Ok::<_, syn::Error>(access_entitlements)
        };

        Ok(match (read_args, write_args) {
            (Some(read_args), Some(write_args)) => {
                if read_schema.width != write_schema.width {
                    Err(syn::Error::new(
                        read_args.span().join(write_args.span()).unwrap(),
                        "read and write schemas must be of equal width",
                    ))?
                }

                Some(Access::ReadWrite {
                    read: Read {
                        schema: read_schema,
                        entitlements: get_access_entitlements(read_args)?,
                        effects: (),
                    },
                    write: Write {
                        schema: write_schema,
                        entitlements: get_access_entitlements(write_args)?,
                        effects: (),
                    },
                })
            }
            (Some(args), None) => Some(Access::Read(Read {
                schema: read_schema,
                entitlements: get_access_entitlements(args)?,
                effects: (),
            })),
            (None, Some(args)) => Some(Access::Write(Write {
                schema: write_schema,
                entitlements: get_access_entitlements(args)?,
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
}
