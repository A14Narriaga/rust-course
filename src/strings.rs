#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	// No modificated string data
	let world = "world";
	// Modificated string data
	let mut hello = String::from("Hello");
	// Push a char
	hello.push(' ');
	// Push a string
	hello.push_str("my ");
	hello += world;
	// Print the string
	println!("{}", hello);
	// Capacity (number of bytes that can store)
	println!("Capacity (bytes): {}", hello.capacity());
	// Number of characters
	println!("Lenght: {}", hello.len());
	// Empty
	println!("Is empty: {}", hello.is_empty());
	// Contains
	println!("Contains: {}", hello.contains("Hello"));
	// Replace
	println!("Replace: {}", hello.replace("world", "World"));
	// Loop by whitespace
	for i in hello.split_whitespace() {
		println!("{}", i);
	}
	// Create str with capacity
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');
	println!("{}", s);
	// Equals testing
	assert_eq!(2, s.len());
	println!("----------------------------");
}