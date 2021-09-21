#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	// Declaration
	let mut numbers: [i32; 5] = [1,2,3,4,5];
	// Print
	println!("{:?}", numbers);
	// Lenght
	println!("Array lenght: {}", numbers.len());
	// Change value
	numbers[0] = 0;
	// Print first value
	println!("Array in [0] = {}", numbers[0]);
	// Size of array in memory bytes
	println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
	// Get slice
	let slice: &[i32] = &numbers[0..2];
	println!("Array slice {:?}" , slice);
	println!("----------------------------");
}