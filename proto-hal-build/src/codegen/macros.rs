mod mask;

mod gates;
mod scaffolding;
mod unmask;

use ir::structures::{field::Field, hal::Hal, peripheral::Peripheral, register::Register};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Expr, ExprLit, ExprPath, Ident, Lit, LitInt, Path, Token, braced,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Comma},
};

pub use gates::{
    read_untracked::read_untracked,
    write::write,
    write_untracked::{write_from_reset_untracked, write_from_zero_untracked},
};
pub use scaffolding::scaffolding;

#[derive(Debug)]
struct Args {
    registers: Vec<RegisterArgs>,
    overrides: Vec<Override>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut registers = Vec::new();
        let mut overrides = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![@]) {
                input.parse::<Token![@]>()?;
                overrides.push(input.parse()?);
            } else {
                registers.push(input.parse()?);
            }
        }

        Ok(Self {
            registers,
            overrides,
        })
    }
}

#[derive(Debug)]
enum Override {
    BaseAddress(Ident, Expr),
}

impl Parse for Override {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        match ident.to_string().as_str() {
            "base_addr" => {
                let peripheral_ident = input.parse()?;
                let addr = input.parse::<Expr>()?;
                Ok(Self::BaseAddress(peripheral_ident, addr))
            }
            other => Err(syn::Error::new_spanned(
                ident,
                format!("unknown override \"{other}\""),
            )),
        }
    }
}

#[derive(Debug)]
struct RegisterArgs {
    path: Path,
    fields: Punctuated<FieldArgs, Comma>,
}

impl Parse for RegisterArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;

        if !input.peek(Brace) {
            return Ok(Self {
                path,
                fields: Default::default(),
            });
        }

        let block;
        braced!(block in input);

        let fields = block.parse_terminated(Parse::parse, Comma)?;

        Ok(Self { path, fields })
    }
}

#[derive(Debug)]
struct FieldArgs {
    ident: Ident,
    binding: Option<Expr>,
    transition: Option<TransitionArgs>,
}

impl Parse for FieldArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;

        let binding = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        let transition = if input.peek(Token![=>]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            ident,
            binding,
            transition,
        })
    }
}

#[derive(Debug)]
struct TransitionArgs {
    state: StateArgs,
}

impl Parse for TransitionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=>]>()?;
        let state = input.parse()?;

        Ok(Self { state })
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

fn get_register<'hal>(
    path: &Path,
    model: &'hal Hal,
) -> Result<(&'hal Peripheral, &'hal Register), syn::Error> {
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
            register_ident,
            format!(
                "register \"{register_ident}\" does not exist in peripheral \"{peripheral_ident}\""
            ),
        ))?;

    // TODO: show some peripherals the register *was* found in?

    Ok((peripheral, register))
}

fn get_field<'a>(ident: &Ident, register: &'a Register) -> syn::Result<&'a Field> {
    register.fields.get(ident).ok_or(syn::Error::new_spanned(
        ident,
        format!(
            "field \"{ident}\" does not exist in register \"{}\"",
            register.module_name()
        ),
    ))
}

pub fn reexports() -> TokenStream {
    let idents = vec!["write", "read_untracked", "write_from_zero_untracked"]
        .into_iter()
        .map(|name| Ident::new(name, Span::call_site()));

    quote! {
        #(
            #[proc_macro]
            pub fn #idents(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
                ::proto_hal_build::codegen::macros::#idents(&::model::generate(), tokens.into()).into()
            }
        )*

        #[proc_macro]
        pub fn scaffolding(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            ::proto_hal_build::codegen::macros::scaffolding().into()
        }
    }
}
