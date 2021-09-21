#![allow(non_snake_case)]

use std::env;

pub fn run() {
	println!("----------------------------");
	let args: Vec<String> = env::args().collect();
	let command = args[1].clone();
	println!("Args {:?}", (args, command));
	println!("----------------------------");
}