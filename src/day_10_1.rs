use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

type Number = i64;
type NumberPair = (Number, Number);

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Position(Number, Number); // (Column, Row)

impl Position {
    fn from_pair(pair: (usize, usize)) -> Self {
        Position(pair.0 as i64, pair.1 as i64)
    }
    fn from_pair_i64(pair: (i64, i64)) -> Self {
        Position(pair.0, pair.1)
    }

    fn is_above(&self, other: &Position) -> bool {
        self.1 == other.1 - 1
    }
    fn is_below(&self, other: &Position) -> bool {
        self.1 == other.1 + 1
    }
    fn is_left(&self, other: &Position) -> bool {
        self.0 == other.1 - 1
    }
    fn is_right(&self, other: &Position) -> bool {
        self.0 == other.1 + 1
    }
}

#[derive(Clone, Debug)]
enum PipeType {
    None,
    NS,
    NE,
    NW,
    SE,
    SW,
    WE,
    Start,
}

impl PipeType {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '|' => Self::NS,
            '-' => Self::WE,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            _ => Self::None,
        }
    }
    fn get_deltas(&self) -> Vec<NumberPair> {
        match self {
            Self::None => vec![],
            Self::NS => vec![(0, 1), (0, -1)],
            Self::NE => vec![(0, -1), (1, 0)],
            Self::NW => vec![(0, -1), (-1, 0)],
            Self::SE => vec![(0, 1), (1, 0)],
            Self::SW => vec![(0, 1), (-1, 0)],
            Self::WE => vec![(-1, 0), (1, 0)],
            Self::Start => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        }
    }
}

#[derive(Clone, Debug)]
struct Pipe {
    pos: Position,
    pipe_type: PipeType,
}

impl Pipe {
    fn new(pos: Position, pipe_type: PipeType) -> Self {
        Pipe { pos, pipe_type }
    }
    fn get_connections(&self) -> Vec<Position> {
        self.pipe_type
            .get_deltas()
            .iter()
            .map(|delta_pair| (self.pos.0 + delta_pair.0, self.pos.1 + delta_pair.1))
            .map(|pair| Position::from_pair_i64(pair))
            .collect()
    }

    fn is_connected(&self, other: &Position) -> bool {
        self.get_connections().contains(other)
    }
}

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut row = 0;
    let mut graph: HashMap<Position, Pipe> = HashMap::new();
    let mut starter_box: Option<Pipe> = None;
    for line in lines {
        for (column, c) in line.chars().enumerate() {
            let pos = Position::from_pair((column, row));
            let pipe_type = PipeType::from_char(c);
            let pipe = Pipe::new(pos.clone(), pipe_type);
            match pipe.pipe_type {
                PipeType::Start => {
                    starter_box = Some(pipe.clone());
                    graph.insert(pos, pipe);
                }
                PipeType::None => drop(pipe),
                _ => {
                    graph.insert(pos, pipe);
                }
            };
        }
        row += 1;
    }
    let max_distance = calculate_distances(starter_box.unwrap(), &graph);
    Ok(max_distance)
}

fn calculate_distances(starter: Pipe, graph: &HashMap<Position, Pipe>) -> usize {
    //println!("Starter pipe [{:?}]", starter);
    //println!("Calculating distances for graph [{:?}]", graph);
    let mut queue: VecDeque<(&Pipe, usize)> = VecDeque::new();
    queue.push_front((&starter, 0));
    let mut visited: HashSet<&Position> = HashSet::new();
    let mut max_distance: usize = 0;
    while !queue.is_empty() {
        let pair = queue.pop_back().unwrap();
        let pipe = pair.0;
        let pos = &pipe.pos;
        if visited.contains(pos) {
            continue;
        }
        let distance = pair.1 + 1;
        visited.insert(pos);
        let connections: Vec<&Pipe> = pipe
            .get_connections()
            .iter()
            .filter(|conn_pos| !visited.contains(conn_pos))
            .map(|conn_pos| graph.get(conn_pos))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        //println!("Loop connections [{:?}] [{:?}]", pipe, connections);
        for conn in connections {
            if !conn.is_connected(pos) {
                //println!("Skipping [{:?}] [{:?}]", conn, conn.get_connections());
                continue;
            }

            queue.push_front((&conn, distance));
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    max_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = " .....
.S-7.
.|.|.
.L-J.
.....";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(4, result.unwrap());
    }
}
