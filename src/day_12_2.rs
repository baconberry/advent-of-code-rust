use std::{
    collections::HashMap,
};

use crate::re_utils;

use anyhow::Result;
use rayon;
use rayon::prelude::*;

/*
* Because rust is so fast given `--release`, this is the first problem
* in this set that a bruteforce solution won't do.
* introducing DP with memoization
*/
const UNFOLD_TIMES: usize = 5;

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let _galaxy_char = '#';
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        //println!("Got line {}", line);
        let line = unfold(&line);
        let (seq, groups) = get_components(&line);
        //println!("Processing {}, {:?}", seq, groups);
        let mut memo: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        let result = mutate_line(&seq, &groups, &mut memo);
        //println!("Result for: {}, {:?}", line, result);
        sum += result;
    }
    Ok(sum)
}

fn mutate_line(
    chunk: &str,
    groups: &[usize],
    memo: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    let memo_key = (chunk.to_string(), groups.to_vec());
    if memo.contains_key(&memo_key) {
        return memo[&memo_key];
    }
    if chunk.is_empty() && groups.is_empty() {
        return 1;
    }
    if chunk.is_empty() {
        return 0;
    }
    let mut sum: usize = 0;
    if let Some(wildcard_location) = chunk.find('?') {
        let left = &chunk[0..wildcard_location]; // this should not have wildcards anymore
        let right = &chunk[1 + wildcard_location..];
        let damaged = left.to_owned() + "#" + right;
        let damaged_count = mutate_line(&damaged, groups, memo);
        sum += damaged_count;

        /*
         * if left does not shift, then there is no point to process right :)
         */
        if let Some(left_shift) = process_chunk(left, groups) {
            let right_groups = &groups[left_shift..];
            let right_sum = mutate_line(right, right_groups, memo);
            sum += right_sum;
        }
    } else {
        //no wildcards
        if let Some(shift) = process_chunk(chunk, groups) {
            let right_groups = &groups[shift..];
            let right_sum = mutate_line("", right_groups, memo);
            sum += right_sum;
        }
    }
    memo.insert(memo_key, sum);
    sum
}

//no wildcard should be here
fn process_chunk(chunk: &str, groups: &[usize]) -> Option<usize> {
    // group shift, counter
    debug_assert!(!chunk.contains("?"));
    let chunk_groups = count_chunk(chunk);
    if chunk_groups.is_empty() || groups.starts_with(&chunk_groups) {
        return Some(chunk_groups.len());
    }
    None
}

fn count_chunk(chunk: &str) -> Vec<usize> {
    if chunk.is_empty() {
        return vec![];
    }
    chunk
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| {
            debug_assert!(!s.contains("."));
            s.len()
        })
        .collect()
}

fn unfold(line: &str) -> String {
    let (seq, groups) = get_components(&line);
    //println!("Got components seq {}, groups {:?}", seq, groups);
    let mut result = String::with_capacity(line.len() * 5);
    for _i in 0..UNFOLD_TIMES {
        result.push_str(&seq);
        result.push('?');
    }
    let result = &result[..result.len() - 1];
    let mut result = result.to_string();
    result.push(' ');
    let group_text = groups
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();

    for _i in 0..UNFOLD_TIMES {
        result.push_str(&group_text);
    }
    let result = result.trim_end_matches(",");
    //println!("Return unfold {}", result);
    result.to_string()
}

fn get_components(line: &str) -> (String, Vec<usize>) {
    let mut split = line.split(" ");
    let text = split.next().unwrap();
    let groups = split.next().unwrap();
    (
        text.to_string(),
        re_utils::parse_line_numbers(groups).expect("Invalid group numbers"),
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils;

    //#[ignore]
    #[test]
    fn test_simple_input() {
        let input = "??? 3";
        test_line(input, 1);
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
        assert_eq!(
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
            unfold("???.### 1,1,3")
        );
    }

    #[test]
    fn test_strip_prefix() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        let expect: Vec<usize> = vec![];
        if let Some(suffix) = a.strip_prefix(b.as_slice()) {
            assert_eq!(expect, suffix);
        } else {
            assert!(false);
        }
    }
}
