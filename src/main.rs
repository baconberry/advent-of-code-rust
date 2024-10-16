#![allow(unused)]

use advent_of_code::prelude::*;
mod utils;

use advent_of_code::process_lines;
use utils::read_lines;

fn main() {
    let lines = read_lines("input.txt".to_string());
    let result = process_lines(lines, 19, 1);
    println!("Result [{:?}]", result);
}
