#![allow(non_snake_case)]

enum Movement {
	Up,
	Down, 
	Left,
	Right
}

fn moveAvatar(m: Movement) {
	// Switch
	match m {
		Movement::Up => println!("Avatar moving Up"),
		Movement::Down => println!("Avatar moving Down"),
		Movement::Left => println!("Avatar moving Left"),
		Movement::Right => println!("Avatar moving Right")
	}
}

pub fn run() {
	println!("----------------------------");
	moveAvatar(Movement::Up);
	moveAvatar(Movement::Down);
	moveAvatar(Movement::Left);
	moveAvatar(Movement::Right);
	println!("----------------------------");
}