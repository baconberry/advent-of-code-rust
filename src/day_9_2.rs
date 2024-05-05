use crate::re_utils;
use anyhow::Result;

type Number = i64;
type Vuz = Vec<Number>;

pub fn process_lines(lines: Vec<String>) -> Result<Number> {
    let result = lines
        .iter()
        .filter(|l| l.len() > 0)
        .map(|l| process_line(l))
        .map(|r| match r {
            Ok(r) => r,
            _ => 0,
        })
        .sum::<Number>();

    Ok(result)
}

fn process_line(line: &str) -> Result<Number> {
    let nums = re_utils::parse_line_numbers_i64(line)?;
    let res = process_nums(&nums)?;
    Ok(diff_to_augment(*nums.first().unwrap(), res))
}

fn process_nums(nums: &Vuz) -> Result<Number> {
    //println!("process_nums input [{:+?}]", nums);
    let all_zeroes = nums.iter().all(|a| *a == 0);
    if all_zeroes || nums.len() == 0 {
        return Ok(0);
    }
    let mut reduced: Vuz = vec![];

    for i in 0..nums.len() - 1 {
        let (a, b) = (nums[i], nums[i + 1]);
        reduced.push(diff(a, b));
    }

    let augment_value = process_nums(&reduced)?;
    let result = diff_to_augment(*reduced.first().unwrap(), augment_value);
    //println!("process_nums result [{}]", result);

    Ok(result)
}
fn diff_to_augment(val: Number, difference: Number) -> Number {
    val - difference
}

fn diff(a: Number, b: Number) -> Number {
    //max(a,b)-min(a,b)
    b - a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_simple_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn test_process_line() {
        let line = "10 13 16 21 30 45";
        let result = process_line(line);
        //println!("Result test [{:?}]", result);
        assert_eq!(5, result.unwrap());
    }
}
