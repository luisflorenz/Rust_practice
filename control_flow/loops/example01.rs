// It loops indefinitely.

// #![allow(unused)]
// fn main() {
// loop {
//    println!("I can't stop!");
// }
// }

#![allow(unused)]
fn main() {
    let mut i = 10;
    loop {
        if i == 0 {
            break; 
    }
    println!("{i}...");
    i -= 1;
}
println!("Launch!");
}