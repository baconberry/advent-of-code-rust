use core::panic;
use std::{collections::HashMap, usize};

use crate::{prelude::*, shoelace::{self, Vertx}};
use anyhow::{bail, Result};
use num::Num;

#[derive(Clone, Debug, PartialEq)]
enum Block {
    Empty,
    Wall,
    Hole,
    EdgeReachable,
}

impl ToString for Block {
    fn to_string(&self) -> String {
        //format!("{:?}", self)
        match self {
            Block::Wall => "W",
            Block::Hole => "H",
            Block::Empty => ".",
            Block::EdgeReachable => "+",
        }
        .to_string()
    }
}

pub fn process(lines: &[String], day_part: usize) -> Result<usize> {
    match day_part {
        1 => process_one(lines),
        2 => process_two(lines),
        _ => panic!(),
    }
}

pub fn process_two(lines: &[String]) -> Result<usize> {
    let mut instructions: Vec<(Direction, usize)> = Vec::new();
    for line in lines {
        let mut split = line.split(" ");
        let rgb = split.nth(2).unwrap();
        let rgb = &rgb[2..8];
        instructions.push(get_line_components_from_rgb(rgb).unwrap());
    }
    process_instructions(&instructions)
}
pub fn process_one(lines: &[String]) -> Result<usize> {
    let mut instructions: Vec<(Direction, usize)> = Vec::new();
    for line in lines {
        instructions.push(get_line_components(line).unwrap());
    }
    process_instructions(&instructions)
}

pub fn process_instructions(instructions: &[(Direction, usize)]) -> Result<usize> {
    println!("Processing instructions [{:?}]", instructions);
    let mut position = Coord::from(0,0);
    let mut circumference = 0;
    let mut vertxs: Vec<Vertx> = Vec::with_capacity(instructions.len());
    for (id, line) in instructions.iter().enumerate() {
        let (dir, len) = line;
        let delta = dir.coord_delta();
        circumference += len;
        let new_pos = position.plus_delta(delta, *len as isize).unwrap();
        let edge_to = if id == instructions.len()-1 { 0 } else { id+1};
        vertxs.push(Vertx::new(id, new_pos.x as isize, new_pos.y as isize, edge_to));
        position = new_pos;
    }
    let map = vertxs
        .iter()
        .map(|v| (v.id, v.clone()))
        .collect::<HashMap<usize, Vertx>>();
    let area = shoelace::calculate_area(&map, 0).unwrap();
    let res = area + (circumference as f64 / 2.0) + 1.0;

     Ok(res as usize)
}


fn get_line_components(line: &str) -> Result<(Direction, usize)> {
    let mut split = line.split(" ");
    let dir = split.next().unwrap().parse::<Direction>().unwrap();
    let len = split.next().unwrap().parse::<usize>().unwrap();
    Ok((dir, len))
}
fn get_line_components_from_rgb(rgb: &str) -> Result<(Direction, usize)> {
    let len = Num::from_str_radix(&rgb[0..rgb.len() - 1], 16).unwrap();
    let dir = match &rgb[rgb.len() - 1..] {
        "0" => "R",
        "1" => "D",
        "2" => "L",
        "3" => "U",
        _ => bail!("Illegal argument for direction"),
    }
    .parse::<Direction>()
    .unwrap();

    Ok((dir, len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    const LINES_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_simple_input() {
        let lines = utils::string_to_lines(LINES_1.to_string());
        let result = process(&lines, 1);
        assert_eq!(62, result.unwrap());
    }
    #[test]
    fn test_simple_two() {
        let lines = utils::string_to_lines(LINES_1.to_string());
        let result = process(&lines, 2);
        assert_eq!(952408144115, result.unwrap());
    }
}
