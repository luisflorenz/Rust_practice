// Array items can be accessed with the indexing operation:

#![allow(unused)]
fn main() {
let array = [1, 2, 3];
println!("{}", array[0]); // 1
println!("{}", array[1]); // 2
println!("{}", array[2]); // 3
// array[3] will result in a compilation error (because the size of the array is known)!
}