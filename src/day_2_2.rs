use regex::Regex;
use std::cmp;

#[allow(unused)]
#[derive(Debug)]
pub enum Color {
    Red(usize),
    Blue(usize),
    Green(usize),
}

pub struct RGB(usize, usize, usize);

impl RGB {
    pub fn red(&self) -> usize {
        self.0
    }
    pub fn green(&self) -> usize {
        self.1
    }
    pub fn blue(&self) -> usize {
        self.2
    }
    pub fn update(&mut self, color: Color) {
        match color {
            Color::Red(a) => self.0 = cmp::max(a, self.0),
            Color::Green(a) => self.1 = cmp::max(a, self.1),
            Color::Blue(a) => self.2 = cmp::max(a, self.2),
        }
    }
}
pub fn score_lines(lines: Vec<String>) {
    let mut sum = 0;
    for line in lines {
        sum += score(line);
    }
    println!("Final score [{}]", sum);
}

pub fn score(line: String) -> usize {
    // println!("Processing [{}]", line);
    if !line.starts_with("Game") {
        return 0;
    }
    let mut limits = RGB(0, 0, 0);
    let game_num_re = Regex::new(r"Game (?P<game>\d+):").unwrap();
    let captures = game_num_re.captures(&line).unwrap();
    let _game_num = captures["game"].parse::<usize>().unwrap();
    let second_part = line.split(":").nth(1).unwrap();
    let part_re = Regex::new(r"(?P<num>\d+) (?P<color>[a-z]+)").unwrap();
    for part in second_part.split(";") {
        for smaller_part in part.split(",") {
            // println!("Part [{}]", smaller_part);
            let capture = part_re.captures(smaller_part).unwrap();
            let num = capture["num"].parse::<usize>().unwrap();
            let color_name = capture["color"].to_string();
            let color = to_color(&color_name, num);
            // println!("Part [{:?}]", color);
            limits.update(color);
        }
    }
    limits.0 * limits.1 * limits.2
}

pub fn to_color(name: &str, num: usize) -> Color {
    match name {
        "red" => Color::Red(num),
        "green" => Color::Green(num),
        "blue" => Color::Blue(num),
        _ => panic!("No color with name {}", name),
    }
}
