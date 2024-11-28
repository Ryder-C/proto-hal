use std::collections::HashMap;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{extra::DelimSpan, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    meta::ParseNestedMeta,
    parse::Parse,
    parse2, parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Colon, Const, Eq, For, Gt, Impl, Lt, Paren, PathSep, Semi, Struct, Unsafe},
    Attribute, Block, Expr, ExprArray, ExprLit, Field, FieldMutability, Fields, FieldsNamed,
    GenericParam, Generics, Ident, ImplItem, ImplItemConst, Item, ItemConst, ItemEnum, ItemImpl,
    ItemMod, ItemStruct, Lit, LitInt, Meta, MetaList, Path, PathArguments, PathSegment, Token,
    Type, TypeParam, TypeTuple, Visibility,
};

#[derive(Debug, FromMeta)]
struct BlockArgs {
    base_addr: LitInt,
    infer_offsets: bool,
    entitlements: PathArray,
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct RegisterArgs {
    #[darling(default)]
    infer_offsets: bool,
}

#[derive(Debug, Clone, Default)]
struct PathArray {
    elems: Vec<Path>,
}

impl FromMeta for PathArray {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        let arr = ExprArray::from_meta(item)?;

        Ok(Self {
            elems: arr
                .elems
                .iter()
                .cloned()
                .map(|expr| {
                    if let Expr::Path(path) = expr {
                        Ok(path.path)
                    } else {
                        Err(darling::Error::custom("expected path").with_span(&expr))
                    }
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl PathArray {
    const fn new() -> Self {
        Self { elems: Vec::new() }
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
struct Access {
    entitlements: PathArray,
    effect: Option<Meta>,
}

#[derive(Debug, Clone, Default, FromMeta)]
struct FieldArgs {
    offset: Option<u8>,
    width: u8,
    read: Option<Access>,
    write: Option<Access>,
}

#[derive(Debug, Clone)]
struct FieldInfo {
    args: FieldArgs,
    ident: Ident,
    reset: Ident,
}

#[derive(Debug)]
struct RegFieldInfo {
    args: FieldArgs,
    ident: Ident,
    ty: Ident,
    gen_ty: Ident,
}

#[derive(Debug, FromMeta)]
struct BlockRegArgs {
    offset: u8,
}

#[derive(Debug)]
struct BlockRegInfo {
    args: BlockRegArgs,
    ident: Ident,
    gen_ty: Ident,
}

#[derive(Debug, FromMeta)]
struct Reset;

#[derive(Debug)]
struct StateInfo {
    args: StateArgs,
    ident: Ident,
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct StateArgs {
    #[darling(default)]
    reset: bool,
    entitlements: PathArray,
}

#[derive(Debug, Default, FromMeta)]
struct ValueArgs;

#[derive(Debug)]
struct RegisterInfo {
    args: RegisterArgs,
    fields: Vec<RegFieldInfo>,
}

struct BlockInfo {
    args: BlockArgs,
    ident: Ident,
    generics: Generics,
    registers: Vec<BlockRegInfo>,
}

struct GenPrimitiveModsArgs {
    ty: Type,
    comma: Token![,],
    width: LitInt,
}

impl Parse for GenPrimitiveModsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            width: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn gen_primitive_mods(item: TokenStream) -> TokenStream {
    let GenPrimitiveModsArgs {
        ty,
        comma: _comma,
        width,
    } = parse_macro_input!(item as GenPrimitiveModsArgs);

    let mod_ident = Ident::new(
        inflector::cases::snakecase::to_snake_case(ty.to_token_stream().to_string().as_str())
            .as_str(),
        Span::call_site(),
    );

    quote! {
        pub mod #mod_ident {
            pub const WIDTH: u8 = #width;
        }
    }
    .into()
}

struct SynErrorCombinator {
    errors: Vec<syn::Error>,
}

impl SynErrorCombinator {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(&mut self, error: syn::Error) {
        self.errors.push(error);
    }

    // TODO: better name
    pub fn try_maybe_then<F, T, E>(&mut self, result: Result<T, E>, mut f: F)
    where
        E: Into<syn::Error>,
        F: FnMut(T) -> Result<(), E>,
    {
        match result {
            Ok(t) => {
                if let Err(e) = f(t) {
                    self.errors.push(e.into());
                }
            }
            Err(e) => {
                self.errors.push(e.into());
            }
        }
    }

    // TODO: better name
    pub fn maybe_then<F, T, E>(&mut self, result: Result<T, E>, mut f: F)
    where
        E: Into<syn::Error>,
        F: FnMut(T),
    {
        self.try_maybe_then(result, |t| {
            f(t);

            Ok(())
        });
    }

    // TODO: better name
    pub fn maybe<T, E>(&mut self, result: Result<T, E>)
    where
        E: Into<syn::Error>,
    {
        self.maybe_then(result, |_| {});
    }

    pub fn coalesce(self) -> Result<(), syn::Error> {
        if let Some(error) = self.errors.iter().cloned().reduce(|mut acc, e| {
            acc.combine(e);
            acc
        }) {
            Err(error)?
        } else {
            Ok(())
        }
    }
}

fn process_value(args: ValueArgs, s: &mut ItemStruct) -> Result<(), syn::Error> {
    Ok(())
}

fn process_state(args: StateArgs, s: &mut ItemStruct) -> Result<(), syn::Error> {
    let Fields::Unit = s.fields else {
        Err(syn::Error::new_spanned(
            s.fields.clone(),
            "state must be a unit struct",
        ))?
    };

    // Q: gross that this is technically fallible
    // but the correct way is too verbose.
    // is there a better strategy?
    s.fields = Fields::Named(
        parse2(quote! {
            {
                sealed: (),
            }
        })
        .unwrap(),
    );

    s.vis = Visibility::Public(Token![pub](s.span()));

    Ok(())
}

fn process_field(args: FieldArgs, module: &mut ItemMod) -> Result<FieldInfo, syn::Error> {
    let items = &mut module.content.as_mut().expect("module cannot be empty").1;

    let mut error_combinator = SynErrorCombinator::new();

    let mut reset = None;

    items.iter_mut().for_each(|item| {
        let Item::Struct(s) = item else { return };

        // 1. try to extract state annotation args
        let mut state_args = None;
        // 1. (cont.) or value annotation args
        let mut value_args = None;

        s.attrs = s
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if attr.path().is_ident("state") {
                    error_combinator.try_maybe_then(StateArgs::from_meta(&attr.meta), |args| {
                        // store reset and validate single occurance
                        if args.reset {
                            if reset.is_none() {
                                reset = Some(s.ident.clone());
                            } else {
                                Err(syn::Error::new_spanned(attr, "reset is already specified"))?
                            }
                        }

                        state_args.replace(args);

                        Ok(())
                    });

                    false
                } else if attr.path().is_ident("value") {
                    error_combinator.maybe_then(ValueArgs::from_meta(&attr.meta), |args| {
                        value_args.replace(args);
                    });

                    false
                } else {
                    true
                }
            })
            .collect();

        match (state_args, value_args) {
            (Some(_), Some(_)) => error_combinator.push(syn::Error::new(
                Span::call_site(),
                "state and value are mutually exclusive",
            )),
            (Some(args), _) => {
                // 2. pass the module over to the state parser
                error_combinator.maybe(process_state(args, s));
            }
            (_, Some(args)) => {
                // 2. (cont.) pass the module over to the value parser
                error_combinator.maybe(process_value(args, s));
            }
            (_, _) => {}
        }
    });

    error_combinator.coalesce()?;

    let Some(reset) = reset else {
        Err(syn::Error::new_spanned(module, "reset must be specified"))?
    };

    Ok(FieldInfo {
        args,
        ident: module.ident.clone(),
        reset,
    })
}

fn process_register(args: RegisterArgs, module: &mut ItemMod) -> Result<(), syn::Error> {
    let items = &mut module.content.as_mut().expect("module cannot be empty").1;

    let mut error_combinator = SynErrorCombinator::new();

    items.iter_mut().for_each(|item| {
        let Item::Mod(inner_mod) = item else { return };

        // 1. try to extract field annotation args
        let mut field_args = None;

        inner_mod.attrs = inner_mod
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if attr.path().is_ident("field") {
                    error_combinator.maybe_then(FieldArgs::from_meta(&attr.meta), |args| {
                        field_args.replace(args);
                    });

                    false
                } else {
                    true
                }
            })
            .collect();

        let Some(field_args) = field_args else {
            return;
        };

        // 2. pass the module over to the field parser
        error_combinator.maybe(process_field(field_args, inner_mod));
    });

    error_combinator.coalesce()?;

    Ok(())
}

fn process_block(args: BlockArgs, module: &mut ItemMod) -> Result<(), syn::Error> {
    let items = &mut module.content.as_mut().expect("module cannot be empty").1;

    let mut error_combinator = SynErrorCombinator::new();

    items.iter_mut().for_each(|item| {
        let Item::Mod(inner_mod) = item else { return };

        // 1. try to extract register annotation args
        let mut register_args = None;

        inner_mod.attrs = inner_mod
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if attr.path().is_ident("register") {
                    error_combinator.maybe_then(RegisterArgs::from_meta(&attr.meta), |args| {
                        register_args.replace(args);
                    });

                    false
                } else {
                    true
                }
            })
            .collect();

        let Some(register_args) = register_args else {
            return;
        };

        // 2. pass the module over to the register parser
        error_combinator.maybe(process_register(register_args, inner_mod));
    });

    error_combinator.coalesce()?;

    Ok(())
}

fn block_inner(args: TokenStream, item: TokenStream) -> Result<TokenStream2, syn::Error> {
    let block_args = BlockArgs::from_list(&NestedMeta::parse_meta_list(args.into())?)?;

    let mut module = parse2::<ItemMod>(item.into())?;

    process_block(block_args, &mut module)?;

    Ok(module.to_token_stream())
}

#[proc_macro_attribute]
pub fn block(args: TokenStream, item: TokenStream) -> TokenStream {
    match block_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
