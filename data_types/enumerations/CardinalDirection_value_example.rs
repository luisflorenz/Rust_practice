
#![allow(unused)]
fn main() {
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

let d = CardinalDirection::East;

if let CardinalDirection::East = d {
    println!("We are going east!");
} else {
    println!("We are not going east but in some other direction!");
}
}
