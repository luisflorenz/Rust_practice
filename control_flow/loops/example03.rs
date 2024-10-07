// You could also write the above loop with for

#![allow(unused)]
fn main() {
    for i in (1..=10).rev() {
        println!("{i}...");
    }
    println!("Launch!");
}