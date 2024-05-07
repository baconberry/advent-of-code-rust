use std::collections::HashSet;

use crate::col_utils;
use crate::re_utils;

use anyhow::Result;

type Pair = (usize, usize);

#[derive(Debug)]
enum Direction {
    Both,
    Left,
    Right,
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

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let _galaxy_char = '#';
    let mut sum: usize = 0;
    let mut line_aggregator: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            sum += process(&line_aggregator.as_slice());
            line_aggregator.clear();
        } else {
            line_aggregator.push(line);
        }
    }
    sum += process(&line_aggregator.as_slice());
    Ok(sum)
}

fn process(block: &[String]) -> usize {
    let vertical = process_vertical(block);
    let horizontal = process_horizontal(block);
    horizontal + vertical
}

fn process_horizontal(block: &[String]) -> usize {
    let block_rotated = rotate(block);
    100 * process_vertical(&block_rotated)
}

fn process_vertical(block: &[String]) -> usize {
    if block.is_empty() {
        return 0;
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
    //println!("Intersection result {:?}", result);

    let max = result
        .iter()
        .max_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            a.1.cmp(&b.1)
        })
        .map(|p| p.0 + (p.1 / 2));

    match max {
        Some(m) => m,
        _ => 0,
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
        let result = process_lines(lines);
        assert_eq!(expect, result.unwrap());
    }
}
