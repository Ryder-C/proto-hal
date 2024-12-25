use std::{collections::HashMap, ops::Deref};

use darling::FromMeta;
use proc_macro2::Span;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{parse_quote, Ident, Index, Item, Path};
use tiva::Validator;

use crate::utils::{extract_items_from, require_module, Offset, Spanned, SynErrorCombinator};

use super::{
    field::{Field, FieldArgs, FieldSpec},
    field_array::{FieldArray, FieldArrayArgs},
    schema::{Schema, SchemaArgs, SchemaSpec},
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
    pub args: Spanned<RegisterArgs>,
    pub ident: Ident,
    pub offset: Offset,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Register {
    spec: RegisterSpec,
}

impl Deref for Register {
    type Target = RegisterSpec;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl RegisterSpec {
    pub fn parse<'a>(
        ident: Ident,
        schemas: &mut HashMap<Ident, Schema>,
        offset: Offset,
        args: Spanned<RegisterArgs>,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        let mut errors = SynErrorCombinator::new();

        let mut register = Self {
            args: args.clone(),
            ident,
            offset,
            fields: Vec::new(),
        };

        let mut field_offset = 0u8;

        for item in items {
            let module = require_module(item)?;

            // TODO: this isn't the most flexible solution
            // but it does work for now.
            // args should be dispatched procedurally.
            match (
                SchemaArgs::get(module.attrs.iter())?,
                FieldArgs::get(module.attrs.iter())?,
                FieldArrayArgs::get(module.attrs.iter())?,
            ) {
                (Some(schema_args), None, None) => {
                    errors.try_maybe_then(
                        SchemaSpec::parse(
                            module.ident.clone(),
                            schema_args,
                            extract_items_from(module)?.iter(),
                        ),
                        |spec| {
                            let schema = Schema::validate(spec)?;
                            schemas.insert(schema.ident().clone(), schema);

                            Ok(())
                        },
                    );
                }
                (None, Some(field_args), None) => {
                    errors.try_maybe_then(
                        FieldSpec::parse(
                            module.ident.clone(),
                            field_args.offset.unwrap_or(field_offset),
                            schemas,
                            field_args,
                            extract_items_from(module)?.iter(),
                        ),
                        |spec| {
                            let field = Field::validate(spec)?;

                            field_offset = field.offset() + field.schema().width();
                            register.fields.push(field);

                            Ok(())
                        },
                    );
                }
                (None, None, Some(field_array_args)) => {
                    errors.try_maybe_then(
                        FieldArray::parse(
                            module.ident.clone(),
                            field_array_args.field.offset.unwrap_or(field_offset),
                            schemas,
                            field_array_args,
                            extract_items_from(module)?.iter(),
                        ),
                        |field_array| {
                            register.fields.extend(field_array.to_fields()?);
                            field_offset = field_array.offset
                                + field_array.schema.width() * field_array.count();

                            Ok(())
                        },
                    );
                }
                (None, None, None) => {
                    errors.push(syn::Error::new_spanned(module, "extraneous item"));
                }
                (schema_args, field_args, field_array_args) => {
                    let msg = "only one module annotation is permitted";

                    for span in [
                        schema_args.and_then(|args| Some(args.span())),
                        field_args.and_then(|args| Some(args.span())),
                        field_array_args.and_then(|args| Some(args.span())),
                    ]
                    .into_iter()
                    .flatten()
                    {
                        errors.push(syn::Error::new(span, msg));
                    }
                }
            }
        }

        errors.coalesce()?;

        Ok(register)
    }

    pub fn is_stateful(&self) -> bool {
        self.fields.iter().any(|field| field.is_stateful())
    }
}

impl Validator<RegisterSpec> for Register {
    type Error = syn::Error;

    fn validate(spec: RegisterSpec) -> Result<Self, Self::Error> {
        let mut errors = SynErrorCombinator::new();

        for field in &spec.fields {
            if field.args().offset.is_none() && !spec.args.auto_increment {
                errors.push(syn::Error::new(
                    field.args().span(),
                    "field offset must be specified. to infer offsets, use `auto_increment`",
                ));
            }
        }

        for slice in spec.fields.windows(2) {
            let lhs = slice.first().unwrap();
            let rhs = slice.last().unwrap();
            if lhs.offset() + lhs.schema().width() > *rhs.offset() {
                let msg = format!(
                    "{} {{ domain: {}..{} }}, {} {{ domain: {}..{} }}",
                    lhs.ident(),
                    lhs.offset(),
                    lhs.offset() + lhs.schema().width(),
                    rhs.ident(),
                    rhs.offset(),
                    rhs.offset() + rhs.schema().width(),
                );

                let mut e = syn::Error::new(
                    spec.args.span(),
                    format!("field domains overlapping or unordered. {msg}"),
                );

                e.combine(syn::Error::new(
                    lhs.ident().span(),
                    format!(
                        "field '{}' is overlapping or out of order with '{}'. {}",
                        lhs.ident(),
                        rhs.ident(),
                        msg,
                    ),
                ));

                e.combine(syn::Error::new(
                    rhs.ident().span(),
                    format!(
                        "field '{}' is overlapping or out of order with '{}'. {}",
                        rhs.ident(),
                        lhs.ident(),
                        msg,
                    ),
                ));

                errors.push(e);
            }
        }

        errors.coalesce()?;

        Ok(Self { spec })
    }
}

impl ToTokens for Register {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let offset = self.offset;

        let span = self.args.span();

        let stateful_fields = self
            .fields
            .iter()
            .filter_map(|field| match field {
                Field::Stateful(field) => Some(field),
                _ => None,
            })
            .collect::<Vec<_>>();

        let stateless_fields = self
            .fields
            .iter()
            .filter_map(|field| match field {
                Field::Stateless(field) => Some(field),
                _ => None,
            })
            .collect::<Vec<_>>();

        let stateful_field_idents = stateful_fields
            .iter()
            .map(|field| &field.ident)
            .collect::<Vec<_>>();

        let stateless_field_idents = stateless_fields
            .iter()
            .map(|field| format_ident!("_{}", field.ident));

        let stateful_field_tys = stateful_fields
            .iter()
            .map(|field| {
                Ident::new(
                    &inflector::cases::pascalcase::to_pascal_case(&field.ident.to_string()),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        let new_stateful_field_tys = stateful_field_tys
            .iter()
            .map(|ident| format_ident!("New{}", ident))
            .collect::<Vec<_>>();

        let field_bodies = self
            .fields
            .iter()
            .map(|field| quote_spanned! { span => #field });

        let mut body = quote_spanned! { span =>
            #(
                #field_bodies
            )*

            pub const OFFSET: u32 = #offset as _;

            pub struct Register<#(#stateful_field_tys,)*> {
                #(
                    pub #stateful_field_idents: #stateful_field_tys,
                )*

                // Q: what is this for?
                #(
                    #stateless_field_idents: (),
                )*
            }
        };

        if self.is_stateful() {
            let writable_stateful_fields = stateful_fields
                .iter()
                .filter(|field| field.access.is_read())
                .collect::<Vec<_>>();

            let writable_stateful_field_idents = writable_stateful_fields
                .iter()
                .map(|field| &field.ident)
                .collect::<Vec<_>>();

            let writable_stateful_field_tys = writable_stateful_field_idents
                .iter()
                .map(|ident| {
                    Ident::new(
                        &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                        Span::call_site(),
                    )
                })
                .collect::<Vec<_>>();

            let entitlement_bounds = stateful_fields
                .iter()
                .map(|field| {
                    if field.schema.entitlement_fields.is_empty() {
                        return None;
                    }

                    let entitled_field_tys = field
                        .schema
                        .entitlement_fields
                        .iter()
                        .map(|ident| {
                            Ident::new(
                                &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                                Span::call_site(),
                            )
                        })
                        .collect::<Vec<_>>();

                    Some(quote_spanned! { span =>
                        + #(::proto_hal::stasis::Entitled<#entitled_field_tys>)+*
                    })
                })
                .collect::<Vec<_>>();

            let new_entitlement_bounds = stateful_fields
                .iter()
                .map(|field| {
                    if field.schema.entitlement_fields.is_empty() {
                        return None;
                    }

                    let entitled_field_tys = field
                        .schema
                        .entitlement_fields
                        .iter()
                        .map(|ident| {
                            format_ident!(
                                "New{}",
                                &inflector::cases::pascalcase::to_pascal_case(&ident.to_string(),),
                            )
                        })
                        .collect::<Vec<_>>();

                    Some(quote_spanned! { span =>
                        + #(::proto_hal::stasis::Entitled<#entitled_field_tys>)+*
                    })
                })
                .collect::<Vec<_>>();

            body.extend(quote_spanned! { span =>
                pub type Reset = Register<
                    #(
                        #stateful_field_idents::Reset,
                    )*
                >;

                pub struct StateBuilder<#(#stateful_field_tys,)*> {
                    #(
                        pub(crate) #stateful_field_idents: core::marker::PhantomData<#stateful_field_tys>,
                    )*
                }

                impl<#(#stateful_field_tys,)*> StateBuilder<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub(crate) unsafe fn conjure() -> Self {
                        Self {
                            #(
                                #stateful_field_idents: core::marker::PhantomData,
                            )*
                        }
                    }

                    pub fn finish(self) -> Register<#(#stateful_field_tys,)*>
                    where
                        Self: ::proto_hal::macro_utils::AsRegister,
                    {
                        #[allow(unused_parens)]
                        let reg_value = #(
                            ((#writable_stateful_field_tys::RAW as u32) << #writable_stateful_field_idents::OFFSET)
                        )|*;

                        // SAFETY: assumes the proc macro implementation is sound
                        // and that the peripheral description is accurate
                        unsafe {
                            core::ptr::write_volatile((super::BASE_ADDR + OFFSET) as *mut u32, reg_value);
                        }

                        // SAFETY: `self` is destroyed
                        Register {
                            #(
                                #stateful_field_idents: unsafe { #stateful_field_tys::conjure() },
                            )*
                        }
                    }
                }

                impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub fn transition<#(#new_stateful_field_tys,)*>(self) -> Register<#(#new_stateful_field_tys,)*>
                    where
                        #(
                            #new_stateful_field_tys: #stateful_field_idents::State #new_entitlement_bounds,
                        )*
                    {
                        // SAFETY: `self` is destroyed
                        unsafe { StateBuilder::conjure() }.finish()
                    }

                    pub fn build_state(self) -> StateBuilder<#(#stateful_field_tys,)*> {
                        // SAFETY: `self` is destroyed
                        unsafe { StateBuilder::conjure() }
                    }
                }

                impl<#(#stateful_field_tys,)*> ::proto_hal::macro_utils::AsBuilder for Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    type Builder = StateBuilder<#(#stateful_field_tys,)*>;
                }

                impl<#(#stateful_field_tys,)*> ::proto_hal::macro_utils::AsRegister for StateBuilder<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State #entitlement_bounds,
                    )*
                {
                    type Register = Register<#(#stateful_field_tys,)*>;
                }

                impl<#(#stateful_field_tys,)*> Into<StateBuilder<#(#stateful_field_tys,)*>> for Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    fn into(self) -> StateBuilder<#(#stateful_field_tys,)*> {
                        self.build_state()
                    }
                }

                impl<#(#stateful_field_tys,)*> Into<Register<#(#stateful_field_tys,)*>> for StateBuilder<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                    Self: ::proto_hal::macro_utils::AsRegister,
                {
                    fn into(self) -> Register<#(#stateful_field_tys,)*> {
                        self.finish()
                    }
                }
            });

            for (i, field) in stateful_fields.iter().enumerate() {
                let ident = &field.ident;
                let field_state_builder_ty = format_ident!(
                    "{}StateBuilder",
                    &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                );

                let prev_field_tys = stateful_field_tys.get(..i).unwrap();
                let next_field_tys = stateful_field_tys.get(i + 1..).unwrap();

                let state_tys = field
                    .schema
                    .states
                    .iter()
                    .map(|state| state.ident.clone())
                    .collect::<Vec<_>>();
                let state_accessor_idents = state_tys
                    .iter()
                    .map(|ident| {
                        Ident::new(
                            &inflector::cases::snakecase::to_snake_case(&ident.to_string()),
                            Span::call_site(),
                        )
                    })
                    .collect::<Vec<_>>();

                for state in &field.schema.states {
                    if state.entitlement_fields.is_empty() {
                        let state_ty = &state.ident;

                        body.extend(quote_spanned! { span =>
                            unsafe impl<T> ::proto_hal::stasis::Entitled<T> for #ident::#state_ty {}
                        });
                    }
                }

                if field.access.is_write() {
                    body.extend(quote_spanned! { span =>
                        impl<#(#stateful_field_tys,)*> StateBuilder<#(#stateful_field_tys,)*>
                        where
                            #(
                                #stateful_field_tys: #stateful_field_idents::State,
                            )*
                        {
                            pub fn #ident(self) -> #field_state_builder_ty<#(#stateful_field_tys,)*> {
                                unsafe { core::mem::transmute(()) }
                            }
                        }

                        pub struct #field_state_builder_ty<#(#stateful_field_tys,)*> {
                            #(
                                #stateful_field_idents: core::marker::PhantomData<#stateful_field_tys>,
                            )*
                        }

                        impl<#(#stateful_field_tys,)*> #field_state_builder_ty<#(#stateful_field_tys,)*>
                        where
                            #(
                                #stateful_field_tys: #stateful_field_idents::State,
                            )*
                        {
                            pub fn generic<S>(self) -> StateBuilder<#(#prev_field_tys,)* S, #(#next_field_tys,)*>
                            where
                                S: #ident::State,
                            {
                                // SAFETY: `self` is destroyed
                                unsafe { StateBuilder::conjure() }
                            }

                            // pub fn dynamic(self, state: #ident::States) -> StateBuilder<#(#prev_field_tys,)* #ident::States, #(#next_field_tys,)*> {
                            //     todo!()
                            // }
                        }
                    });

                    for (ty, accessor) in state_tys.iter().zip(state_accessor_idents) {
                        body.extend(quote_spanned! { span =>
                            impl<#(#stateful_field_tys,)*> #field_state_builder_ty<#(#stateful_field_tys,)*>
                            where
                                #(
                                    #stateful_field_tys: #stateful_field_idents::State,
                                )*
                            {
                                pub fn #accessor(self) -> StateBuilder<#(#prev_field_tys,)* #ident::#ty, #(#next_field_tys,)*>
                                where
                                    #ident::#ty: #ident::State,
                                {
                                    self.generic()
                                }
                            }
                        });
                    }
                }
            }
        }

        // reader
        {
            let readable_stateless_fields = stateless_fields
                .iter()
                .filter(|field| field.access.is_read())
                .collect::<Vec<_>>();

            let readable_stateless_field_idents = readable_stateless_fields
                .iter()
                .map(|field| &field.ident)
                .collect::<Vec<_>>();

            let value_tys = readable_stateless_fields
                .iter()
                .map(|field| {
                    let ident = format_ident!(
                        "u{}",
                        Index {
                            index: field.schema.width as _,
                            span: Span::call_site(),
                        }
                    );

                    match field.schema.width {
                        1 => parse_quote! { bool },
                        8 | 16 | 32 => {
                            parse_quote! { #ident }
                        }
                        _ => {
                            parse_quote! { ::proto_hal::macro_utils::arbitrary_int::#ident }
                        }
                    }
                })
                .collect::<Vec<Path>>();

            if !readable_stateless_fields.is_empty() {
                body.extend(quote_spanned! { span =>
                    pub struct Reader {
                        value: ::proto_hal::macro_utils::RegisterValue,
                    }

                    impl Reader {
                        const fn new(value: u32) -> Self {
                            Self {
                                value: ::proto_hal::macro_utils::RegisterValue::new(value),
                            }
                        }

                        #(
                            pub fn #readable_stateless_field_idents(&self) -> #value_tys {
                                self.value.#value_tys(#readable_stateless_field_idents::OFFSET)
                            }
                        )*
                    }
                });

                body.extend(quote_spanned! { span =>
                        impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                        where
                            #(
                                #stateful_field_tys: #stateful_field_idents::State,
                            )*
                        {
                            pub fn read<T>(&self, f: impl FnOnce(&Reader) -> T) -> T {
                                let reader = Reader::new(
                                    // SAFETY: assumes the proc macro implementation is sound
                                    // and that the peripheral description is accurate
                                    unsafe {
                                        core::ptr::read_volatile((super::BASE_ADDR + OFFSET) as *const u32)
                                    }
                                );

                                f(&reader)
                            }
                        }
                    });
            }
        }

        // writer
        {
            let writable_stateless_fields = stateless_fields
                .iter()
                .filter(|field| field.access.is_write())
                .collect::<Vec<_>>();

            let writable_stateless_field_idents = writable_stateless_fields
                .iter()
                .map(|field| &field.ident)
                .collect::<Vec<_>>();

            let value_tys = writable_stateless_fields
                .iter()
                .map(|field| {
                    let ident = format_ident!(
                        "u{}",
                        Index {
                            index: field.schema.width as _,
                            span: Span::call_site(),
                        }
                    );

                    match field.schema.width {
                        1 => parse_quote! { bool },
                        8 | 16 | 32 => parse_quote! { #ident },
                        _ => parse_quote! { ::proto_hal::macro_utils::arbitrary_int::#ident },
                    }
                })
                .collect::<Vec<Path>>();

            if !writable_stateless_fields.is_empty() {
                body.extend(quote_spanned! { span =>
                    pub struct Writer {
                        value: u32,
                    }

                    impl Writer {
                        const fn new() -> Self {
                            Self {
                                value: 0,
                            }
                        }

                        #(
                            pub fn #writable_stateless_field_idents(&mut self, value: #value_tys) -> &mut Self {
                                self.value |= (value as u32) << #writable_stateless_field_idents::OFFSET;
                                self
                            }
                        )*
                    }
                });

                body.extend(quote_spanned! { span =>
                    impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                    where
                        #(
                            #stateful_field_tys: #stateful_field_idents::State,
                        )*
                    {
                        pub fn write(&self, f: impl FnOnce(&mut Writer) -> &mut Writer) {
                            let mut writer = Writer::new();

                            f(&mut writer);

                            // SAFETY: assumes the proc macro implementation is sound
                            // and that the peripheral description is accurate
                            unsafe {
                                core::ptr::write_volatile((super::BASE_ADDR + OFFSET) as *mut u32, writer.value);
                            }
                        }
                    }
                });
            }
        }

        tokens.extend(quote_spanned! { span =>
            pub mod #ident {
                #body
            }
        });
    }
}
