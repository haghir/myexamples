use std::fs::read_to_string;
use toml::Table;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Ident, Token,
          punctuated::Punctuated,
          parse::{Parse, ParseStream, Result}};
use quote::quote;

// refs: https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
#[derive(Debug)]
struct Args {
    pub(crate) vars: Vec<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

pub(crate) fn attr_macro_body(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);

    let msgs: Table = read_to_string("messages.toml").unwrap().parse().unwrap();
    let key = &args.vars[0].to_string();
    let msg = msgs[key].as_str();

    let item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;

    let opts = item.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            #ident = self.#ident
        }
    });

    quote! {
        #item
        impl #name {
            fn format(&self) -> String {
                format!(#msg, #(#opts),*).to_string()
            }
        }
    }.into()
}
