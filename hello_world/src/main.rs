// fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
// fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

// use std::fmt;

// #[derive(Debug)] // Automatically implements Debug
// struct PerformanceReport {
//     latency_ms: u32,
//     endpoint: String,
// }

// // Manually implementing Display
// impl fmt::Display for PerformanceReport {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "The request to '{}' took {}ms.", self.endpoint, self.latency_ms)
//     }
// }

// fn main() {
//     let report = PerformanceReport {
//         latency_ms: 120,
//         endpoint: String::from("/api/v1/trace"),
//     };

//     // DEBUG output (Technical)
//     println!("{:?}", report); 
//     // Output: PerformanceReport { latency_ms: 120, endpoint: "/api/v1/trace" }

//     // DISPLAY output (User-friendly)
//     println!("{}", report);   
//     // Output: The request to '/api/v1/trace' took 120ms.
// }


// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);


fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    // This will not compile with `{}` because `Structure` does not implement
    // fmt::Display. Use `{:?}` to print types that implement `Debug`.
    println!("This struct `{:?}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");


    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));




}