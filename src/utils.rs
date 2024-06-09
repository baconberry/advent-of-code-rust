pub fn read_lines(file_name: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let lines = std::fs::read_to_string(file_name);
    if let Ok(lines) = lines {
        for line in lines.split("\n").into_iter() {
            result.push(line.to_string());
        }
    }
    result
}

pub fn string_to_lines(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in input.lines() {
        result.push(line.to_string());
    }
    result
}

pub fn rotate_block(block: &[String]) -> Vec<String> {
    if block.is_empty() {
        return vec![];
    }
    let size = block[0].len();
    let mut all_iters = vec![String::new(); size];

    for i in 0..size {
        for s in block {
            all_iters[i].push(s.chars().nth(i).unwrap());
        }
    }
    all_iters
}
