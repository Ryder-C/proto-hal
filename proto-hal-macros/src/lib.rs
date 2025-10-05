//! ```
//! use g4::cordic::{self, wdata};
//!
//! write! {
//!     cordic::csr: {
//!         func: my_func => Sqrt,
//!         scale: &some_scale,
//!         precision: the_precision => P60,
//!         argsize: &my_arg_size,
//!     }
//!     wdata: {
//!         arg: &mut an_arg => 0xdead_beef,
//!     }
//! }
//! ```
//!
//! Expands to:
//!
//! ```
//! unsafe fn foo<Func, Scale, Precision>(func: Func, scale: &Scale, precision: Precision) ->
//! (
//!     g4::cordic::func::Func<g4::cordic::func::Sqrt>,
//!     g4::cordic::precision::Precision<g4::cordic::precision::P60>,
//! )
//! where
//!     Func: ::proto_hal::stasis::Container<Parent = g4::cordic::csr::func::Field>,
//!     Scale: ::proto_hal::stasis::Container<Parent = g4::cordic::csr::scale::Field>,
//!     Precision: ::proto_hal::stasis::Container<Parent = g4::cordic::csr::precision::Field>,
//! {
//!     let reg = (Func);
//!     ::core::ptr::write_volatile(g4::cordic::csr::ADDR as *mut _, reg);
//!
//!     <g4::cordic::func::Container<g4::cordic::func::Sqrt> as ::proto_hal::stasis::Conjure>::conjure()
//! }
//!
//! foo(my_func, &some_scale)
//! ```

use quote::quote;

struct WriteArgs {
    registers: Vec,
}

#[proc_macro]
pub fn write(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}
