

use std::{collections::HashMap, thread::{sleep, sleep_ms, Thread}, time::Duration};

use crate::re_utils;

use anyhow::Result;

/*
* Because rust is so fast given `--release`, this is the first problem 
* in this set that a bruteforce solution won't do, so at the moment, this is 
* a WIP and will optimize the 12_1 which will be fine for 12_2 
*/
const UNFOLD_TIMES: usize = 5;
type MemoResult = Vec<Vec<usize>>;
type Memo = HashMap<String, MemoResult>;

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let _galaxy_char = '#';
    let mut sum: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        println!("Got line {}", line);
        let line = unfold(&line);
        let (seq, groups) = get_components(&line);
        println!("Processing {}, {:?}", seq, groups);
        let mut memo: Memo = HashMap::new();
        sum += brute_force_chunk(&seq, &groups);
    }
    Ok(sum)
}

fn brute_force_chunk(line: &str, groups: &[usize]) -> usize {
    println!("BF_chunk line {}, groups {:?}", line, groups);
    if line.is_empty(){
        if groups.is_empty() {
            println!("line and group empty, returning 1");
            return 1;
        }
        return 0;
    }
    //sleep(Duration::from_secs(1));
    let mut sum: usize = 0;
    let (left_chunk, right_chunk) = if let Some(dot_location) = line.find('.') {
        ( &line[..dot_location+1], &line[dot_location+1..])
    }else{
        (line, "")
    };
    //println!("left_chunk {}, right_chunk {}", left_chunk, right_chunk);
    let left_result =  process_chunk(left_chunk, groups);
    println!("Left_result {:?}", left_result);
    for r in left_result {
        if let Some(remaining_groups) = groups.strip_prefix(r.as_slice()) {
           // println!("chunk-result {:?}, RemainingGroup {:?}", r, remaining_groups);
            sum += brute_force_chunk(right_chunk, remaining_groups);
        }
    }
    
    sum
}

// a chunk is on the form XXX.: X being != .
fn process_chunk(chunk: &str, groups: &[usize]) -> MemoResult {
    if groups.len() < 2 {
        println!("Processing chunk {}, groups {:?}", chunk, groups);
    }
    //println!("Processing chunk {}, groups {:?}", chunk, groups);
    //sleep_ms(2000);


    let mut result: Vec<Vec<usize>> = Vec::new();
    if let Some(wildcard_location) = chunk.find('?') {
        let left = &chunk[0..wildcard_location];
        let right = &chunk[1+wildcard_location..];
        assert!(chunk.len() == 1 + left.len() + right.len());
        //println!("chunk: {}, left {}, right {}", chunk, left, right);
        let damaged_chars = left.to_owned() + "#" + right;
        assert!(chunk.len() == damaged_chars.len());
        let damaged = process_chunk(&damaged_chars, groups);
        for r in &damaged {
            if let Some(_) = groups.strip_prefix(r.as_slice()){
                result.push(r.to_vec());
            }
        }
        let operational_left_chars = left;
        let operational_right_chars = right;
        //sleep(Duration::from_secs(1));
        let operational_left = process_chunk(&operational_left_chars, groups);
        for l in &operational_left {
            //println!("Groups {:?}, prefix {:?}, left {}, right {}", groups, l, operational_left_chars, operational_right_chars);
            
            if let Some(groups_right) = groups.strip_prefix(l.as_slice()){
                //println!("Is prefix");
                let operational_right = process_chunk(&operational_right_chars, groups_right);
                println!("right_chars: {}, right_result {:?}", operational_right_chars, operational_right);
                for r in &operational_right {
                    if let Some(_) = groups_right.strip_prefix(r.as_slice()){
                        let mut v = l.clone();
                        v.extend(r);
                        result.push(v);
                        println!("Pushing result left {}, right {}, result {:?}", left, right, result);
                    }
                }
            }
        }
        //println!("Damaged {}, result {:?}", damaged_chars, damaged);
        /*println!("Op_left: {}, Op_right {}, res_left {:?}, res_right {:?}, result {:?}", 
            operational_left_chars, operational_right_chars, 
            operational_left, operational_right,
            result);
        */
    } else {
        //println!("pre P_chunk result, should not have more than 1 . = {}", chunk);
        let count = chunk.chars().filter(|c| c == &'#').count();
        if count == 0 {
            result.push(vec![]);
        }else{
            result.push(vec![count]);
        }
    }
    result
}

fn unfold(line: &str) -> String {
    let (seq, groups) = get_components(&line);
    println!("Got components seq {}, groups {:?}", seq, groups);
    let mut result = String::with_capacity(line.len() * 5);
    for _i in 0..UNFOLD_TIMES {
        result.push_str(&seq);
        result.push('?');
    }
    let result =  &result[..result.len()-1];
    let mut result = result.to_string();
    result.push(' ');
    let group_text = groups.iter().map(|x| x.to_string() + ",").collect::<String>();

    for _i in 0..UNFOLD_TIMES {
        result.push_str(&group_text);
    }
    let result = result.trim_end_matches(",");
    println!("Return unfold {}", result);
    result.to_string()
}

fn get_components(line: &str) -> (String, Vec<usize>) {
    let mut split = line.split(" ");
    let text = split.next().unwrap();
    let groups = split.next().unwrap();
    (text.to_string(), re_utils::parse_line_numbers(groups).expect("Invalid group numbers"))
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
        //let input = "?###???????? 3,2,1";
        //test_line(input, 506250);
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

    #[test]
    fn test_strip_prefix() {
        let a = vec![1,2,3];
        let b = vec![1,2,3];
        let expect: Vec<usize> = vec![];
        if let Some(suffix) = a.strip_prefix(b.as_slice()){
            assert_eq!(expect, suffix);
        } else{
            assert!(false);
        }
    }

}

