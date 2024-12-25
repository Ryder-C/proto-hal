use std::{
    collections::HashMap,
    ops::{Deref, Range},
};

use darling::FromMeta;
use proc_macro2::Span;
use syn::{spanned::Spanned as _, Expr, ExprLit, ExprRange, Ident, Item, Lit, LitInt, RangeLimits};
use tiva::{Validate, Validator};

use crate::{
    access::{Access, AccessArgs},
    utils::{get_access_from_split, get_schema_from_set, Offset, Spanned, Width},
};

use super::{
    field::{Field, FieldArgs, StatefulFieldSpec, StatelessFieldSpec},
    schema::{Schema, SchemaArgs, SchemaSpec},
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
}

impl Args for FieldArrayArgs {
    const NAME: &str = "field_array";
}

#[derive(Debug)]
pub struct FieldArraySpec {
    pub args: Spanned<FieldArrayArgs>,
    pub ident: Ident,
    pub range: Range<u8>,
    pub offset: Offset,
    pub schema: Schema,
    pub access: Access,
    pub reset: Option<Expr>,
}

#[derive(Debug)]
pub struct FieldArray {
    spec: FieldArraySpec,
}

impl Deref for FieldArray {
    type Target = FieldArraySpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl FieldArraySpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: Offset,
        schemas: &HashMap<Ident, Schema>,
        args: Spanned<FieldArrayArgs>,
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

        let access = get_access_from_split(&args.read, &args.write, ident.span())?;

        // get range from range expr (so stupid)
        let expr = *(args
            .range
            .start
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                args.range.start.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                args.range.start.span(),
                "range bound literals must be integers",
            ))?
        };

        let start = lit.base10_parse::<u8>()?;

        let expr = *(args
            .range
            .end
            .clone()
            .unwrap_or(Box::new(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Int(LitInt::new("0", Span::call_site())),
            }))));
        let Expr::Lit(lit) = expr else {
            Err(syn::Error::new(
                args.range.end.span(),
                "range bounds must be literals",
            ))?
        };

        let Lit::Int(lit) = lit.lit else {
            Err(syn::Error::new(
                args.range.end.span(),
                "range bound literals must be integers",
            ))?
        };

        let end = lit.base10_parse::<u8>()?;

        let range = match args.range.limits {
            RangeLimits::Closed(_) => start..end + 1,
            RangeLimits::HalfOpen(_) => start..end,
        };

        let offset = args.offset.unwrap_or(offset);
        let reset = args.reset.clone();

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
    pub fn count(&self) -> u8 {
        self.range.clone().count() as _
    }

    pub fn to_fields(&self) -> syn::Result<Vec<Field>> {
        let mut fields = Vec::new();

        if !self.ident.to_string().contains("X") {
            Err(syn::Error::new(
                self.ident.span(),
                "field array module ident must contain 'X's to indicate replacement patterns",
            ))?
        }

        let mut offset = self.offset;

        // generate fields
        for i in self.range.clone() {
            let ident = Ident::new(
                &self.ident.to_string().replace("X", &i.to_string()),
                Span::call_site(),
            );

            let field = {
                let args = FieldArgs {
                    offset: self.args.offset,
                    width: self.args.width,
                    read: self.args.read.clone(),
                    write: self.args.write.clone(),
                    reset: self.args.reset.clone(),
                    schema: self.args.schema.clone(),
                    auto_increment: self.args.auto_increment,
                }
                .with_span(self.args.span());
                match self.schema.clone() {
                    Schema::Stateful(schema) => Field::Stateful(
                        StatefulFieldSpec {
                            args,
                            ident,
                            offset,
                            schema,
                            access: self.access.clone(),
                            reset: self.reset.clone().ok_or(syn::Error::new_spanned(
                                self.ident.clone(),
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
                }
            };

            offset += field.schema().width();

            fields.push(field);
        }

        Ok(fields)
    }
}

impl Validator<FieldArraySpec> for FieldArray {
    type Error = syn::Error;

    fn validate(spec: FieldArraySpec) -> Result<Self, Self::Error> {
        if spec.offset + spec.schema.width() > 32 {
            Err(Self::Error::new(
                spec.args.span(),
                "field domain exceeds register domain",
            ))
        } else {
            Ok(Self { spec })
        }
    }
}
