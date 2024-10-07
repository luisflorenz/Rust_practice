// Variables can be initialized by providing an expression. The type of expression must match the type of the pattern


#![allow(unused)]
fn main() {
struct Plant {
    flowering: bool,
    mass: f64,
}

let plant = Plant {
    flowering: true,
    mass: 10.0,
};
let Plant { flowering, mass } = plant;
}
