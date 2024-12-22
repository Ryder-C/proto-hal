use std::collections::HashMap;

use darling::FromMeta;
use syn::{Ident, Item};

use crate::utils::{extract_items_from, require_module, Offset};

use super::{
    field::{FieldArgs, FieldSpec},
    field_array::{FieldArrayArgs, FieldArraySpec},
    schema::{SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct RegisterArgs {
    pub offset: Option<u8>,
    #[darling(default)]
    pub auto_increment: bool,
}

impl Args for RegisterArgs {
    const NAME: &str = "register";
}

#[derive(Debug)]
pub struct RegisterSpec {
    pub ident: Ident,
    pub offset: Offset,
    pub schemas: HashMap<Ident, SchemaSpec>,
    pub fields: Vec<FieldSpec>,
}

impl RegisterSpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        register_args: RegisterArgs,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut register = Self {
            ident,
            offset,
            schemas: HashMap::new(),
            fields: Vec::new(),
        };

        let mut field_offset = 0u8;

        for item in items {
            let module = require_module(item)?;

            // TODO: deny multiple different attributes on one item

            if let Some(schema_args) = SchemaArgs::get(module.attrs.iter())? {
                let schema = SchemaSpec::parse(
                    module.ident.clone(),
                    schema_args,
                    extract_items_from(module)?.iter(),
                )?;

                register.schemas.insert(schema.ident.clone(), schema);
            }

            if let Some(field_args) = FieldArgs::get(module.attrs.iter())? {
                if !register_args.auto_increment && field_args.offset.is_none() {
                    // TODO: improve the span of this error
                    Err(syn::Error::new_spanned(register.ident.clone(), "field offset must be specified. to infer offsets, add the `auto_increment` argument to the register attribute macro"))?
                }

                let field = FieldSpec::parse(
                    module.ident.clone(),
                    field_args.offset.unwrap_or(field_offset),
                    &register.schemas,
                    field_args.clone(),
                    extract_items_from(module)?.iter(),
                )?;

                field_offset = field_args.offset.unwrap_or(field_offset) + field.schema.width;

                register.fields.push(field);
            }

            if let Some(field_array_args) = FieldArrayArgs::get(module.attrs.iter())? {
                let field_array = FieldArraySpec::parse(
                    module.ident.clone(),
                    &register.schemas,
                    field_array_args,
                    extract_items_from(module)?.iter(),
                )?;

                register.fields.extend(field_array.to_fields(field_offset)?);
            }
        }

        Ok(register)
    }
}
