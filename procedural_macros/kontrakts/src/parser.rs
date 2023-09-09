use proc_macro2::TokenStream;

pub type Ast = syn::ItemFn;

pub(crate) fn parse(args: TokenStream, item: TokenStream) -> Ast {
    todo!()
}

#[cfg(test)]
mod tests {}
