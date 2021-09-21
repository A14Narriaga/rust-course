#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	// Constant
	let activity = "XBOX";
	// Variable
	let mut age = 23;
	age += 1;
	// Define constant
	const ID: i32 = 001; // Integer 32 bits
	// Asigned multiple vars
	let (name, lastname) = ("Alan", "Arriaga");
	println!("My name is {} {} and I am {} and play {}, ID: {}", name, lastname, age, activity, ID);
	println!("----------------------------");
}