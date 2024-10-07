// If the programmer so desires, the amount of nesting caused by the if statement by matching against values inside the enum variants and through match guards:


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
    Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
        println!("I had some fish and plenty of chips!");
    }
    Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
        println!("I had plenty of fish and some chips!");
    }
    Meal::FishAndChips { chip_proportion } => {
        println!("I had fish and chips!");
    }
    Meal::Hamburger { vegetarian: true } => {
        println!("I had a vegetarian hamburger!");
    }
    Meal::Hamburger { vegetarian: false } => {
        println!("I had a meaty hamburger!");
    }
}
}
