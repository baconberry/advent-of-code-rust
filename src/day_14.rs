use core::panic;
use std::cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;
use std::sync::atomic::AtomicUsize;

use crate::col_utils;
use crate::prelude::*;
use crate::re_utils;
use crate::utils::rotate_block;
use anyhow::Result;
use rayon::iter::Map;

type Loc = (i64, i64);

const TOTAL_CYCLES: usize = 1000000000;
const MEMO_HIT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq, Hash, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        // x, y
        match (self) {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    fn is_start_backwards(&self) -> bool {
        let delta = self.delta();

        delta.0 > 0 || delta.1 > 0
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
enum CellType {
    Empty,
    Round,
    Cube,
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Cell {
    cell_type: CellType,
}

impl Cell {
    fn is_round(&self) -> bool {
        match (self.cell_type) {
            CellType::Round => true,
            _ => false,
        }
    }
    
    fn is_cube(&self) -> bool {
        match (self.cell_type) {
            CellType::Cube => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match (self.cell_type) {
            CellType::Empty => true,
            _ => false,
        }
    }

    fn char(&self) -> char {
        match (self.cell_type) {
            CellType::Cube => '#',
            CellType::Empty => '.',
            CellType::Round => 'O'
        }
    }
}


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

type CellMap = HashMap<Loc, Cell>;

fn map_to_str(cell_map: &CellMap, max_loc: (usize, usize)) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for x in 0..max_loc.0 {
        let mut text = String::new();
        for y in 0..max_loc.1 {
            let loc = (x as i64, y as i64);
            let cell = cell_map.get(&loc).unwrap();
            text.push(cell.char());
        }
        result.push(text);
    }

    result
}

fn map_to_line(cell_map: &CellMap, max_loc: (usize, usize)) -> String {
    let mut result = String::new();
    for x in 0..max_loc.0 {
        let mut text = String::new();
        for y in 0..max_loc.1 {
            let loc = (x as i64, y as i64);
            let cell = cell_map.get(&loc).unwrap();
            text.push(cell.char());
        }
    }
    result
}

type MemoKey = (Direction, String);
type MemoMap = HashMap<String, usize>;
fn process_block(lines: &[String], day_part: &DayPart) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let mut cell_map: HashMap<Loc, Cell> = parse_map(lines);
    let max_loc = (lines.len(), lines[0].len());
    let mut memo: MemoMap = HashMap::new();
    
    if day_part.is_one() {
        push_rocks(&cell_map, Direction::North, max_loc);
        //let map_as_text = map_to_str(&cell_map, max_loc);
        return calculate_north_load(&cell_map, max_loc);
    }

    let mut i: usize = 0;
    let mut is_cycle_found: bool = false;
    loop { 
        if i >= TOTAL_CYCLES {
            break;
        }
        let memo_key = map_to_line(&cell_map, max_loc);
        if !is_cycle_found && memo.contains_key(&memo_key) {
            let cycle_start = memo.get(&memo_key).unwrap();
            println!("Found cycle at iteration [{}], cycle start [{}]", i, cycle_start);
            let mod_cycle = TOTAL_CYCLES % cycle_start;
            let new_i = TOTAL_CYCLES - mod_cycle;
            i = new_i;
            is_cycle_found = true;
        }
        let cycled_map = Some(&cell_map)
            .map(|cm|push_rocks(&cm, Direction::North, max_loc))
            .map(|cm|push_rocks(&cm, Direction::West, max_loc))
            .map(|cm|push_rocks(&cm, Direction::South, max_loc))
            .map(|cm|push_rocks(&cm, Direction::East, max_loc))
            .unwrap();
        let memo_key = map_to_line(&cycled_map, max_loc);
        memo.insert(memo_key, i-1);
        i += 1;

        cell_map = cycled_map;
    }

    calculate_north_load(&cell_map, max_loc)
}

fn push_rocks(cell_map_source: &CellMap,
    direction: Direction,
    max_loc: (usize, usize)) -> CellMap {
    let mut cell_map = cell_map_source.clone();
    let delta = direction.delta();
    let max_x = max_loc.0 as i64;
    let max_y = max_loc.1 as i64;
    for x in 0..max_loc.0 {
        let real_x = if direction.is_start_backwards() {
            max_loc.0 - x -1
        } else {
            x
        };
        for y in 0..max_loc.1 {
            let real_y = if direction.is_start_backwards() {
                max_loc.1 - y -1
            } else {
                y
            };
            let mut loc = (real_x as i64, real_y as i64);
            let cell_in_loc = cell_map.get(&loc).unwrap().clone();
            if cell_in_loc.is_round() {
                loop {
                    let loc_delta = (loc.0 + delta.0, loc.1 + delta.1);
                    if loc_delta.0 < 0 || loc_delta.1 < 0 {
                        break;
                    }
                    if loc_delta.0 >= max_x || loc_delta.1 >= max_y {
                        break;
                    }
                    let cell_in_delta = cell_map.get(&loc_delta).unwrap().clone();
                    if !cell_in_delta.is_empty() {
                        break;
                    }
                    //swap
                    cell_map.insert(loc_delta, cell_in_loc.clone());
                    cell_map.insert(loc, cell_in_delta);
                    loc = loc_delta;
                }
            }
        }
    }
    cell_map
}

fn parse_map(lines: &[String]) -> HashMap<Loc, Cell> {
    let mut map: HashMap<Loc, Cell> = HashMap::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let loc = (x as i64, y as i64);
            let cell_type = match (c) {
                '.' => CellType::Empty,
                'O' => CellType::Round,
                '#' => CellType::Cube,
                _ => panic!("Could not parse invalid cell type"),
            };
            let cell = Cell { cell_type };

            map.insert(loc, cell);
        }
    }
    map
}

fn calculate_north_load(cell_map: &CellMap, max_loc: (usize, usize)) -> usize {
    let mut total: usize = 0;
    for y in 0..max_loc.1 {
        let mut count = 0;
        let mut offset = 0;
        for x in 0..max_loc.0 {
            let loc = (x as i64, y as i64);
            let cell = cell_map.get(&loc).unwrap();
            if cell.is_cube() {
                total += calculate_value(count, max_loc.1 - offset);
                offset = x + 1;
                count = 0;
            } else if cell.is_round() {
                count += 1;
            }
        }
        total += calculate_value(count, max_loc.1 - offset);
    }
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

    fn test_line_2(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process(lines, DayPart::Two);
        assert_eq!(expect, result.unwrap());
    }

    #[test]
    fn test_calculate_value() {
        assert_eq!(34, calculate_value(4, 10));
    }

    fn test_simple_input_day_2() {
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
        test_line_2(input, 64);
    }
}
