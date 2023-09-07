use plain_macros::my_vec;

fn main() {
    let vs = my_vec!["Hello", "world", "we", "meet", "again"];
    println!("{vs:#?}");
}
