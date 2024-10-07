// For example, let's say you are building an application where the user can draw squares, rectangles, and circles.

// You can figure out what kind of Shapeyou are dealing with through pattern matching with a matchstatement:


#![allow(unused)]
fn main() {
enum Shape {
    Square {
        side: f64
    },
    Rectangle {
        width: f64,
        height: f64,
    },
    Circle {
        radius: f64,
    },
}

let s = Shape::Rectangle {
    width: 800.0,
    height: 60.0,
};
match s {
    Shape::Square { side } => {
        println!("A {}x{} square!", side, side);
    },
    Shape::Rectangle { width, height } => {
        println!("A {}x{} rectangle!", width, height);
    },
    Shape::Circle { radius } => {
        println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
    }
}
}


