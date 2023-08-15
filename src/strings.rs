/* https://youtu.be/zF34dRivLOw?t=2025

There are 2 types of string:

- primitive `str` are immutable, fixed length, somewhere in memory
- `String` - growable, modifiable, heap-allocated data structure; use when you need to modify or own string data (like a vector)

 */
pub fn run() {
    // compiler infers `&str` for double-quoted values
    let str_primitive = "hello";

    // compiler infers `String` when using `from` function
    let mut hello = String::from("Hello");
    println!("`String` value: {}, length: {}", hello, hello.len());

    // `len()` works for both types
    println!("`str` length: {}", str_primitive.len());
    println!("`String` length: {}", hello.len());

    // can push single `char`
    hello.push(' ');
    println!("`String` value: {}, length: {}", hello, hello.len());

    // can push `str`
    let world = "World!";
    hello.push_str(world);
    println!("`String` value: {}, length: {}", hello, hello.len());

    // seems like `contains` performs an exact match
    println!("contains \"{}\"? {}", world, hello.contains(world));

    // seems like `contains` cannot match regex
    println!("contains \"[wW][oO][rR][lL][dD]\"? {}", hello.contains("[wW][oO][rR][lL][dD]"));

    // replace
    let name = "Mike!";
    println!("replace: {}", hello.replace(world, name));

    // iterate through string
    for word in hello.split_whitespace() {
        println!("word: {}", word);
    }

    // create string with capacity
    const CAPACITY: usize = 10;
    let mut s = String::with_capacity(CAPACITY);
    s.push('a');
    s.push('b');

    // assert positive case
    assert_eq!(CAPACITY, s.capacity());
    println!("The string must have the asserted capacity.");

    // assert fail example
    let unexpected_length: usize = s.len() + 1;
    assert_eq!(unexpected_length, s.len(), "s.capacity()");
    println!("UNEXPECTED: the string should not have the asserted length!");
}
