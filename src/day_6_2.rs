use crate::re_utils;
use anyhow::Result;

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut line_it = lines.into_iter();
    let times = re_utils::parse_line_numbers(&line_it.next().unwrap().replace(" ", ""))?;
    let distances = re_utils::parse_line_numbers(&line_it.next().unwrap().replace(" ", ""))?;

    let result = times
        .iter()
        .enumerate()
        .map(|(i, time)| calculate_win_count(*time, distances[i]))
        .fold(1, |acc, e| acc * e);
    Ok(result)
}

fn calculate_win_count(time: usize, distance: usize) -> usize {
    let mut win_counter = 0;
    for i in 1..time {
        if calculate_distance(time, i) > distance {
            win_counter += 1;
        }
    }
    win_counter
}

fn calculate_distance(time: usize, pressed_time: usize) -> usize {
    pressed_time * (time - pressed_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn test_simple_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(71503, result.unwrap());
    }
}
