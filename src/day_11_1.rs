use std::collections::{hash_map, HashMap, HashSet, VecDeque};
use crate::re_utils;

use anyhow::{bail, Result};

type Number = i64;
type NumberPair = (usize, usize);
type Universe = Vec<Vec<Position>>;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Position {
    row: usize,
    column: usize
}

impl Position {
    
    fn new(x: usize, y: usize) -> Self {
        Position {
            row: y,
            column: x
        }
    }

    fn augment_row(&self, agg_row: usize) -> Self {
        Position {
            row: self.row+agg_row,
            column: self.column
        }
    }

    fn augment_column(&self, agg_col: usize) -> Self {
        Position {
            row: self.row,
            column: self.column+agg_col
        }
    }

    fn distance(&self, other: &Self) -> usize {
        diff(self.column, other.column) + diff(self.row, other.row)
    }

    fn is_same(&self, other: &Self) -> bool {
        self.column == other.column &&
        self.row == other.row
    }

    fn is_greater(&self, other: &Self) -> bool {
        self.row > other.row || ( self.row == other.row && self.column > other.column )
    }

}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }

}



pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut universe: Universe = Vec::new();
    let galaxy_char = '#';
    for (line_no, line) in lines.iter().enumerate() {
        let locations = re_utils::parse_loc(&line, &galaxy_char)?;
        universe.push(
            locations.iter()
            .map(|x| Position::new(*x, line_no))
            .collect()
        );
    }
    let universe = expand_universe(universe);
    //println!("{:?}", universe);
    let max_distance = sum_all_positions(&universe);
    Ok(max_distance)
}

fn sum_all_positions(all_positions: &Vec<Position>) -> usize {
    let mut sum: usize = 0;
    for a in all_positions {
        for b in all_positions {
            if a.is_greater(b) {
                //println!("From {:?} to {:?} = {}", a, b, a.distance(b));
                sum += a.distance(b);
            }
        }
    }
    sum
}


fn expand_universe(universe: Universe) -> Vec<Position> {
    let row_universe = expand_universe_rows(universe);
    expand_universe_columns(row_universe)
}

fn expand_universe_columns(universe: Universe) -> Vec<Position> {
    let mut universe_by_col: HashMap<usize, Vec<Position>> = HashMap::new();
    let mut max_col: usize = 0;
    for row in universe {
        for pos in row {
            if max_col < pos.column {
                max_col = pos.column;
            }
            universe_by_col.entry(pos.column)
                .or_insert(Vec::new())
                .push(pos);
        }
    }
    let mut augment: usize = 0;
    let mut all_positions: Vec<Position> = Vec::new();
    for i in 0..max_col+1 {
        if universe_by_col.contains_key(&i) {
            for pos in universe_by_col.get(&i).unwrap() {
                all_positions.push(pos.augment_column(augment));
            }
        } else {
            augment += 1;
        }
    }
    all_positions
}
fn expand_universe_rows(universe: Universe) -> Universe {
    // rows are easy, if vec is empty then double it
    // augment the row num and mutate every vec<position> here after
    let mut augment: usize = 0;
    let mut new_universe: Universe = Vec::new();
    for row in universe {
        if row.is_empty() {
            augment += 1;
            new_universe.push(Vec::new());
        } else {
            new_universe.push(
                row.iter()
                .map(|pos| pos.augment_row(augment))
                .collect()
            );
        }
    }

    new_universe
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(374, result.unwrap());
    }

    #[test]
    fn test_same_position() {
        let a = Position::new(0, 4);
        let b = Position::new(0, 4);
        assert!(a.is_same(&b));
    }

    #[test]
    fn test_distance() {
        let a = Position::new(0, 4);
        let b = Position::new(10, 9);
        test_distance_single(&a, &b, 15);

        let a = Position::new(6, 1);
        let b = Position::new(11, 5);
        test_distance_single(&a, &b, 9);

        let a = Position::new(2, 0);
        let b = Position::new(7, 12);
        test_distance_single(&a, &b, 17);

        let a = Position::new(11, 0);
        let b = Position::new(11, 5);
        test_distance_single(&a, &b, 5);
    }

    fn test_distance_single(a: &Position, b: &Position, expect: usize) {
        assert_eq!(expect, a.distance(&b));
        assert_eq!(a.distance(&b), b.distance(&a));
    }
}

