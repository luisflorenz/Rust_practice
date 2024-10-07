#![allow(unused)]
fn main() {
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

let meal = Meal::FishAndChips {
    chip_proportion: 0.6,
};

match meal {
    Meal::FishAndChips { chip_proportion } => {
        if chip_proportion > 0.5 {
            println!("I had some fish and plenty of chips!");
        } else if chip_proportion < 0.5 {
            println!("I had plenty of fish and some chips!");
        } else {
            println!("I had fish and chips!");
        }
    }
    Meal::Hamburger { vegetarian } => {
        if vegetarian {
            println!("I had a vegetarian hamburger!");
        } else {
            println!("I had a meaty hamburger!");
        }
    }
}
}
