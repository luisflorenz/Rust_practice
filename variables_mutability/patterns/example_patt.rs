// The let statement is quite powerful. ☝️ You can provide a pattern instead of just the variable name.

// let (x, y) = (2, 3);


#![allow(unused)]
fn main() {
// Define the `Person` type.
struct Person {
    name: &'static str,
    age: u32,
    likes_brownies: bool,
}
// Create a `Person` from its parts.
let p = Person {
    name: "Mick",
    age: 30,
    likes_brownies: true,
};
// Deconstruct the `Person` back into its parts,
// omitting fields other than `name` and `age`.
let Person {
    name, age, ..
} = p;
}
