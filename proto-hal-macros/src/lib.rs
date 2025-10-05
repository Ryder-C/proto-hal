#[proc_macro]
pub fn generate_macros(
    #[expect(unused)] tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proto_hal_build::codegen::macros::reexports().into()
}

#[proc_macro]
pub fn scaffolding(#[expect(unused)] tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proto_hal_build::codegen::macros::scaffolding().into()
}
