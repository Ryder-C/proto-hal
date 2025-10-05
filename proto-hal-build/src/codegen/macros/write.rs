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
use syn::{
    Expr, ExprLit, ExprPath, Ident, Lit, LitInt, Path, Token, braced,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Colon, Comma},
};

#[derive(Debug)]
struct WriteArgs {
    registers: Vec<RegisterArgs>,
}

impl Parse for WriteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut registers = Vec::new();

        while !input.is_empty() {
            registers.push(input.parse()?);
        }

        Ok(Self { registers })
    }
}

#[derive(Debug)]
struct RegisterArgs {
    register_path: Path,
    brace_token: Brace,
    fields: Punctuated<FieldArgs, Comma>,
}

impl Parse for RegisterArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let register_path = input.parse()?;
        let block;
        let brace_token = braced!(block in input);

        let fields = block.parse_terminated(Parse::parse, Comma)?;

        Ok(Self {
            register_path,
            brace_token,
            fields,
        })
    }
}

#[derive(Debug)]
struct FieldArgs {
    ident: Ident,
    colon_token: Colon,
    binding: BindingArgs,
    transition: Option<TransitionArgs>,
}

impl Parse for FieldArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let binding = input.parse()?;
        let transition = if input.peek(Token![=>]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            ident,
            colon_token,
            binding,
            transition,
        })
    }
}

#[derive(Debug)]
struct BindingArgs {
    reference: Option<(Token![&], Option<Token![mut]>)>,
    ident: Ident,
}

impl Parse for BindingArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let reference = if input.peek(Token![&]) {
            Some((
                input.parse()?,
                if input.peek(Token![mut]) {
                    Some(input.parse()?)
                } else {
                    None
                },
            ))
        } else {
            None
        };

        let ident = input.parse()?;

        Ok(Self { reference, ident })
    }
}

#[derive(Debug)]
struct TransitionArgs {
    fat_arrow_token: Token![=>],
    state: StateArgs,
}

impl Parse for TransitionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fat_arrow_token = input.parse()?;
        let state = input.parse()?;

        Ok(Self {
            fat_arrow_token,
            state,
        })
    }
}

#[derive(Debug)]
enum StateArgs {
    Path(Path),
    Lit(LitInt),
}

impl Parse for StateArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(match input.parse()? {
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit), ..
            }) => Self::Lit(lit),
            Expr::Path(ExprPath { path, .. }) => Self::Path(path),
            other => Err(syn::Error::new_spanned(
                other,
                "expected path or integer literal",
            ))?,
        })
    }
}

impl WriteArgs {
    fn validate(&self, model: &Hal) -> Result<(), Vec<syn::Error>> {
        let mut diagnostics = Vec::new();

        for register in &self.registers {
            if let Err(e) = register.validate(model) {
                diagnostics.extend(e);
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

impl RegisterArgs {
    fn validate(&self, model: &Hal) -> Result<(), Vec<syn::Error>> {
        let mut diagnostics = Vec::new();

        let register = Self::get_register(&self.register_path, model).map_err(|e| vec![e])?;

        for field in &self.fields {
            if let Err(e) = field.validate(register) {
                diagnostics.extend(e);
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    fn get_register<'a>(path: &Path, model: &'a Hal) -> Result<&'a Register, syn::Error> {
        let mut segments = path.segments.iter().rev();

        let Some(register_ident) = segments.next().map(|segment| &segment.ident) else {
            Err(syn::Error::new_spanned(path, "expected register ident"))?
        };
        let Some(peripheral_ident) = segments.next().map(|segment| &segment.ident) else {
            Err(syn::Error::new_spanned(path, "expected peripheral ident"))?
        };

        let peripheral = model
            .peripherals
            .get(peripheral_ident)
            .ok_or(syn::Error::new_spanned(
                peripheral_ident,
                format!("peripheral \"{peripheral_ident}\" does not exist"),
            ))?;

        let register = peripheral
            .registers
            .get(register_ident)
            .ok_or(syn::Error::new_spanned(
                peripheral_ident,
                format!("register \"{register_ident}\" does not exist in peripheral \"{peripheral_ident}\""),
            ))?;

        // TODO: show some peripherals the register *was* found in?

        Ok(register)
    }
}

impl FieldArgs {
    fn validate(&self, register: &Register) -> Result<(), Vec<syn::Error>> {
        if !register.fields.contains_key(&self.ident) {
            Err(vec![syn::Error::new_spanned(
                &self.ident,
                format!(
                    "field \"{}\" does not exist in register \"{}\"",
                    self.ident, register.ident
                ),
            )])?

            // TODO: show some registers the field *was* found in?
        }

        Ok(())
    }
}

pub fn write(model: Hal, args: WriteArgs) -> TokenStream {
    quote! {
        {
            fn gate() {
                // unsafe { ::core::ptr::write_volatile(#addr as *mut u32, #reg) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod parsing {
        use quote::quote;

        use crate::codegen::macros::write::{
            BindingArgs, FieldArgs, RegisterArgs, StateArgs, WriteArgs,
        };

        fn get_register(write_args: &WriteArgs, ident: impl AsRef<str>) -> &RegisterArgs {
            let ident = ident.as_ref();

            write_args
                .registers
                .iter()
                .find(|register| {
                    register
                        .register_path
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

        fn get_binding(field_args: &FieldArgs, ident: impl AsRef<str>) -> &BindingArgs {
            assert_eq!(field_args.binding.ident, ident.as_ref());

            &field_args.binding
        }

        #[test]
        fn foo() {
            let tokens = quote! {
                foo::bar {
                    baz: &my_baz,
                }
            };

            let parsed = syn::parse2::<WriteArgs>(tokens).expect("parsing should succeed");
            let baz = get_field(get_register(&parsed, "bar"), "baz");
            let binding = get_binding(baz, "my_baz");

            assert!(
                binding
                    .reference
                    .is_some_and(|reference| reference.1.is_none()),
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

            let parsed = syn::parse2::<WriteArgs>(tokens).expect("parsing should succeed");
            let func = get_field(get_register(&parsed, "csr"), "func");
            let binding = get_binding(func, "my_func");

            assert!(
                binding.reference.is_none(),
                "expected func binding to have no reference"
            );

            let StateArgs::Path(path) = &func
                .transition
                .as_ref()
                .expect("expected func transition to be present")
                .state
            else {
                panic!("expected func target state to be a path")
            };

            assert!(
                path.is_ident("Sqrt"),
                "expected func target state to be \"Sqrt\""
            );
        }
    }
}
