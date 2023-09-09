use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(PrintFields)]
pub fn print_fields(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match &ast.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref named_fields) => {
                for field in named_fields.named.iter() {
                    let ident = field.ident.as_ref().unwrap();

                    match &field.ty {
                        Type::Path(ref path) => {
                            if let Some(ty_ident) = path.path.get_ident() {
                                eprintln!(
                                    "Field name = {:#?}, field type = {:?}",
                                    ident.to_string(),
                                    ty_ident.to_string()
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }

    TokenStream::new()
}
