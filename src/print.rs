// https://youtu.be/zF34dRivLOw?t=625
pub fn run() {
    // print to console
    println!("Hello from the print.rs file.");
    // basic formatting
    println!("{} is from {}.", "Mike", "MASS");
    // positional arguments
    println!(
        "{0} is from {1}, but {0} lives in {2}.",
        "Mike", "MASS", "Seattle"
    );
    // named arguments
    println!(
        "{name} likes to play {action}.",
        name = "John",
        action = "baseball"
    );
    // placeholder traits
    println!("Binary: {0:b}, Hex: {0:X}, Octal: {0:o}", 10);
    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}
