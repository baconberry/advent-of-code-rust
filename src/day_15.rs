use core::panic;
use std:: collections::{HashMap, LinkedList, VecDeque};

use anyhow::{bail, Result};
use regex::Regex;

use crate::{DayPart, DayProblem};

pub fn process(lines: Vec<String>, day: DayPart) -> Result<usize> {
    match day {
        DayPart::One => Ok(process_d1(&lines)),
        DayPart::Two => Ok(process_d2(&lines)),
    }
}

// day 1
fn process_d1(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|l| l.split(',').map(|step| hash(step)).sum::<usize>())
        .sum()
}
// day 1

// day 2

struct Lens {
    label: String,
    focal_length: usize,
}

enum Step {
    Dash(String),
    Equals(Lens),
}

impl Step {
    fn of(text: &str) -> Self {
        let re = Regex::new(r"(\w+)([-=])(\d+)?").unwrap();
        if let Some(groups) = re.captures(text) {
            let label = groups.get(1).unwrap().as_str();
            let sign = groups.get(2).unwrap().as_str();
            return match sign {
                "-" => Self::Dash(label.to_string()),
                "=" => Self::Equals(Lens::new(
                    label,
                    groups.get(3).unwrap().as_str().parse().unwrap(),
                )),
                _ => panic!("invalid operation"),
            };
        }
        panic!("Operation not matched [{}]", text)
    }
}

impl Lens {
    fn new(label: &str, focal_length: usize) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }

    fn label_hash(&self) -> usize {
        hash(&self.label)
    }

    fn is_same_label(&self, other_label: &str) -> bool {
        self.label.eq(other_label)
    }

    fn replace(&mut self, other_lens: &Self) {
        self.focal_length = other_lens.focal_length;
    }
}

type MAP = HashMap<usize, VecDeque<Lens>>;
fn process_d2(lines: &[String]) -> usize {
    let step_it = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(','))
        .flatten();
    let mut map: MAP = HashMap::with_capacity(256);
    for s in step_it {
        let step = Step::of(s);
        match step {
            Step::Dash(label) => remove_lens(&mut map, &label),
            Step::Equals(lens) => add_lens(&mut map, lens),
        }
    }
    calculate_map_value(&map)
}

fn calculate_map_value(map: &MAP) -> usize {
    let mut result = 0 as usize;
    for k in map.keys() {
        let box_value = k + 1;
        let list = map.get(k).unwrap();
        for (i, lens) in list.iter().enumerate() {
            result += box_value * (i + 1) * lens.focal_length;
        }
    }
    result
}

fn remove_lens(map: &mut MAP, label: &str) {
    let label_hash = hash(label);
    if let Some(list) = map.get_mut(&label_hash) {
        list.retain(|lens| !lens.is_same_label(label));
    }
}

fn add_lens(map: &mut MAP, lens: Lens) {
    let label_hash = lens.label_hash();
    if !map.contains_key(&label_hash) {
        map.insert(label_hash, VecDeque::new());
    }
    let mut list = map.get_mut(&label_hash).unwrap();
    let mut is_contained = false;
    for e in list.iter_mut() {
        if e.is_same_label(&lens.label) {
            is_contained = true;
            e.replace(&lens);
            break;
        }
    }
    if !is_contained {
        list.push_back(lens);
    }
}

// day 2

// HASHING ALGO

fn hash(chars: &str) -> usize {
    let mut current_value: usize = 0;
    for c in chars.chars() {
        let ascii_value = c as u8;
        current_value = calculate_current_value(current_value, ascii_value);
    }
    current_value
}

fn calculate_current_value(current_value: usize, ascii_value: u8) -> usize {
    let mut cv = current_value;
    cv += ascii_value as usize;
    cv *= 17;
    cv %= 256;
    cv
}

// HASHING ALGO
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_hash_base_cases() {
        test_step("rn=1", 30);
        test_step("cm-", 253);
        test_step("qp=3", 97);
        test_step("cm=2", 47);
        test_step("qp-", 14);
        test_step("pc=4", 180);
        test_step("ot=9", 9);
        test_step("ab=5", 197);
        test_step("pc-", 48);
        test_step("pc=6", 214);
        test_step("ot=7", 231);
    }

    fn test_step(step: &str, result: usize) {
        assert_eq!(result, hash(step))
    }

    #[test]
    fn test_simple_input() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(lines, DayPart::One);
        assert_eq!(1320, result.unwrap());
    }

    #[test]
    fn test_simple_input_day_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let lines = utils::string_to_lines(input.to_string());
        let result = process(lines, DayPart::Two);
        assert_eq!(145, result.unwrap());
    }
}
