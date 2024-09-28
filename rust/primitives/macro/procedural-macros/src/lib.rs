mod derive_macro;
mod attr_macro;
mod fn_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(DeriveMacro)]
pub fn derive_macro(item: TokenStream) -> TokenStream {
    derive_macro::derive_macro_body(item)
}

#[proc_macro_attribute]
pub fn attr_macro(args: TokenStream, item: TokenStream) -> TokenStream {
    attr_macro::attr_macro_body(args, item)
}

#[proc_macro]
pub fn fn_macro(item: TokenStream) -> TokenStream {
    fn_macro::fn_macro_body(item)
}
