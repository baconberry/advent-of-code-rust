

pub fn parse_lines(lines: Vec<String>) {
    let mut sum = 0 as usize;
    for line in lines.into_iter() {
        sum += parse_line(line.to_string());
    }
    println!("Result is [{}]", sum);
}

pub fn parse_line(line: String) -> usize {
    let word_map = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_numeric() {
            last = Some(c);
        } else {
            let word = &line[i..];
            for pair in word_map.iter() {
                if word.starts_with(pair.0) {
                    last = Some(pair.1);
                    break;
                }
            }
        }
        if first.is_none() {
            first = last;
        }
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
