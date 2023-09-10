use crate::Ast;
use proc_macro_error::{abort, abort_call_site};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Expr, ItemFn,
};

pub struct Model {
    pub preconditions: Vec<Expr>,
    pub item: ItemFn,
}

struct AttributeArgument {
    expr: Expr,
}

impl Parse for AttributeArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _parenthesized = parenthesized!(content in input);

        Ok(AttributeArgument {
            expr: content.parse()?,
        })
    }
}

pub fn analyze(ast: Ast) -> Model {
    let mut preconditions = Vec::new();

    let mut item = ast;
    let attrs = &mut item.attrs;

    for idx in (0..attrs.len()).rev() {
        if let Some(ident) = attrs[idx].path().get_ident() {
            if ident.to_string() == "precondition" {
                let attr = attrs.remove(idx);

                if let Ok(arg) = syn::parse2::<AttributeArgument>(attr.parse_args().unwrap()) {
                    preconditions.push(arg.expr);
                } else {
                    let _ = attr.parse_nested_meta(|meta| {
                    abort!(meta.path, "expected an expression as argument"; help = "example syntax: `#[precondition(argument % 2 == 0)]`");
                });
                }
            }
        }
    }

    if preconditions.is_empty() {
        abort_call_site!(
            "no contracts were specified";
            help = "add a `#[precondition]`"
        );
    }

    Model {
        preconditions,
        item,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Attribute};

    #[test]
    #[ignore]
    fn test_extract_preconditions() {
        let model = analyze(parse_quote!(
            #[precondition(x)]
            fn f(x: bool) {}
        ));

        let expected: &[Expr] = &[parse_quote!(x)];
        assert_eq!(expected, &model.preconditions[..]);

        assert!(model.item.attrs.is_empty());
    }

    #[test]
    #[ignore]
    fn test_non_dsl_attrs_are_preserved() {
        let model = analyze(parse_quote!(
            #[a]
            #[precondition(x)]
            #[b]
            fn f(x: bool) {}
        ));

        let expected: &[Expr] = &[parse_quote!(x)];
        assert_eq!(expected, model.preconditions);

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];

        assert_eq!(expected, &model.item.attrs[..]);
    }
}
