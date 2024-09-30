use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{bail, Result};
use num::{Integer, Signed};

#[derive(Clone)]
pub struct Grid<E> {
    pub data: Vec<E>,
    pub width: usize,
    pub height: usize,
}

impl<E> Grid<E> {
    pub fn set(&mut self, coord: &Coord, item: E) {
        self.data[(coord.y * self.width) + coord.x] = item;
    }
    pub fn get_end_coord(&self) -> Coord {
        Coord::from(self.width - 1, self.height - 1)
    }

    pub fn get_neighbors_pos(&self, pos: usize) -> Vec<usize> {
        let coord = Coord::from_pos(pos, self.width);
        self.get_neighbors(&coord)
            .iter()
            .map(|c| c.to_pos(self.width))
            .collect()
    }
    pub fn get_neighbors(&self, pos: &Coord) -> Vec<Coord> {
        Direction::all_dir()
            .iter()
            .map(|d| d.coord_delta())
            .map(|delta| pos.plus_delta(delta))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .filter(|c| self.is_within_bounds(c))
            .collect()
    }

    pub fn is_within_bounds(&self, coord: &Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }
}

impl<E: ToString> Grid<E> {
    pub fn print(&self) {
        for i in 0..self.data.len() {
            if i % self.width == 0 {
                println!("");
            }
            print!("{}", &self.data[i].to_string());
        }
    }
}

impl<E: PartialEq> Grid<E> {
    pub fn count_ne(&self, other: &E) -> usize {
        self.data.iter().filter(|e| *e != other).count()
    }
    pub fn count_eq(&self, other: &E) -> usize {
        self.data.iter().filter(|e| *e == other).count()
    }

    pub fn get_edge_reachable_pos_set(&self, colision_elem: &E) -> HashSet<usize> {
        let mut edge_reachable_set: HashSet<usize> = HashSet::new();
        let mut visited: HashSet<usize> = HashSet::new();
        for i in 0..self.data.len() {
            let c = Coord::from_pos(i, self.width);
            let (x, y) = (c.x, c.y);
            if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
                //we are at an edge so everything is reachable from here
                if colision_elem.eq(&self.data[i]) {
                    continue;
                }
                let mut q: VecDeque<usize> = VecDeque::new();
                q.push_front(i);
                while !q.is_empty() {
                    let pos = q.pop_front().unwrap();
                    if visited.contains(&pos) {
                        continue;
                    }
                    visited.insert(pos);
                    if self.data[pos] == *colision_elem {
                        continue;
                    }
                    edge_reachable_set.insert(pos);
                    for ele in self.get_neighbors_pos(pos) {
                        q.push_front(ele);
                    }
                }
            }
        }
        edge_reachable_set
    }
}
impl<E: Clone> Grid<E> {
    pub fn init(width: usize, height: usize, init_value: E) -> Grid<E> {
        let data = vec![init_value; width * height];
        Grid {
            data,
            width,
            height,
        }
    }

    pub fn rotate(&self) -> Grid<E> {
        let mut new_data = Vec::with_capacity(self.width * self.height);

        for col in (0..self.width).rev() {
            // Iterate over columns in reverse order
            for row in 0..self.height {
                new_data.push(self.data[row * self.width + col].clone());
            }
        }

        Grid {
            data: new_data,
            width: self.height,
            height: self.width,
        }
    }
}

impl<E> Grid<E>
where
    E: FromStr + Clone,
    <E as FromStr>::Err: Debug,
{
    pub fn new(text: &[String]) -> Result<Grid<E>> {
        if text.is_empty() {
            bail!("Empty string")
        }
        let width = text.first().unwrap().len();
        let data: Vec<E> = text
            .iter()
            .filter(|l| !l.is_empty())
            .flat_map(|l| l.split(" "))
            .map(|s| s.parse().expect("Could not turn string to specified type"))
            .collect();

        Ok(Grid {
            height: data.len() / width,
            data,
            width,
        })
    }

    // returns a copy of a subgrid from [start..end)
    pub fn sub_grid(&self, start: Coord, end: Coord) -> Result<Grid<E>> {
        if start.is_after(&end) {
            bail!("Invalid coordinates, start is after end")
        }
        if end.is_after(&self.get_end_coord()) {
            bail!("Invalid coordinates, end out of bounds")
        }
        let data_width = (end.x - start.x);
        let data_height = (end.y - start.y);
        let mut data: Vec<E> = Vec::with_capacity(data_width * data_height);

        for y in start.y..end.y {
            let row_start = y * self.width;
            for i in row_start + start.x..row_start + end.x {
                data.push(self.data[i].clone());
            }
        }

        Ok(Grid {
            data,
            width: data_width,
            height: data_height,
        })
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.to_uppercase();
        Ok(match s.as_str() {
            "N" | "U" => Self::North,
            "S" | "D" => Self::South,
            "E" | "R" => Self::East,
            "W" | "L" => Self::West,
            _ => return Err("String not a valid direction".to_string()),
        })
    }
}

impl Direction {
    pub fn coord_delta(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    pub fn all_dir() -> Vec<Direction> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }
}

#[derive(Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from_pos(pos: usize, width: usize) -> Self {
        let x = pos % width;
        let y = (pos - x) / width;
        Coord::from(x, y)
    }

    pub fn to_pos(&self, width: usize) -> usize {
        (self.y * width) + self.x
    }
    pub fn is_after(&self, other: &Self) -> bool {
        self.x > other.x || self.y > other.y
    }

    pub fn plus_delta(&self, delta: (i32, i32)) -> Result<Coord> {
        if self.x == 0 && delta.0 < 0 || self.y == 0 && delta.1 < 0 {
            bail!("delta overflow")
        }

        Ok(Self {
            x: usize_add(self.x, delta.0),
            y: usize_add(self.y, delta.1),
        })
    }

    pub fn is_left_of(&self, other: &Self) -> bool {
        self.x < other.x
    }
    pub fn is_right_of(&self, other: &Self) -> bool {
        self.x > other.x
    }
    pub fn is_up_of(&self, other: &Self) -> bool {
        self.y > other.y
    }
    pub fn is_down_of(&self, other: &Self) -> bool {
        self.y < other.y
    }
}

fn usize_add(a: usize, b: i32) -> usize {
    if b.is_negative() {
        a - (b.abs() as u8) as usize
    } else {
        a + b as usize
    }
}
