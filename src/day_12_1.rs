
use crate::re_utils;

use anyhow::{Result};

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let _galaxy_char = '#';
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (seq, groups) = get_components(&line);
        //println!("Processing {}, {:?}", seq, groups);
        sum += brute_force(&seq, &groups);
    }
    Ok(sum)
}

fn get_components(line: &str) -> (String, Vec<usize>) {
    let mut split = line.split(" ");
    let text = split.next().unwrap();
    let groups = split.next().unwrap();
    (text.to_string(), re_utils::parse_line_numbers(groups).expect("Invalid group numbers"))
}

fn brute_force(line: &str, groups: &Vec<usize>) -> usize {
    if let Some(wildcard_location) = line.find('?') {
    let left = &line[0..wildcard_location];
    let right = &line[1+wildcard_location..];
        let damaged = left.to_owned() + "#" + right;
        let operational = left.to_owned() + "." + right;
        return brute_force(&damaged, groups) + brute_force(&operational, groups);
    }
    let split: Vec<usize> = line.split(".")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.len())
        .collect();
    let matches = &split == groups;
    //println!("Matches {}, {}, {:?}, split {:?}", line, matches, groups, split);
    if matches {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "???.### 1,1,3";
        test_line(input, 1);
        let input = ".??..??...?##. 1,1,3";
        test_line(input, 4);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        test_line(input, 1);
        let input = "????.#...#... 4,1,1";
        test_line(input, 1);
        let input = "????.######..#####. 1,6,5";
        test_line(input, 4);
        let input = "?###???????? 3,2,1";
        test_line(input, 10);
    }

    fn test_line(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process_lines(lines);
        assert_eq!(expect, result.unwrap());
    }


}

