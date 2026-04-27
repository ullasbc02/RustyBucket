// Enums in Rust are a powerful way to define a type that can be one of several variants. Each variant can have its own data associated with it. Enums are often used in conjunction with pattern matching to handle different cases in a clean and readable way.

// Basic Enum Example
// enum Direction{
//     UP,
//     DOWN,
//     LEFT,
//     RIGHT,
// }

// Enums with Data and multiple variants
enum Event{
    Login { username: String, time: String },
    Logout { username: String},
}

fn main(){
    // let d = Direction::DOWN;
    // match d {
    //     Direction::UP => println!("Going up!"),
    //     Direction::DOWN => println!("Going down!"),
    //     Direction::LEFT => println!("Going left!"),
    //     Direction::RIGHT => println!("Going right!"),
    // }

    let login_event = Event::Login { 
        username: String::from("Ullas"), 
        time: String::from("10:00 AM") 
    };
    let logout_event = Event::Logout { 
        username: String::from("Ullas") 
    };

    match login_event {
        Event::Login { username, time } => {
            println!("{} logged in at {}", username, time);
        }
        Event::Logout { username } => {
            println!("{} logged out", username);
        }
    }

}