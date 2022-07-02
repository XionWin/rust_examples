#[allow(dead_code)]
mod structs;

use structs::*;

fn main() {
    println!("Hello, world!");

    let person = Person {
        name: String::from("Amy"),
        age: 11u8
    };
    println!("{:?}", person);

    let unit = Unit;
    println!("{:?}", unit);

    let pair = Pair(0i32, 0.1f32);
    println!("{:?}", pair);
    println!("Pair [0]: {:?},  [1]: {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

}
