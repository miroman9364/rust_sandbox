/* https://youtu.be/zF34dRivLOw?t=5879

enums are types that have a finite set of enumerable values that can be assigned
*/

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // use the match operator, which is the Rust operator that provide behavior similar to the case operator of C++
    match m {
        Movement::Up => println!("Avatar moved up."),
        Movement::Down => println!("Avatar moved down."),
        Movement::Left => println!("Avatar moved left."),
        Movement::Right => println!("Avatar moved right."),
    }
}
pub fn run() {
    let avatar1: Movement = Movement::Left;
    let avatar2: Movement = Movement::Up;
    let avatar3: Movement = Movement::Right;
    let avatar4: Movement = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
