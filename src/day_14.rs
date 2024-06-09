use core::panic;
use std::collections::HashSet;
use std::ops::Index;

use crate::col_utils;
use crate::prelude::*;
use crate::re_utils;
use crate::utils::rotate_block;
use anyhow::Result;

pub fn process(lines: Vec<String>, day_part: DayPart) -> Result<usize> {
    let mut sum: usize = 0;
    let mut line_aggregator: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            sum += process_block(&line_aggregator.as_slice(), &day_part);
            line_aggregator.clear();
        } else {
            line_aggregator.push(line);
        }
    }
    sum += process_block(&line_aggregator.as_slice(), &day_part);
    Ok(sum)
}

fn process_block(lines: &[String], day_part: &DayPart) -> usize {
    let rotated = rotate_block(lines);
    rotated.iter().map(|line| push_left(line)).sum()
}

fn push_left(line: &str) -> usize {
    if line.is_empty() {
        return 0;
    }
    let mut total: usize = 0;
    let mut count: usize = 0;
    let mut offset: usize = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '#' {
            total += calculate_value(count, line.len() - offset);
            offset = i + 1;
            count = 0;
        }
        if c == 'O' {
            count += 1;
        }
    }

    total += calculate_value(count, line.len() - offset);
    total
}

fn calculate_value(o_count: usize, line_len: usize) -> usize {
    let upper = line_len * (line_len + 1);
    let upper = upper / 2;
    let lower = line_len - o_count;
    let lower = lower * (lower + 1);
    let lower = lower / 2;
    upper - lower
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        test_line(input, 136);
    }

    fn test_line(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process(lines, DayPart::One);
        assert_eq!(expect, result.unwrap());
    }

    #[test]
    fn test_calculate_value() {
        assert_eq!(34, calculate_value(4, 10));
    }
}
