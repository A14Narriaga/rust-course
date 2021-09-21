#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	let age = 22;
	let check_id = true;
	let knowPersonsAge = true;
	// If/Else
	if age >= 21 && check_id || knowPersonsAge {
		println!("Bartender: What would you like to drink?");
	}
	else if age < 21 && check_id {
		println!("Bartender: Sorry, you have to leave");
	}
	else {
		println!("Bartender: I'll need to see your ID");
	}
	// Short if
	let isOfAge = if age >= 21 {true} else {false};
	println!("Is of age: {}", isOfAge);
	println!("----------------------------");
}