use anyhow::{bail, Result};

use crate::{DayPart, DayProblem};

pub fn process(lines: Vec<String>, day: DayPart) -> Result<usize> {
    match day {
        DayPart::One => Ok(process_d1(&lines)),
        DayPart::Two => bail!("not implemented")
    }
}

fn process_d1(lines: &[String]) -> usize {
    lines.iter()
        .map(|l| l.split(',')
            .map(|step| hash(step))
            .sum::<usize>()
        ).sum()
}

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
}
