
use crate::re_utils;

use anyhow::{Result};

/*
* Because rust is so fast given `--release`, this is the first problem 
* in this set that a bruteforce solution won't do, so at the moment, this is 
* a WIP and will optimize the 12_1 which will be fine for 12_2 
*/
const UNFOLD_TIMES: usize = 5;

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let _galaxy_char = '#';
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let line = unfold(&line);
        let (seq, groups) = get_components(&line);
        //println!("Processing {}, {:?}", seq, groups);
        sum += mutate_line(&seq, &groups);
    }
    Ok(sum)
}

fn unfold(line: &str) -> String {
    let (seq, groups) = get_components(&line);
    let mut result = String::with_capacity(line.len() * 5);
    for _i in 0..UNFOLD_TIMES {
        result.push_str(&seq);
        result.push('?');
    }
    let result =  result.trim_end_matches("?");
    let mut result = result.to_string();
    result.push(' ');
    let group_text = groups.iter().map(|x| x.to_string() + ",").collect::<String>();

    for _i in 0..UNFOLD_TIMES {
        result.push_str(&group_text);
    }
    let result = result.trim_end_matches(",");
    result.to_string()
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
    let operational_sum = if is_possibly_valid(&operational, groups) {
        mutate_line(&operational, groups)
    }else {
        0
    };
    let damaged_sum = if is_possibly_valid(&damaged, groups) {
        mutate_line(&damaged, groups)
    }else {
        0
    };

    //return 0;
    operational_sum + damaged_sum
}

// this is to eliminate most branches early
fn is_possibly_valid(line: &str, groups: &Vec<usize>) -> bool {
    let mut contiguos_damaged: usize = 0;
    let mut result: Vec<usize> = Vec::new();
    let mut group_pointer: usize = 0;
    for c in line.chars() {
        if c == '#' {
            contiguos_damaged += 1;
        } else if c == '?' {
            return true;
        } else {
            if contiguos_damaged > 0 {
                result.push(contiguos_damaged);
                //println!("line {} {} {} {:?}", line, contiguos_damaged, group_pointer, groups);
                if group_pointer >= groups.len() {
                    return true;
                }
                if contiguos_damaged > groups[group_pointer] {
                    return false;
                }
                group_pointer += 1;
            }
            contiguos_damaged = 0;
        }
    }
    //one last flush
    if contiguos_damaged > 0 {
        result.push(contiguos_damaged);
        group_pointer += 1;
    }
    if group_pointer >= groups.len() {
        return true;
    }
    if contiguos_damaged > groups[group_pointer] {
        return false;
    }
    true
    
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

    #[ignore]
    #[test]
    fn test_simple_input() {
        let input = "???.### 1,1,3";
        test_line(input, 1);
        let input = ".??..??...?##. 1,1,3";
        test_line(input, 16384);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        test_line(input, 1);
        let input = "????.#...#... 4,1,1";
        test_line(input, 16);
        let input = "????.######..#####. 1,6,5";
        test_line(input, 2500);
        let input = "?###???????? 3,2,1";
        test_line(input, 506250);
    }

    fn test_line(line: &str, expect: usize) {
        let lines = utils::string_to_lines(line.to_string());
        let result = process_lines(lines);
        assert_eq!(expect, result.unwrap());
    }

    #[test]
    fn test_unfold() {
        assert_eq!("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", unfold("???.### 1,1,3"));
    }

}

