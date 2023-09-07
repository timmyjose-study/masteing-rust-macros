use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

//impl HelloMacro for Pancakes {
//    fn hello_macro() {
//        println!("Hello, macro! My name is Pancakes!");
//    }
//}
//

#[derive(HelloMacro)]
#[allow(dead_code)]
pub struct Person {
    name: String,
    age: i32,
}

fn main() {
    Pancakes::hello_macro();
    Person::hello_macro();
}
