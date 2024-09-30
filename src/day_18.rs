use core::panic;
use std::usize;

use crate::prelude::*;
use anyhow::{bail, Result};

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
        _ => panic!(),
    }
}

pub fn process_one(lines: &[String]) -> Result<usize> {
    let end_coord = get_bounds(lines);
    //println!("Starting grid with end coord {:?}", end_coord);
    let mut grid: Grid<Block> =
        Grid::init((end_coord.x * 2) + 1, (end_coord.y * 2) + 1, Block::Empty);
    let mut position = end_coord;
    grid.set(&position, Block::Wall);
    for line in lines {
        let (dir, len) = get_line_components(line).unwrap();
        //println!("Parsed components {:?} {:?} from line {}", dir, len, line);
        let delta = dir.coord_delta();
        for m in 0..len {
            //println!("Position {:?}, moving delta {:?}", position, delta);
            position = position.plus_delta(delta).unwrap();
            grid.set(&position, Block::Wall);
        }
    }

    let edge_reachable = grid.get_edge_reachable_pos_set(&Block::Wall);
    //println!("Edge reachable [{:?}]", edge_reachable);
    for ele in edge_reachable {
        grid.data[ele] = Block::EdgeReachable;
    }
    return Ok(grid.count_eq(&Block::Empty) + grid.count_eq(&Block::Wall));

    //grid.print();

    //grid = mark_inner(&grid, &Block::Wall, &Block::Hole);
    //println!("\nafter 0 rotation + marking");
    //grid.print();
    //for i in 1..4{
    //    grid = grid.rotate();
    //    grid = mark_inner(&grid, &Block::Wall, &Block::Hole);
    //    println!("\nafter {i} rotation + marking");
    //grid.print();
    //}
    //Ok(grid.count_ne(&Block::Empty))
}

// walks from west to east ,north to south,
// the first orruccence of a C will open a parenthesis like logic
// and it starts marking M until another occurrence of C happens
pub fn mark_inner<E: ToString + Clone + PartialEq>(grid: &Grid<E>, c: &E, m: &E) -> Grid<E> {
    let mut new_grid: Grid<E> = grid.clone();
    for y in 1..grid.height - 1 {
        let mut is_open = false;
        let mut last_open_pos = 0;
        for x in 0..grid.width {
            let pos = (grid.width * y) + x;
            let elem = &new_grid.data[pos];
            if c == elem {
                is_open = !is_open;
                if is_open {
                    last_open_pos = pos;
                } else {
                    // mark from last open to pos -1
                    if last_open_pos == pos - 1 {
                        continue;
                    }
                    for i in last_open_pos + 1..pos {
                        new_grid.data[i] = m.clone();
                    }
                }
            }
        }
    }
    new_grid
}

fn get_bounds(lines: &[String]) -> Coord {
    let start_pos = usize::MAX / 2;
    let mut pos = Coord::from(start_pos, start_pos);
    let mut min_left = start_pos;
    let mut max_right = start_pos;
    let mut max_up = start_pos;
    let mut min_down = start_pos;

    for line in lines {
        let (dir, len) = get_line_components(line).unwrap();
        let (d_x, d_y) = dir.coord_delta();
        let (d_x, d_y) = (d_x * len as i32, d_y * len as i32);

        pos = pos.plus_delta((d_x, d_y)).unwrap();
        if pos.x < min_left {
            min_left = pos.x;
        }
        if pos.x > max_right {
            max_right = pos.x;
        }

        if pos.y < min_down {
            min_down = pos.y;
        }
        if pos.y > max_up {
            max_up = pos.y;
        }
    }

    Coord::from(max_right - min_left, max_up - min_down)
}

fn get_line_components(line: &str) -> Result<(Direction, usize)> {
    let mut split = line.split(" ");
    let dir = split.next().unwrap().parse::<Direction>().unwrap();
    let len = split.next().unwrap().parse::<usize>().unwrap();
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
    fn test_bounds() {
        let lines = utils::string_to_lines(LINES_1.to_string());
        let result = get_bounds(&lines);
        let e = Coord::from(7, 10);
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 9);
    }
}
