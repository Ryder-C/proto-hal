use std::collections::HashMap;

use darling::FromMeta;
use proc_macro2::Span;
use syn::{spanned::Spanned, Expr, ExprLit, ExprRange, Ident, Item, Lit, LitInt, RangeLimits};

use crate::{
    access::Access,
    utils::{get_access_from_split, get_schema_from_set},
};

use super::{
    field::{FieldArgs, FieldSpec},
    schema::{SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, FromMeta)]
pub struct FieldArrayArgs {
    pub range: ExprRange,
    #[darling(rename = "field")]
    pub field_args: FieldArgs,
    #[darling(default)]
    pub auto_increment: bool,
}

impl Args for FieldArrayArgs {
    const NAME: &str = "field_array";
}

#[derive(Debug)]
pub struct FieldArraySpec {
    pub ident: Ident,
    pub range: ExprRange,
    pub schema: SchemaSpec,
    pub access: Access,
}

impl FieldArraySpec {
    pub fn parse<'a>(
        ident: Ident,
        schemas: &HashMap<Ident, SchemaSpec>,
        field_array_args: FieldArrayArgs,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &field_array_args.field_args.schema {
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
                    auto_increment: field_array_args.auto_increment,
                    width: field_array_args
                        .field_args
                        .width
                        .ok_or(syn::Error::new_spanned(
                            ident.clone(),
                            "width must be specified",
                        ))?,
                },
                items,
            )?
        };

        let access = get_access_from_split(
            &field_array_args.field_args.read,
            &field_array_args.field_args.write,
            ident.span(),
        )?;

        Ok(Self {
            ident,
            range: field_array_args.range,
            schema,
            access,
        })
    }

    pub fn to_fields(&self, mut offset: u8) -> syn::Result<Vec<FieldSpec>> {
        let mut fields = Vec::new();

        // get range from range expr (so stupid)
        let expr = *(self
            .range
            .start
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                self.range.start.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                self.range.start.span(),
                "range bound literals must be integers",
            ))?
        };

        let start = lit.base10_parse::<u8>()?;

        let expr = *(self
            .range
            .end
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                self.range.end.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                self.range.end.span(),
                "range bound literals must be integers",
            ))?
        };

        let end = lit.base10_parse::<u8>()?;

        let range: Box<dyn Iterator<Item = u8>> = match self.range.limits {
            RangeLimits::Closed(_) => Box::new(start..=end),
            RangeLimits::HalfOpen(_) => Box::new(start..end),
        };

        // generate fields
        for i in range {
            let field = FieldSpec {
                ident: Ident::new(
                    &self.ident.to_string().replace("X", &i.to_string()),
                    Span::call_site(),
                ),
                offset,
                schema: self.schema.clone(),
                access: self.access.clone(),
            };

            offset += field.schema.width;

            fields.push(field);
        }

        Ok(fields)
    }
}
