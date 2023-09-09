use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

//mod analyzer;
//mod codegen;
//mod ir;
mod parser;

use parser::{parse, Ast};
//use analyzer::{Model, analyze};
//use ir::{Ir, lower};
//use codegen::gen_code;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn contracts(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());
    //let model = analyze(ast);
    //let ir = lower(model);
    //let rust_tok_stream = gen_code(ir);

    //rust_tok_stream
    todo!()
}
