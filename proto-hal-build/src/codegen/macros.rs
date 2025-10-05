mod mask;
mod modify;
mod modify_in_place;
mod modify_in_place_with_cs;
mod modify_untracked;
mod modify_untracked_with_cs;
mod modify_with_cs;
mod read;
mod read_untracked;
mod scaffolding;
mod unmask;
mod write;
mod write_from_reset_untracked;
mod write_from_zero_untracked;
mod write_in_place;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub use scaffolding::scaffolding;
pub use write::write;

pub fn reexports() -> TokenStream {
    let idents = vec!["write"]
        .into_iter()
        .map(|name| Ident::new(name, Span::call_site()));

    quote! {
        #(
            #[proc_macro]
            pub fn #idents(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
                ::proto_hal_build::codegen::macros::#idents(::model::generate(), tokens.into()).into()
            }
        )*

        #[proc_macro]
        pub fn scaffolding(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            ::proto_hal_build::codegen::macros::scaffolding().into()
        }
    }
}
