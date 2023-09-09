use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn hello_world(_input: TokenStream) -> TokenStream {
    let gen_code = quote! {
        pub fn hello() {
            println!("Hello, world!");
        }
    };

    gen_code.into()
}
