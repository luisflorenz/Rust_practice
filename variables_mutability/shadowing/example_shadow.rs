// Shadowing☝️ It is allowed to redefine a variable with the same name.

#![allow(unused)]
fn main() {
let x = 2;
{
    let x = 3;
}
println!("{}", x); // 2
}
