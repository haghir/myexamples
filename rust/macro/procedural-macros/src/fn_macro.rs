use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::{quote, format_ident};

pub(crate) fn fn_macro_body(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let name = &item.sig.ident;
    let new_name = format_ident!("double_{}", name);

    quote! {
        #item
        fn #new_name() -> i32 {
            #name() * #name()
        }
    }.into()
}
