// The if let statement is like if let except that it allows for refutable patterns. If the pattern matches, a block of code is executed.


#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let meal = Meal::Hamburger {
    vegetarian: true,
};

if let Meal::Hamburger { vegetarian } = meal {
    println!("I had a hamburger!");
}
}


