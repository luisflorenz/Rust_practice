// If you wanted to skip even numbers, you could make use of continue

#![allow(unused)]
fn main() {
    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");
}