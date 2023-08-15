/* https://youtu.be/zF34dRivLOw?t=2683

A `tuple` is:

- a group of elements
- max of 12 elements
- can be different types

TODO: Find out if a tuple is a native type, standard library type, or some kind of syntactic sugar?
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Mike", "Seattle", 54);

    println!("{} is from {} and is {}.", person.0, person.1, person.2);
}
