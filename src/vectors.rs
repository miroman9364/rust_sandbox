/* https://youtu.be/zF34dRivLOw?t=3300

vectors are resizable arrays
*/

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // index an item, 0-based indexing
    assert_eq!(3, numbers[2]);

    // iterate
    for number in numbers {
        println!("{}", number);
    }

    // mutable array makes array elements mutable
    let mut mut_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    mut_numbers[1] = 20;
    assert_eq!(20, mut_numbers[1]);

    // mutable arrays are still fixed length
    // mut_numbers[5] = 50;
    /*
        error: this operation will panic at runtime
        --> src\arrays.rs:25:5
        |
        25 |     mut_numbers[5] = 50;
        |     ^^^^^^^^^^^^^^ index out of bounds: the length is 5 but the index is 5
        |
        = note: `#[deny(unconditional_panic)]` on by default
    */

    let mut copy_numbers: [i32; 5] = numbers;
    println!("src: {:?}, dst: {:?}", numbers, copy_numbers);

    copy_numbers[4] = 500;
    println!("src: {:?}, dst: {:?}", numbers, copy_numbers);

    // arrays are stack allocated, pass/access array by reference
    println!("array uses {} bytes", mem::size_of_val(&numbers));
    assert_eq!(mem::size_of::<i32>() * numbers.len(), mem::size_of_val(&numbers));

    // get a slice of the array (i.e., a part of it), for example, get a slice that is all of the array
    let slice1: &[i32] = &numbers[1..3];
    println!("slice1: {:?}, len: {}", slice1, slice1.len());

    // specify the first index and the last + 1, i.e., left closed, right open, [1..3)
    let slice: &[i32] = &numbers[1..3];
    println!("slice: {:?}, len: {}", slice, slice.len());
    assert_ne!(3, slice.len());
    assert_eq!(2, slice.len());

    // TODO: can't do the following, find out how
    // let slice: &[i32] = &mut_numbers;
    // slice[4] = 5000;
    // assert_eq!(5000, mut_numbers[4]);
}
