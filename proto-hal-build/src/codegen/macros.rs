mod mask;
mod modify;
mod modify_in_place;
mod modify_in_place_with_cs;
mod modify_untracked;
mod modify_untracked_with_cs;
mod modify_with_cs;
mod read;
mod read_untracked;
mod unmask;
mod write;
mod write_from_reset_untracked;
mod write_from_zero_untracked;
mod write_in_place;

use ir::structures::hal::Hal;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;
pub use write::write;

/// Generate and emit HAL code for use.
///
/// *Note: This function is intended to be called in the "out" phase of synthesis.*
pub fn generate(hal: &Hal) {
    super::generate(hal, |hal| {
        Ok([("macros.rs".to_string(), reexports(hal))].into())
    });
}

fn reexports(hal: &Hal) -> String {
    let idents = vec!["write"]
        .into_iter()
        .map(|name| Ident::new(name, Span::call_site()));

    quote! {
        #(
            #[proc_macro]
            pub fn #idents(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
                ::proto_hal_build::codegen::macros::#idents(::model::generate())
            }
        )*
    }
    .to_string()
}
