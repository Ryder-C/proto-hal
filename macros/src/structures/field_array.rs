use std::{collections::HashMap, ops::Range};

use darling::FromMeta;
use proc_macro2::Span;
use syn::{spanned::Spanned, Expr, ExprLit, ExprRange, Ident, Item, Lit, LitInt, RangeLimits};

use crate::{
    access::{Access, AccessArgs},
    utils::{get_access_from_split, get_schema_from_set, Offset, Width},
};

use super::{
    field::{FieldSpec, StatefulFieldSpec, StatelessFieldSpec},
    schema::{SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, FromMeta)]
pub struct FieldArrayArgs {
    pub range: ExprRange,
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

impl Args for FieldArrayArgs {
    const NAME: &str = "field_array";

    fn attach_span(mut self, span: proc_macro2::Span) -> Self {
        self.span.replace(span);

        self
    }
}

#[derive(Debug)]
pub struct FieldArraySpec {
    pub ident: Ident,
    pub range: Range<u8>,
    pub offset: Offset,
    pub schema: SchemaSpec,
    pub access: Access,
    pub reset: Option<Expr>,
}

impl FieldArraySpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        schemas: &HashMap<Ident, SchemaSpec>,
        field_array_args: FieldArrayArgs,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &field_array_args.schema {
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
                    width: field_array_args.width.ok_or(syn::Error::new_spanned(
                        ident.clone(),
                        "width must be specified",
                    ))?,
                    span: None,
                },
                items,
            )?
        };

        let access = get_access_from_split(
            &field_array_args.read,
            &field_array_args.write,
            ident.span(),
        )?;

        // get range from range expr (so stupid)
        let expr = *(field_array_args
            .range
            .start
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                field_array_args.range.start.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                field_array_args.range.start.span(),
                "range bound literals must be integers",
            ))?
        };

        let start = lit.base10_parse::<u8>()?;

        let expr = *(field_array_args
            .range
            .end
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                field_array_args.range.end.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                field_array_args.range.end.span(),
                "range bound literals must be integers",
            ))?
        };

        let end = lit.base10_parse::<u8>()?;

        let range = match field_array_args.range.limits {
            RangeLimits::Closed(_) => start..end + 1,
            RangeLimits::HalfOpen(_) => start..end,
        };

        let offset = field_array_args.offset.unwrap_or(offset);

        Ok(Self {
            ident,
            range,
            offset,
            schema,
            access,
            reset: field_array_args.reset,
        })
    }

    pub fn count(&self) -> u8 {
        self.range.clone().count() as _
    }

    pub fn to_fields(&self, mut offset: u8) -> syn::Result<Vec<FieldSpec>> {
        let mut fields = Vec::new();

        // generate fields
        for i in self.range.clone() {
            let ident = Ident::new(
                &self.ident.to_string().replace("X", &i.to_string()),
                Span::call_site(),
            );

            let field = match self.schema.clone() {
                SchemaSpec::Stateful(schema) => FieldSpec::Stateful(StatefulFieldSpec {
                    ident,
                    offset,
                    schema,
                    access: self.access.clone(),
                    reset: self.reset.clone().ok_or(syn::Error::new_spanned(
                        self.ident.clone(),
                        "stateful fields must have reset specified",
                    ))?,
                }),
                SchemaSpec::Stateless(schema) => FieldSpec::Stateless(StatelessFieldSpec {
                    ident,
                    offset,
                    schema,
                    access: self.access.clone(),
                    reset: self.reset.clone(),
                }),
            };

            offset += field.schema().width();

            fields.push(field);
        }

        Ok(fields)
    }
}
