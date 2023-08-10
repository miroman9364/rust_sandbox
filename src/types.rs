/* https://youtu.be/zF34dRivLOw?t=1509

primitive types covered in this topic
integers
    u8
    u16
    u32
    u64
    u128
    18
    i16
    i32
    i64
    i128
floats
    f32
    f64
boolean
    bool
characters
    char

primitive types not covered in this topic
arrays
tuples

Rust is a statically typed language, which means types must be declared at compile time. However, the compiler will try to infer the type of any variable that is not explicitly typed.

Note: In VS Code, the extension `rust-lang.rust-analyzer.0.3.1615` will show/prompt to insert explicit types. This is visually different than how explicit types that are present are rendered.

TODO:
- how many bits in a `char` and what encoding format?
- what kind of characters and encoding do strings have?
- "strings are weird in Rust"
- are there structs?
- is the tuple a poor mans struct
- arrays are fixed length
*/

pub fn run() {
    // compiler defaults to `i32` for inferred integers
    let x = 42;

    // compiler defaults to `f64` for inferred floating point
    let y = 2.5;

    // explicit i64
    let z: i64 = 454545454545;

    // get max size for type
    println!("maz size for i32: {}", std::i32::MAX);
    println!("maz size for u128: {}", std::u128::MAX);

    // inferred bool
    let is_active = true;

    // bool inferred from expression
    let is_greater = 10 < 5;

    // single quoted character is inferred as `char`
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
