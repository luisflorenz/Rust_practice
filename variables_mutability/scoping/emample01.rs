// Like many other programming languages, Rust uses the word scope to refer to a piece of code in which variables live. Rust allows you to use variables declared in outer scopes, but variables do not leak out of their scope.


#![allow(unused)]
fn main() {
let x = 2;
println!("{}", x); // 2
// Create a new scope
{
    let y = 3;
    // We can use x here
    println!("{}", x); // 2
    println!("{}", y); // 3
}
println!("{}", x); // 2
// println!("{}", y); // won't compile because y is "not in scope"
}
