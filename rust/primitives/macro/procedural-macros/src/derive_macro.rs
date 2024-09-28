use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, ItemStruct, ext::IdentExt};
use quote::quote;

pub(crate) fn derive_macro_body(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let struct_name = &item.ident;
    let (impl_generics, _, where_clause) = &item.generics.split_for_impl();

    let getters = item.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let getter_name: TokenStream2 = format!(
            "get_{}", ident.unraw().to_string()).parse().unwrap();

        quote! {
            pub fn #getter_name(&self) -> #ty {
                self.#ident.clone()
            }
        }
    });

    quote! {
        impl #impl_generics #struct_name #where_clause {
            #(#getters)*
        }
    }.into()
}
