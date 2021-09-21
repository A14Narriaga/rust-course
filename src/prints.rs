#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	// Basic argumnets
	println!(
		"Number {} & {}",
		1, 2
	);
	// Positional arguments
	println!(
		"{0} & {1} & {0}", 
		"Blue", "White");
	// Named arguments
	println!(
		"{name} play {activity}", 
		name = "Alan", 
		activity = "XBOX"
	);
	// Placeholde
	println!(
		"Bin: {:b}, Hex: {:x}, Oct: {:o}",
		10, 10, 10
	);
	// Placeholder for debug
	println!(
		"{:?}",
		(12, true, "Hello")
	);
	// Basic math
	println!(
		"10 + 10 = {}",
		10+10
	);
	println!("----------------------------");
}