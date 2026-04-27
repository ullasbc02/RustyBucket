// mod math {
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
// }

mod math;

// Nested modules
use math::basic::add;

fn main() {
    // let result = math::add(2, 3);
    let result = add(2, 3);
    println!("{}", result);
}