#![allow(non_snake_case)]

// Max 12 elements

pub fn run() {
	println!("----------------------------");
	let person: (&str, &str, i8) = ("Alan", "Arriaga", 24);
	println!("{} {} is {}", person.0, person.1, person.2);
	println!("----------------------------");
}