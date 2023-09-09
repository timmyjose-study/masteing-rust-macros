use print_ast_derive::PrintAST;

#[test]
fn test_print_ast() {
    #[derive(PrintAST)]
    struct Person {
        name: String,
        age: i32,
        salary: f64,
    }
}