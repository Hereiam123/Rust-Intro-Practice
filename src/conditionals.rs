//Conditionals

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll, need to see your id");
    }
}
