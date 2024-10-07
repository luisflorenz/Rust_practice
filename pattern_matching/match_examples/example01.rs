// The match statement allows you to specify multiple patterns to be matched against. These patterns are matched against in order. This allows these patterns to overlap. This is useful because you can write more specific patterns and handle any other values not matched by those patterns later. 

#![allow(unused)]
fn main() {
for n in 0..=5 {
    match n {
        1 => println!("It was one!"),
        2 => println!("It was two!"),
        // or-pattern
        3 | 4 => println!("It was a bit more than two!"),
        // match guard
        high if high >= 5 => println!("It was a high number!"),
        // a pattern consisting only of the name `other`
        other => println!("It was {other}!"),
    }
}
}
