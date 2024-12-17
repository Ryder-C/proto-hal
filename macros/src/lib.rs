use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use structures::{
    block::{BlockArgs, BlockSpec},
    Spec,
};
use syn::{
    parse::Parse, parse2, parse_macro_input, spanned::Spanned, Attribute, Expr, ExprArray,
    ExprRange, Fields, Ident, Index, Item, ItemMod, ItemStruct, LitInt, Meta, Path, Token, Type,
    Visibility,
};

mod structures;
mod utils;

fn block_inner(args: TokenStream, item: TokenStream) -> Result<TokenStream2, syn::Error> {
    let block_args = BlockArgs::from_list(&NestedMeta::parse_meta_list(args.into())?)?;

    let module = parse2::<ItemMod>(item.into())?;

    let block = BlockSpec::parse(
        module.ident.clone(),
        block_args.clone(),
        utils::extract_items_from(&module)?.iter(),
    )?;

    println!("{:?}", block);

    Ok(quote! {})
}

#[proc_macro_attribute]
pub fn block(args: TokenStream, item: TokenStream) -> TokenStream {
    match block_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
