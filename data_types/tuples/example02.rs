// Tuple items can be access with the tuple indexing operation:

#![allow(unused)]
fn main() {
    let tuple = (true, 2, 3.0);
    println!("{}", tuple.0); // true
    println!("{}", tuple.1); // 2
    println!("{}", tuple.2); //3.0
    // tuple.3 would result in a compilation error!
}
