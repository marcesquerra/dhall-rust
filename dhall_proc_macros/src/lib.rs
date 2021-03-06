#![doc(html_root_url = "https://docs.rs/dhall_proc_macros/0.1.0")]
//! This crate contains the code-generation primitives for the [dhall-rust][dhall-rust] crate.
//! This is highly unstable and breaks regularly; use at your own risk.
//!
//! [dhall-rust]: https://github.com/Nadrieril/dhall-rust

extern crate proc_macro;

mod derive;

use proc_macro::TokenStream;

#[proc_macro_derive(StaticType)]
pub fn derive_static_type(input: TokenStream) -> TokenStream {
    derive::derive_static_type(input)
}
