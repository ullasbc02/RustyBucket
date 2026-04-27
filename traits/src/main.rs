trait Area {
    fn area(&self) -> f64;

    // A "provided" method that uses the required area() method
    // fn print_area(&self) {  // default implementation of print_area
    //     println!("The area is: {}", self.area());
    // }
    // fn print_area(&self);


    // Associated function with trait bounds
    // fn print_area(shape: &impl Area) {
    //     println!("{}", shape.area());
    // }

    
    
}

struct Rectangle { length: f64, width: f64 }
struct Circle { radius: f64 }

//Trait bound syntax
fn print_area<T: Area>(shape: &T) {
    println!("{}", shape.area());
}
impl Area for Rectangle {
    fn area(&self) -> f64 { self.length * self.width }
    // fn print_area(&self) {
    //     println!("The area of the rectangle is: {}", self.area());
    // }
}

impl Area for Circle {
    fn area(&self) -> f64 { 3.14 * self.radius * self.radius }
    // fn print_area(&self) {
    //     println!("The area of the circle is: {}", self.area());
    // }
}

fn main() {
    let r = Rectangle { length: 5.0, width: 3.0 };
    let c = Circle { radius: 2.0 };

    // r.print_area(); // Prints 15
    // c.print_area(); // Prints 12.56


    // Rectangle::print_area(&r); // Prints 15
    // Circle::print_area(&c); // Prints 12.56


    print_area(&r); // Prints 15
    print_area(&c); // Prints 12.56
}