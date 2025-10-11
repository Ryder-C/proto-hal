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

use crate::codegen::macros::{Args, Override, RegisterArgs, StateArgs, get_field, get_register};

/// A parsed unit of the provided tokens and corresponding model nodes which
/// represents a single register.
struct Parsed<'hal> {
    peripheral: &'hal Peripheral,
    register: &'hal Register,
    fields: HashMap<Ident, &'hal Field>,
}

fn parse<'hal>(args: &Args, model: &'hal Hal) -> (HashMap<Path, Parsed<'hal>>, Vec<syn::Error>) {
    let mut out = HashMap::new();
    let mut errors = Vec::new();

    let (registers, e) = parse_registers(args, model);
    errors.extend(e);

    for (register_ident, (register_args, peripheral, register)) in registers {
        let (fields, e) = parse_fields(register_args, register);
        errors.extend(e);

        out.insert(
            register_ident.clone(),
            Parsed {
                peripheral,
                register,
                fields,
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

        if let Some(binding) = &field_args.binding {
            errors.push(syn::Error::new_spanned(binding, "no binding is accepted"));
        }

        if let Some(transition) = &field_args.transition {
            errors.push(syn::Error::new(
                match &transition.state {
                    StateArgs::Path(path) => path.span(),
                    StateArgs::Lit(lit_int) => lit_int.span(),
                },
                "no transition is accepted",
            ));
        }
    }

    (fields, errors)
}

fn validate<'hal>(parsed: &HashMap<Path, Parsed<'hal>>) -> Vec<syn::Error> {
    parsed
        .values()
        .flat_map(|Parsed { fields, .. }| fields.iter())
        .filter_map(|(ident, field)| {
            if field.access.get_read().is_none() {
                Some(syn::Error::new_spanned(
                    ident,
                    format!("field \"{ident}\" is not readable"),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn unique_register_ident(peripheral: &Peripheral, register: &Register) -> Ident {
    format_ident!("{}_{}", peripheral.module_name(), register.module_name(),)
}

pub fn read_untracked(model: &Hal, tokens: TokenStream) -> TokenStream {
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

    let returns = parsed.iter().flat_map(|(path, Parsed { fields, .. })| {
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
        .map(|parsed| unique_register_ident(parsed.peripheral, parsed.register))
        .collect::<Vec<_>>();

    let addrs = parsed.iter().filter_map(|(path, parsed)| {
        let register_offset = parsed.register.offset as usize;

        Some(
            if let Some(base_addr) = overridden_base_addrs.get(&parsed.peripheral.module_name()) {
                quote! { (#base_addr + #register_offset) }
            } else {
                quote! { #path::ADDR }
            },
        )
    });

    let values = parsed.iter().map(|(path, parsed)| {
        parsed
            .fields
            .iter()
            .filter_map(|(ident, field)| {
                let reg = unique_register_ident(parsed.peripheral, parsed.register);

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
