use core::panic;
use std::collections::HashSet;

use crate::prelude::*;
use crate::col_utils;
use crate::re_utils;
use anyhow::Result;

type Pair = (usize, usize);
const ZERO_PAIR: Pair = (0,0);

#[derive(Debug)]
enum Direction {
    Both,
    Left,
    Right,
}

#[derive(PartialEq,Debug,Copy,Clone)]
enum ReflectionLine {
    Horizontal(usize),
    Vertical(usize)
}

const ZERO_VERTICAL_REFLECTION: ReflectionLine = ReflectionLine::Vertical(0);
const ZERO_HORIZONTAL_REFLECTION: ReflectionLine = ReflectionLine::Horizontal(0);

impl ReflectionLine {

    fn to_horizontal(&self) -> Self {
        match self {
            Self::Horizontal(_) => panic!("Horizontal to horizontal invalid map"),
            Self::Vertical(n) => Self::Horizontal(*n)
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Vertical(n) => *n,
            Self::Horizontal(n) => *n * 100
        }
    }

    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}


impl Direction {
    fn is_left(&self) -> bool {
        match self {
            Direction::Right => false,
            _ => true,
        }
    }
    fn is_right(&self) -> bool {
        match self {
            Direction::Left => false,
            _ => true,
        }
    }
}

pub fn process(lines: Vec<String>, day_part: DayPart) -> Result<usize> {
    let mut sum: usize = 0;
    let mut line_aggregator: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            sum += process_part(&line_aggregator.as_slice(), &day_part);
            line_aggregator.clear();
        } else {
            line_aggregator.push(line);
        }
    }
    sum += process_part(&line_aggregator.as_slice(), &day_part);
    Ok(sum)
}

fn process_part(lines: &[String], day_part: &DayPart) -> usize {
    match day_part {
        DayPart::One => {
            let result = process_all(lines);
            result.0.value() + result.1.value()
        },
        DayPart::Two => process_permutations(lines)
    }
}

fn process_permutations(block: &[String]) -> usize {
    let mut max = 0;
    let mut sum = 0;
    let col_size = block.first().map_or(0, |l| l.len());
    let original_result = process_all(block);
    let original_reflection = get_real_reflection(&original_result);
    for row in 0..block.len() {
        for col in 0..col_size {
            let mutated_line = mutate_col(&block[row], col);
            let new_block = agg_block(block, &mutated_line, row);
            let mutated_results = process_all(&new_block);
            if original_reflection != mutated_results.0  {
                let mutated_result = mutated_results.0;
                sum += mutated_result.value();
                if mutated_result.value() > max {
                    //println!("Found {:?} for row {} and col {}", 
                     //   mutated_result, row, col);
                    max = mutated_result.value();
                }
            }
            if original_reflection != mutated_results.1  {
                let mutated_result = mutated_results.1;
                sum += mutated_result.value();
                if mutated_result.value() > max {
                    println!("Found {:?} for row {} and col {}", 
                        mutated_result, row, col);
                    max = mutated_result.value();
                }
            }
        }
    }
    sum
}

fn get_real_reflection(reflections: &(ReflectionLine, ReflectionLine)) 
-> ReflectionLine {
    if reflections.0.value() > 0 {
        reflections.0
    }else {
        reflections.1
    }
}


fn agg_block(block: &[String], new_line: &str, row: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(block.len());
    for (i, line) in block.iter().enumerate() {
        if i == row {
            result.push(new_line.to_string());
        }else {
            result.push(line.to_string());
        }
    }
    result
}

fn mutate_col(line: &str, col: usize) -> String {
    let mut result = String::with_capacity(line.len());
    for (i, c) in line.chars().enumerate() {
        if i == col {
            match c {
                '.' => result.push('#'),
                '#' => result.push('.'),
                _ => panic!("Illegal state")
            }
        }else{
            result.push(c);
        }
    }
    result
}


fn process_all(block: &[String]) -> (ReflectionLine, ReflectionLine) { // (Vertical, Horizontal)
    let vertical = process_vertical(block);
    let horizontal = process_horizontal(block);
    (vertical, horizontal)
}

fn process_horizontal(block: &[String]) -> ReflectionLine {
    let block_rotated = rotate(block);
    let result = process_vertical(&block_rotated);
    match result {
        ReflectionLine::Vertical(n) => result.to_horizontal(),
        ReflectionLine::Horizontal(_) => panic!("Invalid state")
    }
}

fn process_vertical(block: &[String]) -> ReflectionLine {
    if block.is_empty() {
        return ZERO_VERTICAL_REFLECTION;
    }

    //println!("Current block [{:?}]", block);

    let mut result: HashSet<Pair> = get_palindromes(&block[0], 0, Direction::Both);
    for line in block.iter().skip(1) {
        let current = get_palindromes(line, 0, Direction::Both);
        //println!("Current line {}, palindromes {:?}", line, current);
        result = result
            .intersection(&current)
            //.filter(|(a,b)| a+b >= line.len()-1)
            .map(|p| p.clone())
            .collect();
    }
    if result.len() > 1 {
        println!("Intersection result {:?}", result);
    }

    let max = result
        .iter()
        .max_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            a.1.cmp(&b.1)
        });

    match max {
        Some(p) => ReflectionLine::Vertical(p.0 + (p.1 / 2)),
        _ => ZERO_VERTICAL_REFLECTION
    }
}

fn rotate(block: &[String]) -> Vec<String> {
    if block.is_empty() {
        return vec![];
    }
    let size = block[0].len();
    let mut all_iters = vec![String::new(); size];

    for i in 0..size {
        for s in block {
            all_iters[i].push(s.chars().nth(i).unwrap());
        }
    }
    all_iters
}

fn get_palindromes(line: &str, offset: usize, direction: Direction) -> HashSet<Pair> {
    let mut set: HashSet<Pair> = HashSet::new();
    //println!("Calculating palindromes for {} direction {:?}", line, direction);
    if line.is_empty() {
        return set;
    }
    if is_palindrome(line) {
        let pair = (offset, line.len());
        //println!("Palindromes for {} direction {:?} pair {:?}",
        //    line, direction, pair);
        set.insert(pair);
        //return set;
    }
    if direction.is_right() {
        let right_window = get_palindromes(&line[1..], offset + 1, Direction::Right);
        set.extend(right_window);
    }
    if direction.is_left() {
        let left_window = get_palindromes(&line[..line.len() - 1], offset, Direction::Left);
        set.extend(left_window);
    }
    //println!("Palindromes for {} direction {:?} palindromes {:?}",
    //    line, direction, set);
    set
}

fn is_palindrome(line: &str) -> bool {
    line.len() > 1
        && line.len() % 2 == 0
        && line.chars().zip(line.chars().rev()).all(|(o, r)| o == r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        test_line(input, 5);
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        test_line(input, 400);
        let input = ".#.##.#...#####
.##..##.##..##.
.######.#..#.##
##.##.###.##.##
.##..##.#..#...
##.##.##.#..###
#......###.#.##
.#....#.#####..
.######.##.#.##
..####..#..####
##....##.###.##
.#....#...#.###
##....###.##.##
..........#.#..
#########.#....";
        test_line(input, 4);
    }

    fn test_line(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process(lines, DayPart::One);
        assert_eq!(expect, result.unwrap());
    }

    fn test_line_part_two(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process(lines, DayPart::Two);
        assert_eq!(expect, result.unwrap());
    }

    #[test]
    fn test_part_two_simple_input() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        test_line_part_two(input, 400);
    }
}
