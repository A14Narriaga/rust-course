#![allow(non_snake_case)]

pub fn run() {
	println!("----------------------------");
	let mut count = 0;
	// Infinite loop
	loop {
		println!("Loop: {}", count);
		if count == 5 {break}
		count += 1;
	}
	// While
	while count <= 8 {
		println!("While: {}", count);
		count += 1;
	}
	// For
	for i in 0..2 {
		println!("For: {}", i);
	}
	println!("----------------------------");
}