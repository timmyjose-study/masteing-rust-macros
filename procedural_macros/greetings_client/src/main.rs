use greetings::Greet;
use greetings_derive::Greet;

fn main() {
    #[derive(Greet)]
    struct Person {
        name: String,
        age: i32,
        salary: f64,
    }

    let bob = Person {
        name: "Bob".to_string(),
        age: 42,
        salary: 12345.67,
    };

    bob.greet();
}
