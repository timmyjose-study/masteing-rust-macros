use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item};

pub type Ast = syn::ItemFn;

pub(crate) fn parse(args: TokenStream, item: TokenStream) -> Ast {
    const ERROR: &str = "this attribute takes no arguments";
    const HELP: &str = "use `#[contracts]`";

    // attribute macro takes no arguments
    if !args.is_empty() {
        if let Ok(expr) = syn::parse2::<Expr>(args) {
            // if we can extract an expr, report with that expr
            abort!(expr, ERROR; help = HELP);
        } else {
            // else simply abort with the general error message and help
            abort_call_site!(ERROR; help = HELP);
        }
    }

    // this attribute can only be used on a function item
    match syn::parse2::<Item>(item) {
        Ok(Item::Fn(item)) => item,
        Ok(item) => {
            abort!(item, "item is not a function"; help = "`#[contracts]` can only be used on functions");
        }
        Err(__) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use quote::quote;

    #[test]
    fn test_parse_valid_syntax() {
        parse(
            quote!(),
            quote!(
                #[inline]
                #[precondition(x % 2 == 0)]
                fn even_to_odd(x: u32) -> u32 {
                    x + 1
                }
            ),
        );
    }
}
