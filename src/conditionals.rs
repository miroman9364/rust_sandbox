/* https://youtu.be/zF34dRivLOw?t=3615
*/

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_is_old_enough = true;

    if age >= 21 && check_id || knows_person_is_old_enough{
        println!("what do you want to drink?");
    } else if age < 21 && check_id {
        println!("get outta here before I call the cops you mug!");
    } else {
        println!("show me your ID");
    }
}
