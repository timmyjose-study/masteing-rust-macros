use print_fields_derive::PrintFields;

#[test]
fn test_print_fields() {
    #[derive(PrintFields)]
    struct Person {
        name: String,
        age: i32,
        salary: f64,
        married: bool,
    }
}
