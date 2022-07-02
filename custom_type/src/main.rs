#[allow(dead_code)]
mod structs;

fn main() {
    println!("Hello, world!");

    let person = structs::Person {
        name: String::from("Amy"),
        age: 11u8
    };

    println!("{:?}", person);
}
