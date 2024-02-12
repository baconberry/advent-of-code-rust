use std::collections::HashMap;

use anyhow::{bail, Result};
use regex::Regex;

#[derive(Clone)]
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
    loop {
        match line_it.next() {
            None => break,
            Some(l) if l.len() == 0 => continue,
            Some(l) => {
                let r = parse_line(&l)?;
                map.insert(r.0.to_string(), Node::from_tuple(r));
            }
        }
    }

    let mut current = map.get("AAA").unwrap().clone();
    let mut result = 0;
    while current.value != "ZZZ" {
        let (end_node, step_count) = navigate(&step, &current, &map)?;
        result += step_count;
        current = end_node;
    }
    Ok(result)
}
fn navigate(steps: &str, start: &Node, map: &HashMap<String, Node>) -> Result<(Node, usize)> {
    let mut current = start;
    let mut step_count = 0;
    for step in steps.chars() {
        if current.value == "ZZZ" {
            break;
        }
        step_count += 1;

        current = match step {
            'L' => map.get(&current.left).unwrap(),
            'R' => map.get(&current.right).unwrap(),
            _ => bail!("Wrong step direction"),
        };
    }

    Ok((current.clone(), step_count))
}

fn parse_line(line: &str) -> Result<(String, String, String)> {
    let re = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)")?;
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
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

        Ok(())
    }
}
