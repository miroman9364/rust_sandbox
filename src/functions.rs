/* https://youtu.be/zF34dRivLOw?t=4406

functions are reusable blocks of code
*/

pub fn run() {
    greeting("Hello", "Mike");

    let added: i32 = add(5, 5);
    assert_eq!(10, added);

    // closure - can access variables in same scope
    let n3 = 5;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    assert_eq!(15, add_nums(5, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
