use std::collections::HashMap;

use indexmap::IndexMap;
use ir::structures::{
    field::{Field, Numericity},
    hal::Hal,
    peripheral::Peripheral,
    register::Register,
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Expr, Ident, Path, spanned::Spanned};

use crate::codegen::macros::{Args, Override, RegisterArgs, StateArgs, get_field, get_register};

enum Scheme {
    FromZero,
    FromReset,
}

/// A parsed unit of the provided tokens and corresponding model nodes which
/// represents a single register.
struct Parsed<'args, 'hal> {
    peripheral: &'hal Peripheral,
    register: &'hal Register,
    transitions: IndexMap<Ident, (&'hal Field, &'args StateArgs)>,
}

fn parse<'args, 'hal>(
    args: &'args Args,
    model: &'hal Hal,
) -> (IndexMap<Path, Parsed<'args, 'hal>>, Vec<syn::Error>) {
    let mut out = IndexMap::new();
    let mut errors = Vec::new();

    let (registers, e) = parse_registers(args, model);
    errors.extend(e);

    for (register_ident, (register_args, peripheral, register)) in registers {
        let (transitions, e) = parse_fields(register_args, register);
        errors.extend(e);

        out.insert(
            register_ident.clone(),
            Parsed {
                peripheral,
                register,
                transitions,
            },
        );
    }

    (out, errors)
}

/// Lookup peripherals and registers from the model given provided register paths.
fn parse_registers<'args, 'hal>(
    args: &'args Args,
    model: &'hal Hal,
) -> (
    IndexMap<Path, (&'args RegisterArgs, &'hal Peripheral, &'hal Register)>,
    Vec<syn::Error>,
) {
    let mut registers = IndexMap::new();
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

/// Lookup fields from a register given provided field idents and transitions.
fn parse_fields<'args, 'hal>(
    register_args: &'args RegisterArgs,
    register: &'hal Register,
) -> (
    IndexMap<Ident, (&'hal Field, &'args StateArgs)>,
    Vec<syn::Error>,
) {
    let mut transitions = IndexMap::new();
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

            let transition = field_args
                .transition
                .as_ref()
                .map(|transition| &transition.state)
                .ok_or(syn::Error::new_spanned(
                    &field_args.ident,
                    "expected transition",
                ))?;

            if let Some(..) = transitions.insert(field_args.ident.clone(), (field, transition)) {
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

        if let Some(binding) = &field_args.binding {
            errors.push(syn::Error::new_spanned(binding, "no binding is accepted"));
        }
    }

    (transitions, errors)
}

fn validate<'args, 'hal>(parsed: &IndexMap<Path, Parsed<'args, 'hal>>) -> Vec<syn::Error> {
    parsed
        .values()
        .flat_map(|Parsed { transitions, .. }| transitions.iter())
        .filter_map(|(ident, (field, ..))| {
            if field.access.get_write().is_none() {
                Some(syn::Error::new_spanned(
                    ident,
                    format!("field \"{ident}\" is not writable"),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn write_untracked(scheme: Scheme, model: &Hal, tokens: TokenStream) -> TokenStream {
    let args = match syn::parse2::<Args>(tokens) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let mut errors = Vec::new();

    let (parsed, e) = parse(&args, &model);
    errors.extend(e);
    errors.extend(validate(&parsed));

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
                    #[allow(unused_imports)]
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

    let addrs = parsed.iter().map(|(path, parsed)| {
        let register_offset = parsed.register.offset as usize;

        if let Some(base_addr) = overridden_base_addrs.get(&parsed.peripheral.module_name()) {
            quote! { (#base_addr + #register_offset) }
        } else {
            quote! { #path::ADDR }
        }
    });

    let initials = parsed.values().map(|parsed| match scheme {
        Scheme::FromZero => 0,
        Scheme::FromReset => {
            let mask = parsed.register.fields.values().fold(0, |acc, field| {
                acc | ((u32::MAX >> (32 - field.width)) << field.offset)
            });

            parsed.register.reset.unwrap_or(0) & !mask
        }
    });

    let values = parsed.iter().map(|(path, parsed)| {
        parsed
            .transitions
            .iter()
            .map(|(ident, (field, transition))| {
                let value = match (
                    transition,
                    &field
                        .access
                        .get_write()
                        .expect("field should be writable")
                        .numericity,
                ) {
                    (StateArgs::Path(state_path), Numericity::Enumerated { .. }) => {
                        quote! {{
                            #[allow(unused_imports)]
                            use #path::#ident::write::Variant::*;
                            #state_path as u32
                        }}
                    }
                    (StateArgs::Path(state_path), ..) => {
                        quote! {{
                            #state_path as u32
                        }}
                    }
                    (StateArgs::Lit(lit_int), ..) => quote! { #lit_int },
                };

                quote! {
                    #value << #path::#ident::OFFSET
                }
            })
            .collect::<Vec<_>>()
    });

    quote! {
        #suggestions
        #errors

        {
            unsafe fn gate() {
                #(
                    unsafe {
                        ::core::ptr::write_volatile(
                            #addrs as *mut u32,
                            #initials #(
                                | #values
                            )*
                        )
                    };
                )*
            }

            gate()
        }
    }
}

pub fn write_from_zero_untracked(model: &Hal, tokens: TokenStream) -> TokenStream {
    write_untracked(Scheme::FromZero, model, tokens)
}

pub fn write_from_reset_untracked(model: &Hal, tokens: TokenStream) -> TokenStream {
    write_untracked(Scheme::FromReset, model, tokens)
}
