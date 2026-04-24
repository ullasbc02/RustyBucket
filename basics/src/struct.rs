#[derive(Debug)]
pub struct User {
	pub name: String,
	pub age: u8,
}

impl User {
	// Rust has no special constructor keyword.
	// Convention is to use associated functions like `new`.
	pub fn new(name: String, age: u8) -> Self {
		Self { name, age }
	}

	pub fn can_vote(&self) -> bool {
		self.age >= 18
	}
}

// Default values can be provided via the `Default` trait.
impl Default for User {
	fn default() -> Self {
		Self {
			name: String::from("Guest"),
			age: 0,
		}
	}
}

fn create_user_with_shorthand(name: String, age: u8) -> User {
	// Field shorthand: when variable names and field names are same,
	// `name: name, age: age` becomes `name, age`.
	User { name, age }
}

pub fn run_struct_demo() {

    let user1 = User {
        name: String::from("Ullas"),
        age: 25
    };
	// 1) "Programmer-written constructor" style using `new`.
	let user = User::new(String::from("Ullas"), 24);

	// 2) "Default constructor" style using `Default`.
	let default_user = User::default();

	// 3) You do NOT re-declare field types each time.
	// Types are declared once in struct definition; object literal values are checked.
	let name = String::from("Anu"); // type inferred as String
	let age = 19; // type inferred; fits into u8 from function signature below
	let shorthand_user = create_user_with_shorthand(name, age);

    // println!("User1: {:?}", user1);
	// println!("User: {}", user.name);
	// println!("Can vote: {}", user.can_vote());
	// println!("Default user: {:?}", default_user);
	// println!("Shorthand user: {:?}", shorthand_user);
}
