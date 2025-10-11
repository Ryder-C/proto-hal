use std::collections::HashMap;

use ir::structures::{
    field::{Field, Numericity},
    hal::Hal,
    peripheral::Peripheral,
    register::Register,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{Expr, Ident, Path, spanned::Spanned};

use crate::codegen::macros::{Args, Override, RegisterArgs, get_field, get_register};

fn parse<'hal>(
    args: &Args,
    model: &'hal Hal,
) -> (
    HashMap<
        Path,
        (
            &'hal Peripheral,
            &'hal Register,
            HashMap<Ident, &'hal Field>,
        ),
    >,
    Vec<syn::Error>,
) {
    let mut out = HashMap::new();
    let mut errors = Vec::new();

    let (registers, e) = parse_registers(args, model);
    errors.extend(e);

    for (register_ident, (register_args, peripheral, register)) in registers {
        let (f, e) = parse_fields(register_args, register);
        errors.extend(e);

        out.insert(register_ident.clone(), (peripheral, register, f));
    }

    (out, errors)
}

/// Lookup peripherals and registers from the model given provided register paths.
fn parse_registers<'args, 'hal>(
    args: &'args Args,
    model: &'hal Hal,
) -> (
    HashMap<Path, (&'args RegisterArgs, &'hal Peripheral, &'hal Register)>,
    Vec<syn::Error>,
) {
    let mut registers = HashMap::new();
    let mut errors = Vec::new();

    if args.registers.is_empty() {
        errors.push(syn::Error::new(
            Span::call_site(),
            "at least one register must be specified",
        ));
    }

    for register_args in &args.registers {
        let mut parse_register = || {
            let (peripheral, register) = get_register(&register_args.path, model)?;

            if let Some(..) = registers.insert(
                register_args.path.clone(),
                (register_args, peripheral, register),
            ) {
                Err(syn::Error::new_spanned(
                    &register_args.path,
                    "register already specified",
                ))?
            }

            Ok(())
        };

        if let Err(e) = parse_register() {
            errors.push(e);
        }
    }

    (registers, errors)
}

/// Lookup fields from a register given provided field idents.
fn parse_fields<'args, 'hal>(
    register_args: &'args RegisterArgs,
    register: &'hal Register,
) -> (HashMap<Ident, &'hal Field>, Vec<syn::Error>) {
    let mut fields = HashMap::new();
    let mut errors = Vec::new();

    if register_args.fields.is_empty() {
        errors.push(syn::Error::new(
            Span::call_site(),
            "at least one field must be specified",
        ));
    }

    for field_args in &register_args.fields {
        let mut parse_field = || {
            let field = get_field(&field_args.ident, register)?;

            if let Some(..) = fields.insert(field_args.ident.clone(), field) {
                Err(syn::Error::new_spanned(
                    &field_args.ident,
                    "field already specified",
                ))?
            }

            Ok(())
        };

        if let Err(e) = parse_field() {
            errors.push(e);
        }
    }

    (fields, errors)
}

fn unique_register_ident(peripheral: &Peripheral, register: &Register) -> Ident {
    format_ident!("{}_{}", peripheral.module_name(), register.module_name(),)
}

pub fn read_untracked(model: &Hal, tokens: TokenStream) -> TokenStream {
    let args = match syn::parse2::<Args>(tokens) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let (parsed, errors) = parse(&args, &model);

    let suggestions = if errors.is_empty() {
        None
    } else {
        let imports = args
            .registers
            .iter()
            .map(|register| {
                let path = &register.path;
                let fields = register.fields.iter().map(|field| &field.ident);

                let span = path.span();

                quote_spanned! { span =>
                    use #path::{#(
                        #fields as _,
                    )*};
                }
            })
            .collect::<TokenStream>();
        Some(imports)
    };

    let errors = {
        let errors = errors.into_iter().map(|e| e.to_compile_error());

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

    let returns = parsed.iter().flat_map(|(path, (.., fields))| {
        fields
            .iter()
            .filter_map(|(ident, field)| {
                Some(match field.access.get_read()?.numericity {
                    Numericity::Numeric => quote! { u32 },
                    Numericity::Enumerated { .. } => quote! {
                        #path::#ident::ReadVariant
                    },
                })
            })
            .collect::<Vec<_>>()
    });

    let reg_idents = parsed
        .values()
        .map(|(peripheral, register, ..)| unique_register_ident(peripheral, register))
        .collect::<Vec<_>>();

    let addrs = parsed
        .iter()
        .filter_map(|(path, (peripheral, register, ..))| {
            let register_offset = register.offset as usize;

            Some(
                if let Some(base_addr) = overridden_base_addrs.get(&peripheral.module_name()) {
                    quote! { (#base_addr + #register_offset) }
                } else {
                    quote! { #path::ADDR }
                },
            )
        });

    let values = parsed.iter().map(|(path, (peripheral, register, fields))| {
        fields
            .iter()
            .filter_map(|(ident, field)| {
                let reg = unique_register_ident(peripheral, register);

                let mask = u32::MAX >> (32 - field.width);

                let value = quote! {
                    (#reg >> #path::#ident::OFFSET) & #mask
                };

                Some(match field.access.get_read()?.numericity {
                    Numericity::Numeric => value,
                    Numericity::Enumerated { .. } => quote! {
                        unsafe { #path::#ident::ReadVariant::from_bits(#value) }
                    },
                })
            })
            .collect::<Vec<_>>()
    });

    quote! {
        #suggestions
        #errors

        {
            unsafe fn gate() -> (#(#returns),*) {
                #(
                    let #reg_idents = unsafe {
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
