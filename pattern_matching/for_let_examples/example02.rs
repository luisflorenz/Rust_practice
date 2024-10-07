#![allow(unused)]
fn main() {
let colors = [
    "red",
    "white",
    "blue",
    "yellow",
    "brown",
    "black",
];

for (index, color) in colors.into_iter().enumerate() {
    let numbering = index + 1;
    println!("Color #{numbering} is {color}!");
}
}

