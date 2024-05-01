
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
        sum += mutate_line(&seq, &groups);
    }
    Ok(sum)
}

fn get_components(line: &str) -> (String, Vec<usize>) {
    let mut split = line.split(" ");
    let text = split.next().unwrap();
    let groups = split.next().unwrap();
    (text.to_string(), re_utils::parse_line_numbers(groups).expect("Invalid group numbers"))
}

fn mutate_line(line: &str, groups: &Vec<usize>) -> usize {
    if !line.chars().any(|c| c=='?') {
        return is_valid(line, groups);
    }
    let wildcard_location = line.find('?').unwrap();
    let first_part = &line[0..wildcard_location];
    let second_part = &line[1+wildcard_location..];
    let operational = format!("{first_part}.{second_part}");
    //println!("Operational {:?}", operational);
    let damaged = format!("{first_part}#{second_part}");
    //return 0;
    return mutate_line(&operational, groups) + mutate_line(&damaged, groups);
}

fn is_valid(line: &str, groups: &Vec<usize>) -> usize {
    let mut contiguos_damaged: usize = 0;
    let mut result: Vec<usize> = Vec::new();
    for c in line.chars() {
        if c == '#' {
            contiguos_damaged += 1;
        } else {
            if contiguos_damaged > 0 {
                result.push(contiguos_damaged);
            }
            contiguos_damaged = 0;
        }
    }
    //one last flush
    if contiguos_damaged > 0 {
        result.push(contiguos_damaged);
    }
    //println!("Testing {:?}, result {:?}", line, result);
    return if groups == &result {
        1
    } else {
        0
    }
    
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

