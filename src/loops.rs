/* https://youtu.be/zF34dRivLOw?t=4002

loops repeat until condition is met
*/

pub fn run() {
    let mut count: u128 = 0;

    // infinite loop (with internal safe guard)
    loop {
        fizz_buzz(count);
        count += 1;

        if count > 100 {
            // would otherwise loop forever
            break;
        }
    }

    // while loop (FizzBuzz)
    count = 0;
    while count <= 100 {
        fizz_buzz(count);
        count += 1;
    }

    // for-range

    for count in 0..101 {
        fizz_buzz(count);
    }

}

fn fizz_buzz(count: u128) {
    let mut show_count = true;

    if count % 3 == 0 {
        print!("Fizz");
        show_count = false;
    }

    if count % 5 == 0 {
        print!("Buzz");
        show_count = false;
    }

    if show_count {
        print!("{}", count);
    }

    println!();
}
