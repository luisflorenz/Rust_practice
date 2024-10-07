// The  while let statement lets you loop until the pattern does not match. If you like, try to determine what the following program prints.


#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let mut meal = Meal::FishAndChips {
    chip_proportion: 0.6,
};
while let Meal::FishAndChips { chip_proportion } = meal {
    println!("Having fish and chips with chip proportion {:.2}...", chip_proportion);
    if chip_proportion > 0.3 {
        // Order a meal with less chips.
        meal = Meal::FishAndChips {
            chip_proportion: chip_proportion - 0.2,
        }
    } else {
        // Too fishy, let's get a hamburger next.
        meal = Meal::Hamburger { vegetarian: true }
    }
}
println!("I'm so done with fish and chips!");
}
