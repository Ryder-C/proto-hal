use std::collections::HashMap;

use ir::structures::{
    field::{Field, Numericity},
    hal::Hal,
    peripheral::Peripheral,
    register::Register,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Path};

use crate::codegen::macros::{Args, Override, get_field, get_register};

fn validate<'hal>(
    args: &Args,
    model: &'hal Hal,
) -> (
    HashMap<
        Ident,
        (
            &'hal Peripheral,
            &'hal Register,
            HashMap<Ident, &'hal Field>,
        ),
    >,
    Vec<syn::Error>,
) {
    let mut diagnostics = Vec::new();

    let mut fields = HashMap::new();
    for register_args in &args.registers {
        match get_register(&register_args.path, model) {
            Ok((peripheral, register)) => {
                for field_args in &register_args.fields {
                    match get_field(&field_args.ident, register) {
                        Ok(field) => {
                            fields
                                .entry(format_ident!(
                                    "{}_{}",
                                    peripheral.module_name(),
                                    register.module_name()
                                ))
                                .or_insert((peripheral, register, HashMap::new()))
                                .2
                                .insert(field.module_name(), field);
                        }
                        Err(e) => diagnostics.push(e),
                    }
                }
            }
            Err(e) => {
                diagnostics.push(e);
            }
        }
    }

    (fields, diagnostics)
}

fn register_unique_ident(path: &Path) -> Option<Ident> {
    path.segments
        .iter()
        .map(|segment| &segment.ident)
        .cloned()
        .rev()
        .take(2)
        .reduce(|acc, ident| format_ident!("{ident}_{acc}"))
}

pub fn read_untracked(model: &Hal, tokens: TokenStream) -> TokenStream {
    let args = match syn::parse2::<Args>(tokens) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let (fields, diagnostics) = validate(&args, &model);

    let errors = {
        let errors = diagnostics.into_iter().map(|e| e.to_compile_error());

        quote! {
            #(
                #errors
            )*
        }
    };

    let mut overridden_base_addrs: HashMap<Ident, Expr> = HashMap::new();

    for (ident, expr) in args
        .overrides
        .iter()
        .filter_map(|override_| match override_ {
            Override::BaseAddress(ident, expr) => Some((ident.clone(), expr)),
        })
    {
        overridden_base_addrs.insert(ident, expr.clone());
    }

    let returns = args.registers.iter().flat_map(|register| {
        let register_path = &register.path;

        register
            .fields
            .iter()
            .filter_map(|field| {
                let field = fields
                    .get(&register_unique_ident(&register.path)?)?
                    .2
                    .get(&field.ident)?;

                let ident = field.module_name();

                Some(match field.access.get_read()?.numericity {
                    Numericity::Numeric => quote! { u32 },
                    Numericity::Enumerated { .. } => quote! {
                        #register_path::#ident::ReadVariant
                    },
                })
            })
            .collect::<Vec<_>>()
    });

    let regs = args
        .registers
        .iter()
        .filter_map(|register| register_unique_ident(&register.path))
        .collect::<Vec<_>>();

    let addrs = args.registers.iter().filter_map(|register| {
        let ident = register_unique_ident(&register.path)?;
        let (peripheral, register, ..) = fields.get(&ident)?;

        let register_offset = register.offset as usize;

        Some(
            if let Some(base_addr) = overridden_base_addrs.get(&peripheral.module_name()) {
                quote! { (#base_addr + #register_offset) }
            } else {
                let addr = (peripheral.base_addr + register.offset) as usize;
                quote! { #addr }
            },
        )
    });

    let values = args.registers.iter().map(|register| {
        let register_ident = register_unique_ident(&register.path);
        let register_path = &register.path;

        register
            .fields
            .iter()
            .filter_map(|field| {
                let reg = register_ident.as_ref()?;
                let ident = &field.ident;
                let field = fields.get(reg)?.2.get(&field.ident)?;

                let offset = field.offset;
                let mask = u32::MAX >> (32 - field.width);

                let value = quote! {
                    (#reg >> #offset) & #mask
                };

                Some(match field.access.get_read()?.numericity {
                    Numericity::Numeric => value,
                    Numericity::Enumerated { .. } => quote! {
                        unsafe { #register_path::#ident::ReadVariant::from_bits(#value) }
                    },
                })
            })
            .collect::<Vec<_>>()
    });

    quote! {
        #errors

        {
            unsafe fn gate() -> (#(#returns),*) {
                #(
                    let #regs = unsafe {
                        ::core::ptr::read_volatile(#addrs as *const u32)
                    };
                )*

                (
                    #(#(
                        #values
                    )*),*
                )
            }

            gate()
        }
    }
}
