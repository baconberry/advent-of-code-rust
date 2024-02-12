use anyhow::{bail, Result};
use regex::Regex;

pub fn parse_line_numbers(line: &str) -> Result<Vec<usize>> {
    let re = Regex::new(r"(\d+)")?;
    let mut result: Vec<usize> = vec![];
    for num in re.find_iter(line) {
        result.push(num.as_str().parse::<usize>()?);
    }

    Ok(result)
}

pub fn parse_line_numbers_i64(line: &str) -> Result<Vec<i64>> {
    let re = Regex::new(r"(-?\d+)")?;
    let mut result: Vec<i64> = vec![];
    for num in re.find_iter(line) {
        result.push(num.as_str().parse::<i64>()?);
    }

    Ok(result)
}

pub fn parse_3(line: &str) -> Result<(usize, usize, usize)> {
    let nums = parse_line_numbers(line)?;
    if nums.len() < 3 {
        bail!(format!("Parse_3 with less than 3 numbers, [{}]", line));
    }

    Ok((nums[0], nums[1], nums[2]))
}
