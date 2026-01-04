mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
use d1::{d1p1, d1p2};
use d2::{d2p1, d2p2};
use d3::{d3p1, d3p2};
use d4::{d4p1, d4p2};
use d5::{d5p1, d5p2};
use d6::{d6p1, d6p2};

fn main() {
    let solutions = vec![
        ("Day 1 - Part 1", d1p1 as fn() -> String),
        ("Day 1 - Part 2", d1p2 as fn() -> String),
        ("Day 2 - Part 1", d2p1 as fn() -> String),
        ("Day 2 - Part 2", d2p2 as fn() -> String),
        ("Day 3 - Part 1", d3p1 as fn() -> String),
        ("Day 3 - Part 2", d3p2 as fn() -> String),
        ("Day 4 - Part 1", d4p1 as fn() -> String),
        ("Day 4 - Part 2", d4p2 as fn() -> String),
        ("Day 5 - Part 1", d5p1 as fn() -> String),
        ("Day 5 - Part 2", d5p2 as fn() -> String),
        ("Day 6 - Part 1", d6p1 as fn() -> String),
        ("Day 6 - Part 2", d6p2 as fn() -> String),
    ];

    println!("Solving Advent of Code 2025!");
    for (name, func) in solutions {
        let output: String = func();
        println!("{} Solution: {}", name, output);
    }
}
