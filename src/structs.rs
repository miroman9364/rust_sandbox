/* https://youtu.be/zF34dRivLOw?t=4982

structs are used to create custom data types, they are like classes, but they are not classes
*/

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct TupleColor(u8, u8, u8);

// traditional struct with impl
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // constructor
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    fn to_tuple(&self) -> (&String, &String) {
        (&self.first_name, &self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: red: {}, green: {}, blue: {}", c.red, c.green, c.blue);

    let mut tc: TupleColor = TupleColor(255, 0, 0);
    tc.0 = 200;
    println!("Tuple Color: red: {}, green: {}, blue: {}", tc.0, tc.1, tc.2);

    let mut p: Person = Person::new("mike", "Romanchuk");
    p.first_name = "Mike".to_string();
    println!("Person: {} {}.", p.first_name, p.last_name);
    p.set_last_name("the Most Interesting Man Alive!");
    println!("Person: full name: {}.", p.full_name());
    let mut t: (&String, &String) = p.to_tuple();
    println!("Person: tuple: {} {}.", t.0, t.1);
    let real_last_name: String = "Romanchuk".to_string();
    t.1 = &real_last_name;
    println!("Person: full name: {}, tuple: {} {}.", p.full_name(), t.0, t.1);
}
