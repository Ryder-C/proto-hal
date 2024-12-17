use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use structures::block::{BlockArgs, BlockSpec};
use syn::{parse2, ItemMod};

mod access;
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

    Ok(quote! {
        #block
    })
}

#[proc_macro_attribute]
pub fn block(args: TokenStream, item: TokenStream) -> TokenStream {
    match block_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
