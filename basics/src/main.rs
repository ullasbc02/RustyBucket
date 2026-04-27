
mod r#struct;
mod r#practice;
mod r#options;
mod r#trait;
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
fn main() {
    r#struct::run_struct_demo();
    r#practice::practice();
    r#options::options_demo();
    r#trait::demo();
    // declaring of mutable variable and shadowing of variable
    // let x = 5;
    // let x = 3;
    // println!("This is the value of x: {}", x);

    // declaring of constant variable
    // const PI: f64 = 3.14159;
    // println!("The value of PI is: {}", PI);

    // declaring datatypes
    // let x: i32 = 5;
    // let y: f64 = 3.14;
    // let z: bool = true;
    // let name: &str = "Ullas";
    // println!("Integer: {}, Float: {}, Boolean: {}, String: {}", x, y, z, name);

    // shadowing of variable
    // let x = 5;
    // let x = x + 1; // shadowing the previous value of x
    // let x = x * 2; // shadowing again
    // println!("The value of x is: {}", x);

    // calling a function
    // let result = add(5, 3);
    // println!("The result is: {}", result);

    // using a loop
    // for i in 0..5 {
    //     println!("The value of i is: {}", i);
    // }

    // using an if statement
    // let number = 10;
    // if number < 5 {
    //     println!("The number is less than 5");
    // } else if number == 5 {
    //     println!("The number is equal to 5");
    // } else {
    //     println!("The number is greater than 5");
    // }

    // using a match statement similar to switch case in other languages
    // let number = 2;
    // match number {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    //     3 => println!("Three"),
    //     _ => println!("Other"),
    // }

    // using a vector
    // let mut numbers = vec![1, 2, 3, 4, 5];
    // numbers.push(6);
    // println!("The numbers are: {:?}", numbers);
    // for number in &numbers { // if we use &numbers, we are borrowing the vector, if we use numbers, we are taking ownership of the vector
    //     println!("The number is: {}", number);
    // }
    // println!("The numbers are: {:?}", numbers);


    // using a tuple
    // let person: (&str, i32) = ("Ullas", 30);
    // println!("Name: {}, Age: {}", person.0, person.1);


    // ownership
    // let s1 = String::from("Hello");
    // let s2 = s1; // s1 is moved to s2, s1
    // println!("s1: {}", s1); // this will cause an error because s1 is moved
    // println!("s2: {}", s2); // this will work because s2 has ownership


    // borrowing
    // let mut s1 = String::from("Hello");
    // let s2 = &s1; // s2 is borrowing s1, s1 still has ownership
    // println!("s1: {}", s1); // this will work because s1 still has ownership
    // println!("s2: {}", s2); // this will work because s2 is borrowing s1

    // let s2 = s1.clone(); // s2 is a clone of s1, s1 still has ownership
    // println!("s1: {}", &s1); // this will work because s1 still has ownership
    // println!("s2: {}", &s2); // this will work because s2 is a clone of s1

    // let x = 100;
    // let xref = &x;
    // println!("{:p}", &x);
    // println!("{:p}", &xref);

    // let r1 = &s1; // r1 is borrowing s1, s1 still has ownership - Multiple immutable references are allowed
    // let r2 = &s1;
    // println!("r1: {}, r2: {}", r1, r2);

    // let r3 = &mut s1;
    // // println!("s1: {:p}", &s1);
    // println!("r3: {:p}", &r3);
    // r1.push_str("not mutable");
    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    // r3.push_str("world"); 
    // println!("r3: {}", r3);

    // let mut s1 = String::from("Hello");
    // let mut s2 = String::from("Hi");

    // let mut r1 = &mut s1;

    // println!("r1:{}", r1);
    // r1 = &mut s2; //  allowed because r1 is mutable
    // println!("r1:{}", r1);
    
}

