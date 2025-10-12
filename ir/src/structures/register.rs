use colored::Colorize;
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, parse_quote};

use crate::{
    structures::peripheral::Peripheral,
    utils::diagnostic::{Context, Diagnostic, Diagnostics},
};

use super::{entitlement::Entitlement, field::Field};

#[derive(Debug, Clone)]
pub struct Register {
    pub ident: Ident,
    pub offset: u32,
    pub fields: IndexMap<Ident, Field>,
    pub reset: Option<u32>,
    pub docs: Vec<String>,
}

impl Register {
    pub fn new(
        ident: impl AsRef<str>,
        offset: u32,
        fields: impl IntoIterator<Item = Field>,
    ) -> Self {
        Self {
            ident: Ident::new(ident.as_ref().to_lowercase().as_str(), Span::call_site()),
            offset,
            fields: IndexMap::from_iter(
                fields.into_iter().map(|field| (field.module_name(), field)),
            ),
            reset: None,
            docs: Vec::new(),
        }
    }

    pub fn reset(mut self, reset: u32) -> Self {
        self.reset = Some(reset);

        self
    }

    #[expect(unused)]
    pub fn entitlements(mut self, entitlements: impl IntoIterator<Item = Entitlement>) -> Self {
        todo!()
    }

    pub fn docs<I>(mut self, docs: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        self.docs
            .extend(docs.into_iter().map(|doc| doc.as_ref().to_string()));

        self
    }

    pub fn module_name(&self) -> Ident {
        self.ident.clone()
    }

    /// A register is resolvable if at least one field within it is resolvable.
    pub fn is_resolvable(&self) -> bool {
        self.fields.values().any(|field| field.is_resolvable())
    }

    pub fn validate(&self, context: &Context) -> Diagnostics {
        let mut diagnostics = Diagnostics::new();
        let new_context = context.clone().and(self.module_name().to_string());

        if self.offset % 4 != 0 {
            diagnostics.insert(
                Diagnostic::error(format!(
                    "register offset must be word aligned. (offset {} does not satisfy: offset % 4 == 0)",
                    self.offset
                ))
                    .with_context(new_context.clone()),
            );
        }

        let mut fields = self.fields.values().collect::<Vec<_>>();
        fields.sort_by(|lhs, rhs| lhs.offset.cmp(&rhs.offset));

        for (i, field) in fields.iter().enumerate() {
            let remaining = &fields[i + 1..];

            for other in remaining {
                if field.offset + field.width <= other.offset {
                    break;
                }

                // unfortunate workaround for `is_disjoint` behavior when sets are empty
                if !field.entitlements.is_empty()
                    && !other.entitlements.is_empty()
                    && field.entitlements.is_disjoint(&other.entitlements)
                {
                    continue;
                }

                diagnostics.insert(
                    Diagnostic::error(format!(
                        "fields [{}] and [{}] overlap.",
                        field.module_name().to_string().bold(),
                        other.module_name().to_string().bold()
                    ))
                    .with_context(new_context.clone())
                    .notes(
                        if !field.entitlements.is_empty() || !other.entitlements.is_empty() {
                            vec![format!(
                                "overlapping fields have non-trivial intersecting entitlement spaces {:?} and {:?}",
                                field.entitlements.iter().map(ToString::to_string).collect::<Vec<_>>(),
                                other.entitlements.iter().map(ToString::to_string).collect::<Vec<_>>(),
                            )]
                        } else {
                            vec![]
                        },
                    ),
                );
            }
        }

        if let Some(field) = fields.last()
            && field.offset + field.width > 32
        {
            diagnostics.insert(
                Diagnostic::error(format!(
                    "field [{}] exceeds register width.",
                    field.module_name().to_string().bold()
                ))
                .with_context(new_context.clone()),
            );
        }

        if self.is_resolvable() && self.reset.is_none() {
            diagnostics.insert(
                Diagnostic::error(
                    "a reset value must be specified for registers containing resolvable fields",
                )
                .notes([format!(
                    "resolvable fields: {}",
                    fields
                        .iter()
                        .filter(|field| field.is_resolvable())
                        .map(|field| field.module_name().to_string().bold().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )])
                .with_context(new_context.clone()),
            );
        }

        for field in fields {
            diagnostics.extend(field.validate(&new_context));
        }

        diagnostics
    }
}

// codegen
impl Register {
    fn generate_fields(&self) -> TokenStream {
        self.fields.values().fold(quote! {}, |mut acc, field| {
            acc.extend(field.generate());

            acc
        })
    }

    fn generate_addr(base_addr: u32, offset: u32) -> TokenStream {
        let addr = base_addr + offset;
        quote! {
            pub const ADDR: u32 = #addr;
        }
    }

    fn generate_reset<'a>(
        fields: impl Iterator<Item = &'a Field> + Clone,
        reset: Option<u32>,
    ) -> TokenStream {
        let field_idents = fields
            .clone()
            .map(|field| field.module_name())
            .collect::<Vec<_>>();
        let field_tys = fields
            .clone()
            .map(|field| field.type_name())
            .collect::<Vec<_>>();
        let reset_tys = fields
            .map(|field| {
                let ident = field.module_name();
                field.reset_ty(parse_quote! { #ident }, reset)
            })
            .collect::<Vec<_>>();

        quote! {
            pub struct Reset {
                #(
                    pub #field_idents: #field_idents::#field_tys<#reset_tys>,
                )*
            }

            impl ::proto_hal::stasis::Conjure for Reset {
                unsafe fn conjure() -> Self {
                    Self {
                        #(
                            #field_idents: unsafe { <#field_idents::#field_tys<#reset_tys> as ::proto_hal::stasis::Conjure>::conjure() },
                        )*
                    }
                }
            }
        }
    }
}

impl Register {
    pub fn generate(&self, parent: &Peripheral) -> TokenStream {
        let mut body = quote! {};

        let module_name = self.module_name();

        body.extend(self.generate_fields());
        body.extend(Self::generate_addr(parent.base_addr, self.offset));
        body.extend(Self::generate_reset(self.fields.values(), self.reset));

        let docs = &self.docs;
        quote! {
            #(#[doc = #docs])*
            pub mod #module_name {
                #body
            }
        }
    }
}
