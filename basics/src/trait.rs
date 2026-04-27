// trait Speak {
//     fn speak(&self);
// }

// struct Dog {
//     name: String,
// }
// struct Cat {
//     name: String,
// }

// impl Speak for Cat {
//     fn speak(&self) {
//         println!("{} says meow", self.name);
//     }
// }
// impl Speak for Dog {
//     fn speak(&self) {
//         println!("{} says woof", self.name);
//     }
// }


// default trait implementation
trait Logger {
    fn log(&self, msg: &str) {
        println!("LOG: {}", msg);
    }
}
struct App;

impl Logger for App {
    // fn log(&self, msg: &str) {   // overriding the default implementation
    //     println!("APP LOG: {}", msg);
    // }
}  // no method written, it will use the default implementation

pub fn demo() {
    // let dog = Dog {
    //     name: String::from("Bruno"),
    // };

    // let cat = Cat {
    //     name: String::from("Whiskers"),
    // };
    // cat.speak();
    // dog.speak();
    let a = App;
    a.log("started");
}