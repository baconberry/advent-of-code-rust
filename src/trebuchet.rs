fn main() {
    let lines = std::fs::read_to_string("input.txt");
    if let Ok(lines) = lines {
        let mut sum = 0 as usize;
        for line in lines.split("\n").into_iter() {
            sum += parse_line(line.to_string());
        }
        println!("Result is [{}]", sum);
    }
}

fn parse_line(line: String) -> usize {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in line.chars().into_iter() {
        if c.is_numeric() {
            last = Some(c);
            if first.is_none() {
                first = Some(c);
            }
        }
    }

    if first.is_none() {
        first = last;
    }

    let mut number_text = String::new();
    if let Some(c) = first {
        number_text.push(c);
    }
    if let Some(c) = last {
        number_text.push(c);
    }

    let result = number_text.parse::<usize>();

    match result {
        Ok(num) => num,
        Err(_) => 0,
    }
}
