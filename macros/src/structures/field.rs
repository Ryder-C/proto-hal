use std::{collections::HashMap, ops::Deref};

use darling::{util::SpannedValue, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{Expr, Ident, Item};
use tiva::{Validate, Validator};

use crate::{
    access::{Access, AccessArgs},
    utils::{
        get_access_from_split, get_schema_from_set, FieldOffset, Spanned, SynErrorCombinator, Width,
    },
};

use super::{
    schema::{Numericity, Schema, SchemaArgs, SchemaSpec},
    Args,
};

#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldArgs {
    pub offset: Option<FieldOffset>,
    pub width: Option<SpannedValue<Width>>,
    pub schema: Option<SpannedValue<Ident>>,
    pub read: Option<SpannedValue<AccessArgs>>,
    pub write: Option<SpannedValue<AccessArgs>>,
    pub reset: Option<SpannedValue<Expr>>,

    #[darling(default)]
    pub auto_increment: bool,
}

impl Args for FieldArgs {
    const NAME: &str = "field";
}

#[derive(Debug)]
pub enum Resolvability {
    Resolvable { reset: Expr },
    Unresolvable,
}

#[derive(Debug)]
pub struct FieldSpec {
    pub args: Spanned<FieldArgs>,
    pub ident: Ident,
    pub offset: FieldOffset,
    pub schema: Schema,
    pub access: Access,

    // private because it is a computed
    // property
    resolvability: Resolvability,
}

#[derive(Debug)]
pub struct Field {
    spec: FieldSpec,
}

impl Deref for Field {
    type Target = FieldSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl FieldSpec {
    pub fn parse<'a>(
        ident: Ident,
        offset: FieldOffset,
        schemas: &HashMap<Ident, Schema>,
        args: Spanned<FieldArgs>,
        mut items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let schema = if let Some(schema) = &args.schema {
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
                    auto_increment: args.auto_increment,
                    width: *args
                        .width
                        .ok_or(syn::Error::new(args.span(), "width must be specified"))?,
                }
                .with_span(args.span()),
                items,
            )?
            .validate()?
        };

        let offset = args.offset.unwrap_or(offset);
        let access =
            get_access_from_split(args.read.as_deref(), args.write.as_deref(), args.span())?;

        Ok(Self::new(args, ident, offset, schema, access)?)
    }
}

impl FieldSpec {
    pub fn new(
        args: Spanned<FieldArgs>,
        ident: Ident,
        offset: FieldOffset,
        schema: Schema,
        access: Access,
    ) -> Result<Self, syn::Error> {
        let resolvability = Self::resolvability(&args, &access)?;

        Ok(Self {
            args,
            ident,
            offset,
            schema,
            access,
            resolvability,
        })
    }

    pub fn is_resolvable(&self) -> bool {
        matches!(&self.resolvability, Resolvability::Resolvable { reset: _ })
    }

    fn resolvability(
        args: &Spanned<FieldArgs>,
        access: &Access,
    ) -> Result<Resolvability, syn::Error> {
        /*
        Determining Resolvability:

        For a field to be resolvable, it is to
        be statically analyzable. That means it can
        be configured such that its state is trackable
        with the type system.

        For a field to be resolvable, it must be:
        1. explicitly resolved by an effect somewhere or
        2. read and write

        If a field is purely read, with no resolving effects,
        this field's value must be sourced from an external
        influence (for example the IDR register of GPIO) and
        as such cannot be resolved statically.

        An example of a field that is read only but *has*
        resolving effects, is the RRDY bit in CORDIC.
        We know this bit to be 1 when the interrupt fires,
        so at that point it is resolved.

        An example of an unresolvable write only field
        is the ARG field of the CORDIC. The register
        maintains no state and can only be written to,
        so it has no state to resolve, thus unresolvable.

        ---

        This is NOT to be confused with statefulness. Fields
        can be stateful and unresolvable. Some peripheral states
        simply may be too dynamic to be tracked statically.
        */

        Ok(if matches!(access, Access::ReadWrite(_)) {
            Resolvability::Resolvable {
                reset: args.reset.as_deref().cloned().ok_or(syn::Error::new(
                    args.span(),
                    "resolvable fields must have a reset specified",
                ))?,
            }
        } else {
            Resolvability::Unresolvable
        })
    }
}

impl Validator<FieldSpec> for Field {
    type Error = syn::Error;

    fn validate(spec: FieldSpec) -> Result<Self, Self::Error> {
        let mut errors = SynErrorCombinator::new();

        if spec.args.width.is_some() && spec.args.schema.is_some() {
            errors.push(syn::Error::new(
                spec.args.span(),
                "field width is inherited from imported schema",
            ));
        }

        if spec.offset + spec.schema.width > 32 {
            let msg = format!(
                "field domain exceeds register domain. {{ domain: {}..{} }}",
                spec.offset,
                spec.offset + spec.schema.width
            );

            errors.push(Self::Error::new(spec.args.span(), msg));
        }

        errors.coalesce()?;

        Ok(Self { spec })
    }
}

impl Field {
    fn maybe_generate_state_bodies(&self) -> Option<TokenStream2> {
        if !self.is_resolvable() {
            return None;
        };
        let Numericity::Enumerated { variants } = &self.schema.numericity else {
            return None;
        };

        let span = self.args.span();

        // NOTE: the variant renders to a state implementation
        let state_bodies = variants.iter().map(|variant| quote! { #variant });

        Some(quote_spanned! { span =>
            #(
                #state_bodies
            )*
        })
    }

    fn generate_offset_const(&self) -> TokenStream2 {
        let span = self.args.span();

        let offset = self.offset;

        quote_spanned! { span =>
            pub const OFFSET: u8 = #offset;
        }
    }

    fn generate_width_const(&self) -> TokenStream2 {
        let span = self.args.span();

        let width = self.schema.width;

        quote_spanned! { span =>
            pub const WIDTH: u8 = #width;
        }
    }

    fn maybe_generate_resets(&self) -> Option<TokenStream2> {
        let span = self.args.span();

        let Resolvability::Resolvable { reset } = &self.resolvability else {
            return None;
        };

        match &self.schema.numericity {
            Numericity::Enumerated { variants: _ } => Some(quote_spanned! { span =>
                pub type Reset = #reset;
                pub const RESET: u32 = Reset::RAW as u32;
            }),
            Numericity::Numeric => todo!(),
        }
    }

    fn maybe_generate_variant_enum(&self) -> Option<TokenStream2> {
        let span = self.args.span();

        let Numericity::Enumerated { variants } = &self.schema.numericity else {
            return None;
        };

        let variant_idents = variants
            .iter()
            .map(|variant| variant.ident.clone())
            .collect::<Vec<_>>();
        let variant_bits = variants
            .iter()
            .map(|variant| variant.bits)
            .collect::<Vec<_>>();

        let is_variant_idents = variants.iter().map(|variant| {
            format_ident!(
                "is_{}",
                inflector::cases::snakecase::to_snake_case(&variant.ident.to_string())
            )
        });

        Some(quote_spanned! { span =>
            #[repr(u32)]
            pub enum Variant {
                #(
                    #variant_idents = #variant_bits,
                )*
            }

            impl Variant {
                pub unsafe fn from_bits(bits: u32) -> Self {
                    match bits {
                        #(
                            #variant_bits => Self::#variant_idents,
                        )*
                        _ => ::core::hint::unreachable_unchecked(),
                    }
                }

                #(
                    pub fn #is_variant_idents(&self) -> bool {
                        matches!(self, Self::#variant_idents)
                    }
                )*
            }
        })
    }

    fn maybe_generate_state_trait(&self) -> Option<TokenStream2> {
        let span = self.args.span();

        if !self.is_resolvable() {
            return None;
        };

        match &self.schema.numericity {
            Numericity::Enumerated { variants } => {
                let variant_idents = variants
                    .iter()
                    .map(|variant| variant.ident.clone())
                    .collect::<Vec<_>>();

                let conversion_methods = if self.access.is_write() {
                    let into_func_idents = variant_idents.iter().map(|ident| {
                        format_ident!(
                            "into_{}",
                            inflector::cases::snakecase::to_snake_case(&ident.to_string())
                        )
                    });

                    let warning_msg = "# Warning
This method incurs a runtime cost and is lossy,
as an entire register read is needed to mutate a single field.
Consider using register accessors when performing state transitions.";

                    let into_func_docs = variant_idents.iter().map(|ident| {
                        format!("Convert this state into [`{}`].\n{}", ident, warning_msg)
                    });

                    Some(quote! {
                        /// Convert this state into a new state.
                        #[doc = #warning_msg]
                        fn into_state<S>(self) -> S
                        where
                            S: State,
                        {
                            // SAFETY: assumes the proc macro implementation is sound
                            // and that the peripheral description is accurate
                            let mut reg_value = unsafe { core::ptr::read_volatile((super::super::BASE_ADDR + super::OFFSET) as *const u32) };

                            // i.e.
                            // 0000 0000 0000 0000 0111 1111 1100 0000
                            const MASK: u32 = (0xffff_ffff >> (32 - (WIDTH as u32))) << (OFFSET as u32);

                            reg_value &= !MASK;
                            reg_value |= (S::RAW as u32) << (OFFSET as u32);

                            // SAFETY: assumes the proc macro implementation is sound
                            // and that the peripheral description is accurate
                            unsafe {
                                core::ptr::write_volatile((super::super::BASE_ADDR + super::OFFSET) as *mut u32, reg_value);
                            }

                            // SAFETY:
                            // 1. previous state is moved and destroyed
                            // 2. state has been written to field
                            unsafe { S::conjure() }
                        }

                        #(
                            #[doc = #into_func_docs]
                            fn #into_func_idents(self) -> #variant_idents
                            {
                                self.into_state()
                            }
                        )*
                    })
                } else {
                    None
                };

                Some(quote_spanned! { span =>
                    pub trait State: ::proto_hal::stasis::Freeze {
                        const RAW: Variant;

                        unsafe fn conjure() -> Self;

                        #conversion_methods
                    }
                })
            }
            Numericity::Numeric => todo!(),
        }
    }

    fn generate_module_docs(&self) -> TokenStream2 {
        let span = self.args.span();

        let access_doc = match &self.access {
            Access::Read(_) => "- Access: read",
            Access::Write(_) => "- Access: write",
            Access::ReadWrite(_) => "- Access: read/write",
        };

        let domain_doc = format!(
            "- Domain: {}..{}",
            self.offset,
            self.offset + self.schema.width
        );

        let resolvability_doc = if self.is_resolvable() {
            "- Type: resolvable"
        } else {
            "- Type: unresolvable"
        };

        let variants_doc = if let Numericity::Enumerated { variants } = &self.schema.numericity {
            let msg = format!("\t- Variants: {}", variants.len());

            Some(quote! { #[doc = #msg] })
        } else {
            None
        };

        quote_spanned! { span =>
            #[doc = "A register field with the following properties:"]
            #[doc = #access_doc]
            #[doc = #domain_doc]
            #[doc = #resolvability_doc]
            #variants_doc
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let span = self.args.span();
        let ident = &self.ident;
        let mut body = TokenStream2::new();

        body.extend(self.maybe_generate_state_bodies());
        body.extend(self.generate_offset_const());
        body.extend(self.generate_width_const());
        body.extend(self.maybe_generate_resets());
        body.extend(self.maybe_generate_variant_enum());
        body.extend(self.maybe_generate_state_trait());

        let docs = self.generate_module_docs();

        tokens.extend(quote_spanned! { span =>
            #docs
            pub mod #ident {
                #body
            }
        });
    }
}
