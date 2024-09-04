use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{bail, Result};
use num::Integer;
use rayon::{slice::Windows, spawn};

use crate::{DayPart, DayProblem};

#[derive(Debug)]
enum SpaceType {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct ParseSpaceTypeError;

impl SpaceType {
    fn from_char(s: char) -> Result<Self> {
        match s {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::RightMirror),
            '\\' => Ok(Self::LeftMirror),
            '-' => Ok(Self::HorizontalSplitter),
            '|' => Ok(Self::VerticalSplitter),
            _ => bail!("{} is not valid SpaceType", s),
        }
    }
}

type SpaceId = usize;
#[derive(Debug)]
struct Space {
    id: SpaceId,
    space_type: SpaceType,
    up: Option<SpaceId>,
    down: Option<SpaceId>,
    left: Option<SpaceId>,
    right: Option<SpaceId>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            id: 0,
            space_type: SpaceType::Empty,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}

impl Space {
    fn new_alone(space_type: SpaceType) -> Self {
        Self {
            space_type,
            ..Default::default()
        }
    }

    fn from_id(id: SpaceId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    fn set_right(&mut self, id: SpaceId) {
        self.right = Some(id);
    }
    fn set_left(&mut self, id: SpaceId) {
        self.left = Some(id);
    }
    fn set_up(&mut self, id: SpaceId) {
        self.up = Some(id);
    }
    fn set_down(&mut self, id: SpaceId) {
        self.down = Some(id);
    }

    fn next_from_direction(&self, direction: &Direction) -> Option<SpaceId> {
        match direction {
            Direction::Right => self.right.clone(),
            Direction::Left => self.left.clone(),
            Direction::Up => self.up.clone(),
            Direction::Down => self.down.clone(),
        }
    }
}

pub fn process(lines: Vec<String>, part: DayPart) -> Result<usize> {
    match part {
        DayPart::One => Ok(process_one(&lines, 0, &Direction::Right)),
        DayPart::Two => Ok(process_corners(&lines)),
    }
}

#[derive(Debug)]
struct SpaceMap {
    height: usize,
    width: usize,
    max_position: usize,
    map: Vec<Space>,
}

impl SpaceMap {
    fn new(height: usize, width: usize) -> Self {
        let max_position = (height * width);
        let mut map: Vec<Space> = Vec::with_capacity(max_position);
        for i in 0..=max_position {
            map.push(Space::from_id(i));
        }
        Self {
            height,
            width,
            map,
            max_position,
        }
    }

    fn add(&mut self, c: char, row: usize, col: usize) {
        let pos = self.get_position(row, col);
        let space_type = SpaceType::from_char(c);
        self.map[pos].space_type = space_type.unwrap();
    }

    fn populate_neighbors(&mut self) {
        let width = self.width;
        for i in 0..self.max_position {
            let (row, col) = get_coordinates(i, width);
            // only populate bidirectional right and down neighbours
            if col < (self.width - 1) {
                let right_pos = get_position(row, col + 1, width);
                self.map[i].set_right(right_pos);
                self.map[right_pos].set_left(i);
            }

            if row < (self.height - 1) {
                let down_pos = get_position(row + 1, col, width);
                self.map[i].set_down(down_pos);
                self.map[down_pos].set_up(i);
            }
        }
    }

    fn get_coordinates(&self, pos: usize) -> (usize, usize) {
        let col = pos % self.width;
        let row = num::integer::div_floor(pos, self.width) as usize;
        (row, col)
    }

    fn get_position(&self, row: usize, col: usize) -> usize {
        let pos = (row * self.width) + col;
        if pos < self.max_position {
            return pos;
        }
        panic!("Position not valid {}, max pos {}", pos, self.max_position)
    }
}

fn get_coordinates(pos: usize, width: usize) -> (usize, usize) {
    let col = pos % width;
    let row = num::integer::div_floor(pos, width) as usize;
    (row, col)
}

fn get_position(row: usize, col: usize, width: usize) -> usize {
    let pos = (row * width) + col;
    return pos;
}
fn parse_map(lines: &[String]) -> SpaceMap {
    let map_height = lines.len();
    let map_width = lines[0].len();
    let mut space_map = SpaceMap::new(map_height, map_width);
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            space_map.add(c, row, col);
        }
    }
    space_map.populate_neighbors();
    space_map
}

fn process_corners(lines: &[String]) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let space_map = parse_map(lines);

    let mut corners: Vec<(SpaceId, Direction)> = vec![];
    let width = space_map.width;
    let height = space_map.height;
    for i in 0..space_map.max_position {
        // top row, down
        if i < width {
            corners.push((i, Direction::Down));
        }
        // bottom row, up
        if i > (height - 1) * width {
            corners.push((i, Direction::Up));
        }
        // left col, right
        if i % width == 0 {
            corners.push((i, Direction::Right));
        }
        // right col, left
        if i % width == width - 1 {
            corners.push((i, Direction::Left));
        }
    }
    corners
        .iter()
        .map(|(pos, dir)| beam_walker(&space_map, *pos, dir))
        .max()
        .unwrap_or(0)
}
fn process_one(lines: &[String], start_pos: SpaceId, start_direction: &Direction) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let space_map = parse_map(lines);
    //println!("Populated map [{:?}]", space_map);
    beam_walker(&space_map, start_pos, start_direction)
}

fn beam_walker(space_map: &SpaceMap, start_pos: usize, start_direction: &Direction) -> usize {
    let capacity = space_map.max_position;
    let mut energized: HashSet<SpaceId> = HashSet::with_capacity(capacity);
    let mut up_beam: HashSet<SpaceId> = HashSet::with_capacity(capacity);
    let mut down_meam: HashSet<SpaceId> = HashSet::with_capacity(capacity);
    let mut left_beam: HashSet<SpaceId> = HashSet::with_capacity(capacity);
    let mut right_beam: HashSet<SpaceId> = HashSet::with_capacity(capacity);
    let mut beam_map: HashMap<Direction, HashSet<SpaceId>> = HashMap::new();
    beam_map.insert(Direction::Up, up_beam);
    beam_map.insert(Direction::Down, down_meam);
    beam_map.insert(Direction::Left, left_beam);
    beam_map.insert(Direction::Right, right_beam);
    let mut queue: VecDeque<(SpaceId, Direction)> = VecDeque::new();
    queue.push_front((start_pos, start_direction.clone()));
    while !queue.is_empty() {
        let (id, direction) = queue.pop_front().unwrap();
        energized.insert(id);
        let space = &space_map.map[id];
        match (&space.space_type, &direction) {
            // to continue thru
            (SpaceType::Empty, _)
            | (SpaceType::HorizontalSplitter, Direction::Right)
            | (SpaceType::HorizontalSplitter, Direction::Left)
            | (SpaceType::VerticalSplitter, Direction::Up)
            | (SpaceType::VerticalSplitter, Direction::Down) => {
                let next = space.next_from_direction(&direction);
                if let Some(n) = next {
                    let mut set = beam_map.get_mut(&direction).unwrap();
                    if !set.contains(&n) {
                        set.insert(n);
                        queue.push_front((n, direction));
                    }
                }
            }
            (SpaceType::VerticalSplitter, Direction::Right)
            | (SpaceType::VerticalSplitter, Direction::Left) => {
                if let Some(u) = space.up {
                    let mut set = beam_map.get_mut(&Direction::Up).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Up));
                    }
                }
                if let Some(d) = space.down {
                    let mut set = beam_map.get_mut(&Direction::Down).unwrap();
                    if !set.contains(&d) {
                        set.insert(d);
                        queue.push_front((d, Direction::Down));
                    }
                }
            }
            (SpaceType::LeftMirror, Direction::Right)
            | (SpaceType::RightMirror, Direction::Left) => {
                if let Some(d) = space.down {
                    let mut set = beam_map.get_mut(&Direction::Down).unwrap();
                    if !set.contains(&d) {
                        set.insert(d);
                        queue.push_front((d, Direction::Down));
                    }
                }
            }
            (SpaceType::LeftMirror, Direction::Left)
            | (SpaceType::RightMirror, Direction::Right) => {
                if let Some(u) = space.up {
                    let mut set = beam_map.get_mut(&Direction::Up).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Up));
                    }
                }
            }
            (SpaceType::LeftMirror, Direction::Down) | (SpaceType::RightMirror, Direction::Up) => {
                if let Some(u) = space.right {
                    let mut set = beam_map.get_mut(&Direction::Right).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Right));
                    }
                }
            }
            (SpaceType::RightMirror, Direction::Down) | (SpaceType::LeftMirror, Direction::Up) => {
                if let Some(u) = space.left {
                    let mut set = beam_map.get_mut(&Direction::Left).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Left));
                    }
                }
            }
            (SpaceType::HorizontalSplitter, Direction::Down)
            | (SpaceType::HorizontalSplitter, Direction::Up) => {
                if let Some(u) = space.left {
                    let mut set = beam_map.get_mut(&Direction::Left).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Left));
                    }
                }
                if let Some(u) = space.right {
                    let mut set = beam_map.get_mut(&Direction::Right).unwrap();
                    if !set.contains(&u) {
                        set.insert(u);
                        queue.push_front((u, Direction::Right));
                    }
                }
            }

            (_, _) => todo!("{:?},{:?} not implemented", &space.space_type, &direction),
        }
    }
    //print_energized(&energized, space_map.height, space_map.width);
    energized.len()
}

fn print_energized(energized: &HashSet<SpaceId>, height: usize, width: usize) {
    let mut line = String::with_capacity(width);
    for row in 0..height {
        for col in 0..width {
            let id: usize = (row * width) + col;
            if energized.contains(&id) {
                line.push_str("#");
            } else {
                line.push_str(".");
            }
        }
        //println!("{}", line);
        line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(lines, DayPart::One);
        assert_eq!(46, result.unwrap());
    }

    #[test]
    fn test_simple_input_two() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(lines, DayPart::Two);
        assert_eq!(51, result.unwrap());
    }
}
