#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let meal = Meal::Hamburger {
    vegetarian: true,
};

if let Meal::Hamburger { .. } = meal {
    println!("I had a hamburger!");
}
}
