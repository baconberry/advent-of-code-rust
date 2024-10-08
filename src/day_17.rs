use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::RangeInclusive;
use std::usize;

use anyhow::{bail, Result};
use num::iter::Range;
use num::One;

pub fn process(lines: &[String], day_part: usize) -> Result<usize> {
    match day_part {
        1 => Ok(process_with_range(lines, 0..=3)),
        2 => Ok(process_with_range(lines, 4..=10)),
        _ => todo!(),
    }
}

type HeatMap = Vec<u8>;
type HeatPath = Vec<usize>;

fn process_with_range(lines: &[String], directional_range: RangeInclusive<u8>) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let height = lines.len();
    let width = lines[0].len();
    let mut map: HeatMap = Vec::with_capacity(height * width);
    for line in lines {
        for c in line.chars() {
            map.push(c.to_digit(10).unwrap() as u8);
        }
    }
    let end_pos = get_pos(width - 1, height - 1, width);
    if let Some(map_path) = find_shortest_path(&map, height, width, 0, end_pos, directional_range) {
        println!("Result: {:?}", map_path);
        return map_path.heat_loss;
    }
    0
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn is_oposite(&self, dir: &Direction) -> bool {
        match (&self, dir) {
            (Direction::Right, Direction::Left)
            | (Direction::Left, Direction::Right)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up) => true,
            (_, _) => false,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct MapPath {
    //path: HeatPath,
    position: usize,
    heat_loss: usize,
    same_direction_len: u8,
    direction: Direction,
}
impl Ord for MapPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}
impl PartialOrd for MapPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl MapPath {
    fn new(
        position: usize,
        same_direction_len: u8,
        direction: Direction,
        heat_loss: usize,
    ) -> Self {
        Self {
            position,
            heat_loss,
            same_direction_len,
            direction,
        }
    }

    fn move_one(&self, new_pos: usize, new_dir: &Direction, pos_heat_loss: u8) -> Self {
        let direction_len = if self.direction == *new_dir {
            self.same_direction_len + 1
        } else {
            1
        };

        Self::new(
            new_pos,
            direction_len,
            new_dir.clone(),
            self.heat_loss + pos_heat_loss as usize,
        )
    }
}
fn find_shortest_path(
    map: &HeatMap,
    height: usize,
    width: usize,
    start_pos: usize,
    end_pos: usize,
    move_range: RangeInclusive<u8>,
) -> Option<MapPath> {
    let max_offset = *move_range.end() as usize * 9;
    let mut path_queue: BinaryHeap<MapPath> = BinaryHeap::new();
    let mut dist: Vec<usize> = (0..map.len()).map(|_| usize::MAX - max_offset).collect();
    let mut visited = HashSet::new();
    let mut path_queue: BinaryHeap<MapPath> = BinaryHeap::new();
    let all_dirs = Direction::all();
    dist[start_pos] = 0;
    for dir in &all_dirs {
        if is_dir_possbile(start_pos, width, height, &dir) {
            let starting = MapPath::new(start_pos, 1, dir.clone(), 0);
            path_queue.push(starting);
        }
    }

    let mut result_path: Option<MapPath> = None;
    let mut iter_count: usize = 0;
    while let Some(path) = path_queue.pop() {
        let last_pos = path.position;
        if path.same_direction_len > *move_range.end() {
            continue;
        }
        let current_in_range = move_range.contains(&path.same_direction_len);
        if path.heat_loss < dist[last_pos] {
            dist[last_pos] = path.heat_loss;
        }
        iter_count += 1;
        if iter_count % 1000 == 0 {
            println!(
                "Iteration [{}] queue len [{}], current heat_loss [{}]",
                iter_count,
                path_queue.len(),
                path.heat_loss
            );
        }

        if current_in_range && last_pos == end_pos {
            if let Some(prev) = &result_path {
                if path.heat_loss < prev.heat_loss {
                    result_path = Some(path)
                }
            } else {
                result_path = Some(path)
            };
            continue;
        }

        for dir in &all_dirs {
            if path.direction.is_oposite(dir) {
                continue;
            }
            if !is_dir_possbile(last_pos, width, height, dir) {
                continue;
            }
            let new_pos = get_move_pos(last_pos, width, dir);
            let new_path = path.move_one(new_pos, dir, map[new_pos]);
            let is_in_range = move_range.contains(&new_path.same_direction_len);
            if *dir != path.direction && !current_in_range {
                continue;
            }
            if new_path.heat_loss < dist[new_pos] + max_offset {
                if visited.insert((new_pos, dir, new_path.same_direction_len)) {
                    path_queue.push(new_path);
                }
            }
        }
    }
    result_path
}

fn get_move_pos(pos: usize, width: usize, dir: &Direction) -> usize {
    let (x, y) = get_coordenates(pos, width);
    let (x, y) = match dir {
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y),
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
    };
    get_pos(x, y, width)
}

fn is_dir_possbile(pos: usize, width: usize, height: usize, direction: &Direction) -> bool {
    let (x, y) = get_coordenates(pos, width);
    match direction {
        Direction::Up => {
            if y == 0 {
                return false;
            }
        }
        Direction::Left => {
            if x == 0 {
                return false;
            }
        }
        Direction::Right => {
            if x == (width - 1) {
                return false;
            }
        }
        Direction::Down => {
            if y == (height - 1) {
                return false;
            }
        }
    }
    true
}
fn get_coordenates(pos: usize, width: usize) -> (usize, usize) {
    (pos % width, num::Integer::div_floor(&pos, &width))
}

fn get_pos(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(&lines, 1);
        assert_eq!(102, result.unwrap());
        let result = process(&lines, 2);
        assert_eq!(94, result.unwrap());
    }
    #[test]
    fn test_simple_input_99() {
        let input = "99999
99999
99999
99999
99999";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(&lines, 1);
        assert_eq!(72, result.unwrap());
    }
    #[test]
    fn test_simple_input_day2_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(&lines, 2);
        assert_eq!(71, result.unwrap());
    }
}
