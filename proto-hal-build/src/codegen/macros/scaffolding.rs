use proc_macro2::TokenStream;
use quote::quote;

pub fn scaffolding() -> TokenStream {
    quote! {
        include!(concat!(env!("OUT_DIR"), "/hal.rs"));
        pub use macros::{write};
    }
}
