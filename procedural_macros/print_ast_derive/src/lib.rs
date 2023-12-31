use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintAST)]
pub fn print_ast(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    eprintln!("{ast:#?}");

    TokenStream::new()
}
