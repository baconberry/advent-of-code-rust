use crate::prelude::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, PartialEq, Debug)]
pub struct Coord {
    x: isize,
    y: isize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vertx {
    pub id: usize,
    pub pos: Coord,
    pub edge_to: usize,
}

impl Vertx {
    pub fn new(id: usize, x: isize, y: isize, edge_to: usize) -> Self {
        Vertx {
            id,
            pos: Coord { x, y },
            edge_to,
        }
    }
}

pub fn calculate_area(vertx: &HashMap<usize, Vertx>, start_key: usize) -> Result<f64> {
    let mut pos = start_key;
    let mut interior = 0.0;
    loop {
        let v1 = vertx.get(&pos).unwrap();
        let v2 = vertx.get(&v1.edge_to).unwrap();
        interior += calculate_one_lace(v1, v2);
        pos = v1.edge_to;
        if pos == start_key {
            break;
        }
    }
    Ok(interior.abs() / 2.0)
}

fn calculate_one_lace(v1: &Vertx, v2: &Vertx) -> f64 {
    (v1.pos.x * v2.pos.y) as f64 - (v2.pos.x * v1.pos.y) as f64
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn simple_area_test() {
        let mut vec: Vec<Vertx> = Vec::new();
        vec.push(Vertx::new(1, 1, 6, 2));
        vec.push(Vertx::new(2, 3, 1, 3));
        vec.push(Vertx::new(3, 7, 2, 4));
        vec.push(Vertx::new(4, 4, 4, 5));
        vec.push(Vertx::new(5, 8, 5, 1));

        let map = vec
            .iter()
            .map(|v| (v.id, v.clone()))
            .collect::<HashMap<usize, Vertx>>();

        assert_eq!(16.5, calculate_area(&map, 1).unwrap());
    }

    #[test]
    fn area_test() {
        let points = vec![
            (0, 0),
            (0, 2),
            (2, 2),
            (2, 5),
            (0, 5),
            (0, 7),
            (1, 7),
            (1, 9),
            (6, 9),
            (6, 7),
            (4, 7),
            (4, 5),
            (4, 5),
            (6, 5),
            (6, 0),
        ];

        let mut calculated_pairs: Vec<Vertx> = Vec::new();

        for (i, elem) in points.iter().enumerate() {
            let next = if i == points.len() - 1 { 0 } else { i + 1 };
            calculated_pairs.push(Vertx::new(i, elem.0, elem.1, next));
        }

        let map = calculated_pairs
            .iter()
            .map(|v| (v.id, v.clone()))
            .collect::<HashMap<usize, Vertx>>();

        assert_eq!(62.0, calculate_area(&map, 0).unwrap());
    }
}
