use regex::Regex;

#[allow(unused)]
#[derive(Debug, Clone)]
struct Position {
    row: i64,
    start: i64,
    end: i64,
}

#[derive(Debug, Clone)]
struct Part {
    pos: Position,
    value: i64,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    pos: Position,
}

struct Row {
    row: i64,
    symbols: Vec<Symbol>,
    parts: Vec<Part>,
}

impl Position {
    pub fn overlaps_columns(&self, other: &Position) -> bool {
        self.is_between(other.start)
            || self.is_between(other.start - 1)
            || self.is_between(other.start + 1)
    }

    fn is_between(&self, n: i64) -> bool {
        n >= self.start && n <= self.end
    }
}

impl Part {
    pub fn is_overlap_symbol(&self, symbol: &Symbol) -> bool {
        let diff = self.pos.row - symbol.pos.row;
        diff >= -1 && diff <= 1 && self.pos.overlaps_columns(&symbol.pos)
    }
}

pub fn process_lines(lines: Vec<String>) {
    let mut all_parts: Vec<Part> = vec![];
    let mut all_symbol: Vec<Symbol> = vec![];
    let mut count = 0 as i64;
    for line in lines {
        let (mut parts, mut symbols) = parse_line(line, count);
        all_parts.append(&mut parts);
        all_symbol.append(&mut symbols);
        count += 1;
    }
    let mut result = 0 as i64;

    for symbol in all_symbol {
        let parts = find_adjacent_parts(&symbol, &all_parts);
        if parts.len() == 2 {
            result += parts[0].value * parts[1].value;
        }
    }

    println!("Result is [{}]", result);
}

pub fn find_adjacent_parts(symbol: &Symbol, parts: &Vec<Part>) -> Vec<Part> {
    let mut result: Vec<Part> = vec![];

    for part in parts {
        if part.is_overlap_symbol(symbol) {
            result.push(part.clone());
        }
    }
    result
}

fn parse_line(line: String, row: i64) -> (Vec<Part>, Vec<Symbol>) {
    let part_re = Regex::new(r"(\d+)").unwrap();
    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for part in part_re.find_iter(&line) {
        parts.push(Part {
            pos: Position {
                start: part.start() as i64,
                end: part.end() as i64 - 1 as i64,
                row,
            },
            value: part.as_str().parse::<i64>().unwrap(),
        });
    }

    for (i, c) in line.chars().enumerate() {
        if is_symbol(c) {
            symbols.push(Symbol {
                pos: Position {
                    start: i as i64,
                    end: i as i64,
                    row,
                },
                symbol: c,
            });
        }
    }

    (parts, symbols)
}

fn is_symbol(c: char) -> bool {
    //!c.is_numeric() && c != '.' && c != ' '
    c == '*'
}
