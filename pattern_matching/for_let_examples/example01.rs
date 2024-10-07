
#![allow(unused)]
fn main() {
let tuples: [(usize, &'static str); 6] = [
    (1, "red"),
    (2, "white"),
    (3, "blue"),
    (4, "yellow"),
    (5, "brown"),
    (6, "black"),
];

for (numbering, color) in tuples {
    println!("Color #{numbering} is {color}!");
}
}
