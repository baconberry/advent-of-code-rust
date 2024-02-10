use regex::Regex;
use std::collections::HashSet;

pub fn process_lines(lines: Vec<String>) {
    let mut result = 0;

    for line in lines {
        result += parse_line(line);
    }

    println!("Result [{}]", result);
}

fn parse_line(line: String) -> usize {
    //println!("Parsing line [{}]", line);
    if (line.len() == 0) {
        return 0;
    }
    let mut parts = line.split(":").nth(1).unwrap().split("|");

    let win_text = parts.next().unwrap();
    let own_text = parts.next().unwrap();

    let num_re = Regex::new(r"(\d+)").unwrap();
    let mut win: HashSet<usize> = HashSet::new(); // assuming no repeating numbers
    let mut own: HashSet<usize> = HashSet::new(); // assuming no repeating numbers
    for num in num_re.find_iter(&win_text) {
        win.insert(num.as_str().parse::<usize>().unwrap());
    }
    for num in num_re.find_iter(&own_text) {
        own.insert(num.as_str().parse::<usize>().unwrap());
    }
    let intersection = own.intersection(&win);
    let win_count = intersection.count();
    if win_count == 0 {
        return 0;
    }

    2_i32.pow((win_count - 1).try_into().unwrap()) as usize
}
