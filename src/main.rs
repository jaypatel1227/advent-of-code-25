mod d1;
mod d2;
use d1::{d1p1, d1p2};
use d2::{d2p1, d2p2};

fn main() {
    let solutions = vec![
        ("Day 1 - Part 1", d1p1 as fn() -> String),
        ("Day 1 - Part 2", d1p2 as fn() -> String),
        ("Day 2 - Part 1", d2p1 as fn() -> String),
        ("Day 2 - Part 2", d2p2 as fn() -> String),
    ];

    println!("Solving Advent of Code 2025!");
    for (name, func) in solutions {
        let output: String = func();
        println!("{} Solution: {}", name, output);
    }
}
