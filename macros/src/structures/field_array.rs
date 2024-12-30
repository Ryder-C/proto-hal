use std::{collections::HashMap, ops::Range};

use darling::FromMeta;
use syn::{Expr, ExprRange, Ident, Item};
use tiva::Validate;

use crate::{
    access::Access,
    utils::{
        get_access_from_split, get_schema_from_set, parse_expr_range, FieldOffset, Spanned,
        SynErrorCombinator,
    },
};

use super::{
    field::{Field, FieldArgs, StatefulFieldSpec, StatelessFieldSpec},
    schema::{Schema, SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, FromMeta)]
pub struct FieldArrayArgs {
    pub range: ExprRange,

    #[darling(flatten)]
    pub field: FieldArgs,
}

impl Args for FieldArrayArgs {
    const NAME: &str = "field_array";
}

#[derive(Debug)]
pub struct FieldArray {
    pub args: Spanned<FieldArrayArgs>,
    pub ident: Ident,
    pub range: Range<u32>,
    pub offset: FieldOffset,
    pub schema: Schema,
    pub access: Access,
    pub reset: Option<Expr>,
}

impl FieldArray {
    pub fn parse<'a>(
        ident: Ident,
        offset: FieldOffset,
        schemas: &HashMap<Ident, Schema>,
        args: Spanned<FieldArrayArgs>,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &args.field.schema {
            // Q: wish this wasn't here as it is a validation step... kind of?
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
                    auto_increment: args.field.auto_increment,
                    width: *args
                        .field
                        .width
                        .ok_or(syn::Error::new(args.span(), "width must be specified"))?,
                }
                .with_span(args.span()),
                items,
            )?
            .validate()?
        };

        let access = get_access_from_split(
            args.field.read.as_deref(),
            args.field.write.as_deref(),
            args.span(),
        )?;

        let range = parse_expr_range(&args.range)?;

        let offset = args.field.offset.unwrap_or(offset);
        let reset = args.field.reset.as_deref().cloned();

        Ok(Self {
            args,
            ident,
            range,
            offset,
            schema,
            access,
            reset,
        })
    }
}

impl FieldArray {
    pub fn count(&self) -> usize {
        self.range.clone().count() as _
    }

    pub fn to_fields(&self) -> syn::Result<Vec<Field>> {
        let mut errors = SynErrorCombinator::new();
        let mut fields = Vec::new();
        let mut offset = self.offset;

        let replace_pos = self.ident.to_string().rfind("X").ok_or(syn::Error::new(
            self.ident.span(),
            "field array module ident must contain an 'X' to indicate replacement location",
        ))?;

        // generate fields
        for i in self.range.clone() {
            let mut s = self.ident.to_string();
            s.replace_range(replace_pos..replace_pos + 1, &i.to_string());
            let ident = Ident::new(&s, self.ident.span());

            let args = self.args.field.clone().with_span(self.args.span());

            let get_field = || {
                Ok::<_, syn::Error>(match self.schema.clone() {
                    Schema::Stateful(schema) => Field::Stateful(
                        StatefulFieldSpec {
                            args,
                            ident,
                            offset,
                            schema,
                            access: self.access.clone(),
                            reset: self.reset.clone().ok_or(syn::Error::new(
                                self.args.span(),
                                "stateful fields must have reset specified",
                            ))?,
                        }
                        .validate()?,
                    ),
                    Schema::Stateless(schema) => Field::Stateless(
                        StatelessFieldSpec {
                            args,
                            ident,
                            offset,
                            schema,
                            access: self.access.clone(),
                            reset: self.reset.clone(),
                        }
                        .validate()?,
                    ),
                })
            };

            errors.maybe_then(get_field(), |field| {
                offset += field.schema().width();

                fields.push(field);
            });
        }

        errors.coalesce()?;

        Ok(fields)
    }
}
