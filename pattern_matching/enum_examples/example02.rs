// Now you only want to print a message if you had a vegetarian hamburger. You can do that by requiring vegetarian to be true.


#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let meal = Meal::Hamburger {
    vegetarian: true,
};

if let Meal::Hamburger { vegetarian: true } = meal {
    println!("I had a vegetarian hamburger!");
}
}
