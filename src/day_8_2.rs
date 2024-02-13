use std::collections::HashMap;

use anyhow::{bail, Result};
use num::integer::lcm;
use rayon;
use rayon::prelude::*;
use regex::Regex;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn from_tuple(t: (String, String, String)) -> Self {
        Node {
            value: t.0,
            left: t.1,
            right: t.2,
        }
    }
}

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut line_it = lines.into_iter();
    let step = line_it.next().unwrap();
    let mut map: HashMap<String, Node> = HashMap::new();
    let mut starter_nodes: Vec<String> = vec![];
    loop {
        match line_it.next() {
            None => break,
            Some(l) if l.len() == 0 => continue,
            Some(l) => {
                let r = parse_line(&l)?;
                let key = r.0.to_string();
                map.insert(key.clone(), Node::from_tuple(r));
                if key.ends_with("A") {
                    starter_nodes.push(key);
                }
            }
        }
    }

    let starters: &Vec<&Node> = &starter_nodes
        .iter()
        .map(|k| map.get(k))
        .map(|o| o.unwrap())
        .collect();
    let finals: Vec<(Node, usize)> = starters
        .par_iter()
        .map(|n| navigate_one(&step, n, &map).unwrap())
        .collect();
    let result = finals
        .iter()
        .map(|p| p.1)
        .fold(1 as usize, |acc, e| lcm(acc, e));
    Ok(result)
}

fn navigate_one(steps: &str, start: &Node, map: &HashMap<String, Node>) -> Result<(Node, usize)> {
    let mut current = start;
    let mut step_count = 0;
    let mut pos = 0;
    loop {
        if current.value.ends_with("Z") {
            break;
        }
        let step = steps.chars().nth(pos % steps.len()).unwrap();
        step_count += 1;
        pos += 1;

        current = match step {
            'L' => map.get(&current.left).unwrap(),
            'R' => map.get(&current.right).unwrap(),
            _ => bail!("Wrong step direction"),
        };
    }

    Ok((current.clone(), step_count))
}

fn parse_line(line: &str) -> Result<(String, String, String)> {
    let re = Regex::new(r"^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)")?;
    for (_, [a, b, c]) in re.captures_iter(line).map(|a| a.extract()) {
        return Ok((a.to_string(), b.to_string(), c.to_string()));
    }
    bail!("Something is not right");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(6, result.unwrap());
    }

    #[test]
    fn test_parse_line() -> Result<()> {
        let text = "AAA = (BBB, CCC)";
        let result = parse_line(text)?;
        let expect = ("AAA".to_string(), "BBB".to_string(), "CCC".to_string());
        assert_eq!(expect, result);

        let text = "11A = (11B, XXX)";
        let result = parse_line(text)?;
        let expect = ("11A".to_string(), "11B".to_string(), "XXX".to_string());
        assert_eq!(expect, result);
        Ok(())
    }
}
