use std::collections::HashMap;

// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {

//         if s.len()!=t.len(){
//             return false;
//         }

//         let mut map = HashMap::new();

//         for c in s.chars(){
//             let count = map.entry(c).or_insert(0);
//             *count += 1;
//         }

//         for c in t.chars(){
//             let count = map.entry(c).or_insert(0);
//             *count -= 1;
//         }

//         map.values().all(|&x| x == 0)
//     }
// }

fn main() {
    println!("Hello, world!");
    let s = String::from("anagram");

    let t = String::from("nagaram");


    let mut map = HashMap::new();

    for c in s.chars(){
        let count = map.entry(c).or_insert(0);
        println!("count: {}", count);
        *count += 1;
    }

    for c in t.chars(){
        let count = map.entry(c).or_insert(0);
        *count -= 1;
    }

    // map.values().all(|&x| x == 0)

}
