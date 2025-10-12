//! ```ignore
//! use g4::cordic::{self, wdata};
//!
//! write! {
//!     x::y::z::cordic::csr {
//!         func: my_func => Sqrt,
//!         scale: &some_scale,
//!         precision: the_precision => P60,
//!         argsize: &my_arg_size,
//!     }
//!     wdata {
//!         arg: &mut an_arg => 0xdead_beef,
//!     }
//! }
//! ```

use ir::structures::{hal::Hal, register::Register};
use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::macros::{Args, FieldArgs, RegisterArgs, get_register};

fn validate(args: &Args, model: &Hal) -> Result<(), Vec<syn::Error>> {
    let mut diagnostics = Vec::new();

    for register in &args.registers {
        if let Err(e) = validate_register(&register, model) {
            diagnostics.extend(e);
        }
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_register(args: &RegisterArgs, model: &Hal) -> Result<(), Vec<syn::Error>> {
    let mut diagnostics = Vec::new();

    let (.., register) = get_register(&args.path, model).map_err(|e| vec![e])?;

    for field in &args.fields {
        if let Err(e) = validate_field(&field, register) {
            diagnostics.extend(e);
        }
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_field(args: &FieldArgs, register: &Register) -> Result<(), Vec<syn::Error>> {
    if !register.fields.contains_key(&args.ident) {
        Err(vec![syn::Error::new_spanned(
            &args.ident,
            format!(
                "field \"{}\" does not exist in register \"{}\"",
                args.ident, register.ident
            ),
        )])?

        // TODO: show some registers the field *was* found in?
    }

    Ok(())
}

pub fn write(model: &Hal, tokens: TokenStream) -> TokenStream {
    let args = match syn::parse2::<Args>(tokens) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let errors = if let Err(e) = validate(&args, &model) {
        let errors = e.into_iter().map(|e| e.to_compile_error());

        Some(quote! {
            #(
                #errors
            )*
        })
    } else {
        None
    };

    let field_paths = args.registers.iter().flat_map(|register| {
        let path = &register.path;

        if register.fields.is_empty() {
            vec![quote! {
                use #path as _;
            }]
        } else {
            register
                .fields
                .iter()
                .map(|field| {
                    let ident = &field.ident;

                    quote! {
                        use #path::#ident as _;
                    }
                })
                .collect()
        }
    });

    quote! {
        #errors

        {
            fn gate() {
                #(
                    #field_paths
                )*

                // unsafe { ::core::ptr::write_volatile(#addr as *mut u32, #reg) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod parsing {
        use quote::quote;
        use syn::Expr;

        use crate::codegen::macros::{Args, FieldArgs, RegisterArgs, StateArgs};

        fn get_register(write_args: &Args, ident: impl AsRef<str>) -> &RegisterArgs {
            let ident = ident.as_ref();

            write_args
                .registers
                .iter()
                .find(|register| {
                    register
                        .path
                        .segments
                        .last()
                        .expect("register paths should be non-empty")
                        .ident
                        == ident
                })
                .expect(&format!("expected register with ident \"{ident}\""))
        }

        fn get_field(register_args: &RegisterArgs, ident: impl AsRef<str>) -> &FieldArgs {
            let ident = ident.as_ref();

            register_args
                .fields
                .iter()
                .find(|field| field.ident == ident)
                .expect(&format!("expected field with ident \"{ident}\""))
        }

        #[test]
        fn foo() {
            let tokens = quote! {
                foo::bar {
                    baz: &my_baz,
                }
            };

            let parsed = syn::parse2::<Args>(tokens).expect("parsing should succeed");
            let baz = get_field(get_register(&parsed, "bar"), "baz");

            assert!(
                matches!(baz.binding, Some(Expr::Reference(..))),
                "expected binding to be shared reference"
            );

            assert!(
                baz.transition.is_none(),
                "expected transition to not be present"
            );
        }

        #[test]
        fn basic() {
            let tokens = quote! {
                cordic::csr {
                    func: my_func => Sqrt,
                    precision: p => 0x10,
                    scale: &some_scale,
                }
                cordic::wdata {
                    arg: &mut my_arg => 0
                }
            };

            let parsed = syn::parse2::<Args>(tokens).expect("parsing should succeed");
            let func = get_field(get_register(&parsed, "csr"), "func");

            assert!(
                matches!(func.binding, Some(Expr::Path(..))),
                "expected func binding to have no reference"
            );

            let StateArgs::Expr(..) = &func
                .transition
                .as_ref()
                .expect("expected func transition to be present")
                .state
            else {
                panic!("expected func target state to be a path")
            };
        }
    }
}
