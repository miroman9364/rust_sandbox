/* https://youtu.be/zF34dRivLOw?t=4740
*/

pub fn run() {
    // access primitive
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr1;

    println!("values: {:?}", (arr1, arr2));

    // access user types
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: &Vec<i32> = &vec1;

    println!("values: {:?}", (&vec1, vec2));
}
