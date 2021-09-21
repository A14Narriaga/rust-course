#![allow(non_snake_case)]

fn printFullName(name: &str, lastname: &str) {
	println!("{} {}", name, lastname);
}

fn add(a: i32, b: i32) -> i32 {
	a+b
}

pub fn run() {
	println!("----------------------------");
	printFullName("Alan", "Arriaga");
	println!("{}", add(5, 5));
	// Closure
	let c: i32 = 10;
	let addNums = |a:i32, b:i32| a+b+c;
	println!("{}", addNums(3, 3));
	println!("----------------------------");
}