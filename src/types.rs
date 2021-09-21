#![allow(non_snake_case)]

/*
		Primitive Types --
		Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
		(u: unsigned & i: signed)
		Floats: f32, f64
		Boolean: bool
		Characters: char
		Tuples
		Arrays
	*/

pub fn run() {
	println!("----------------------------");
	// Default is 'i32'
	let x = 1;
	// Default is 'f64'
	let y = 2.5;
	// Add explicit type
	let z: i64 = 1234567890123;
	// Find max size
	let active:bool = true;
	let str1 = 'a';
	let str2 = "ab";
	let face = '\u{1F600}'; // Unicode
	println!("Max i32: {}", std::i32::MAX);
	println!("Max i64: {}", std::i64::MAX);
	println!("{:?}", (x, y, z, active, str1, str2, face));
	println!("----------------------------");
}