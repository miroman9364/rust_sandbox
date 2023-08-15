/* https://youtu.be/zF34dRivLOw?t=3300

vectors are resizable arrays
*/

use std::mem;

pub fn run() {
    println!("!!!\nVectors\n!!!");

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("vector has {} element length, but uses {} bytes, elements: {:?}", numbers.len(), mem::size_of_val(&numbers), numbers);

    // index an item, 0-based indexing
    assert_eq!(3, numbers[2]);

    // can modify mut
    numbers[1] = 20;
    assert_eq!(20, numbers[1]);

    // add with push
    numbers.push(6);
    numbers.push(7);
    println!("vector has {} element length, but uses {} bytes, elements: {:?}", numbers.len(), mem::size_of_val(&numbers), numbers);

    // drop last with pop
    numbers.pop();
    println!("vector has {} element length, but uses {} bytes, elements: {:?}", numbers.len(), mem::size_of_val(&numbers), numbers);

    // iterate (immutable)
    for x in numbers.iter() {
        println!("x: {}", x)
    }

    // iterate (mutable)
    for x in numbers.iter_mut() {
        *x *= 10;
        println!("x: {}", x)

    }

    // Note: size of val doesn't make sense; it's not changing as the vector grows?
    numbers.push(7);
    numbers.push(8);
    numbers.push(9);
    numbers.push(10);
    numbers.push(11);
    numbers.push(12);
    numbers.push(13);
    numbers.push(14);
    numbers.push(15);
    numbers.push(16);
    numbers.push(17);
    numbers.push(18);
    numbers.push(19);
    println!("vector has {} element length, but uses {} bytes, elements: {:?}", numbers.len(), mem::size_of_val(&numbers), numbers);

    // specify the first index and the last + 1, i.e., left closed, right open, [1..3)
    let slice1: &[i32] = &numbers[1..3];
    println!("slice1: {:?}, len: {}", slice1, slice1.len());
    assert_ne!(3, slice1.len());
    assert_eq!(2, slice1.len());
}
