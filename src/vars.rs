// https://youtu.be/zF34dRivLOw?t=1166
// Variables hold primitive data or references to data.
// Variables are immutable by default.
// Rust is a block-scoped language.

pub fn run() {
    let name = "Mike";

    // TODO: How do I do date arithmetic? It would be better to calculate my current age.
    let mut age = 54;

    println!("My name is {}, and I'm {}.", name, age);

    age = 55;
    println!("I will be {} on my next birthday.", age);

    // define a constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables at once
    let (my_name, my_age) = ("Mike", 54);
    println!("My name is {}, and I'm {}.", my_name, my_age);
}
