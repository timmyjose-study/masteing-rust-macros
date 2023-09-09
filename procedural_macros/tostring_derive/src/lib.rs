use proc_macro::TokenStream;

#[proc_macro_derive(ToString)]
pub fn to_string(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote::quote! {
        impl std::string::ToString for #name {
            fn to_string(&self) -> String {
                let mut res = String::new();

                #(
                    res.push_str(&self.#ast.field.to_string());
                )*
                res
            }
        }
    };

    gen.into()
}
