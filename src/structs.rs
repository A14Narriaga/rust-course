#![allow(non_snake_case)]

// Struct
struct Color {
	red: u8,
	green: u8,
	blue: u8
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

struct Person {
	name: String,
	lastname: String
}
impl Person {
	fn new(newName: &str, newLastname: &str) -> Person {
		Person {
			name: newName.to_string(),
			lastname: newLastname.to_string()
		}
	}
	fn setName(&mut self, n: &str) {
		self.name = n.to_string();
	} 
	fn getName(&self) -> String {
		format!("{}", self.name)
	}
	fn getTuple(self) -> (String, String) {
		(self.name, self.lastname)
	}
	fn toString(&self) -> String {
		format!("{} {}", self.name, self.lastname)
	}
}

pub fn run() {
	println!("----------------------------");
	let mut color = Color {
		red: 255,
		green: 0,
		blue: 0
	};
	color.red = 200;
	println!("Color: ({}, {}, {})", color.red, color.green, color.blue);
	let mut color1 = TupleColor(255, 0, 0);
	color1.0 = 200;
	println!("Color: ({}, {}, {})", color1.0, color1.1, color1.2);
	let mut person = Person::new("Alan", "Arriaga");
	println!("Person: {}", person.toString());
	println!("Name: {}", person.getName());
	person.setName("Eduardo");
	println!("Person: {}", person.toString());
	println!("Tuple: {:?}", person.getTuple());
	println!("----------------------------");
}