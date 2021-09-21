#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
	// Print
	println!("{:?}", numbers);
	// Lenght
	println!("Vector lenght: {}", numbers.len());
	// Add element
	numbers.push(5);
	println!("Add element: {:?}", numbers);
	// Print first value
	println!("Vector in [0] = {}", numbers[0]);
	// Delete last value
	numbers.pop();
	println!("Drop element {:?}", numbers);
	// Size of array in memory bytes
	println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
	// Get slice
	let slice: &[i32] = &numbers[0..2];
	println!("Vector slice {:?}" , slice);
	// Loop by values
	for i in numbers.iter() {
		println!("{}", i);
	}
	// Loop and mutate values
	for i in numbers.iter_mut() {
		*i *= 2;
	}
	println!("Mutate velues: {:?}", numbers);
	println!("----------------------------");
}