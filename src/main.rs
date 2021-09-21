#![allow(non_snake_case)]

mod prints;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointersRef;
mod structs;
mod enums;
mod cli;

fn main() {
  prints::run();
	vars::run();
	types::run();
	strings::run();
	tuples::run();
	arrays::run();
	vectors::run();
	conditionals::run();
	loops::run();
	functions::run();
	pointersRef::run();
	structs::run();
	enums::run();
	cli::run();
}

