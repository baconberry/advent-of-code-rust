use crate::utils;
use regex::Regex;
use std::collections::HashSet;

#[derive(Default)]
struct Card {
    number: usize,
    win_count: usize,
    card_count: usize,
}

pub fn process_lines(lines: Vec<String>) {
    let mut cards: Vec<Card> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            continue;
        }
        println!("Parse line [{}], [{}]", i, line);
        let win_count = parse_line(line);
        let card = Card {
            number: i,
            win_count,
            card_count: 1,
        };
        cards.push(card);
    }
    println!("Total cards [{}]", cards.len());
    let mut card_counts: Vec<usize> = vec![0; cards.len()];

    let mut result = 0;
    for i in 0..cards.len() {
        card_counts[i] += 1;
        result += card_counts[i];
        for j in i + 1..cards[i].win_count + i + 1 {
            if j < cards.len() {
                card_counts[j] += card_counts[i];
            }
        }
    }

    println!("Result [{}]", result);
}

fn parse_line(line: &str) -> usize {
    //println!("Parsing line [{}]", line);
    if (line.len() == 0) {
        return 0;
    }
    let mut parts = line.split(":").nth(1).unwrap().split("|");

    let win_text = parts.next().unwrap();
    let own_text = parts.next().unwrap();

    let num_re = Regex::new(r"(\d+)").unwrap();
    let mut win: HashSet<usize> = HashSet::new(); // assuming no repeating numbers
    let mut own: HashSet<usize> = HashSet::new(); // assuming no repeating numbers
    for num in num_re.find_iter(&win_text) {
        win.insert(num.as_str().parse::<usize>().unwrap());
    }
    for num in num_re.find_iter(&own_text) {
        own.insert(num.as_str().parse::<usize>().unwrap());
    }
    let intersection = own.intersection(&win);
    intersection.count()
}
