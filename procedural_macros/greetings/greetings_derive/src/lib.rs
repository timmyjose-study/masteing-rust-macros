use greetings::Greet;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Greet)]
pub fn greet(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl Greet for #name {
            fn greet(&self) {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    gen.into()
}
