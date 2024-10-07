// You can create a value for your custom type MyStruct as follows:

#![allow(unused)]
fn main() {
struct MyStruct {
    should_do_groceries: bool,
    birth_year: u32,
    height_in_meters: f64,
}

let my_struct = MyStruct {
    should_do_groceries: true,
    birth_year: 1992,
    height_in_meters: 1.79,
};
}