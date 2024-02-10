use regex::Regex;

pub fn parse_line_numbers(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut result: Vec<usize> = vec![];
    for num in re.find_iter(line) {
        result.push(num.as_str().parse::<usize>().unwrap());
    }

    result
}

pub fn parse_3(line: &str) -> (usize, usize, usize) {
    let nums = parse_line_numbers(line);
    if nums.len() < 3 {
        panic!("Parse_3 with less than 3 numbers, [{}]", line);
    }

    (nums[0], nums[1], nums[2])
}
