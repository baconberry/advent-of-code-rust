use crate::re_utils;
use anyhow::Result;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Bet {
    hand: Hand,
    bid: usize,
}

trait Value {
    fn get_value(&self) -> usize;
    fn get_text(&self) -> String;
}

impl Value for Hand {
    fn get_value(&self) -> usize {
        match self {
            Hand::HighCard(_) => 1,
            Hand::OnePair(_) => 2,
            Hand::TwoPair(_) => 3,
            Hand::ThreeOfAKind(_) => 4,
            Hand::FullHouse(_) => 5,
            Hand::FourOfAKind(_) => 6,
            Hand::FiveOfAKind(_) => 7,
        }
    }

    fn get_text(&self) -> String {
        let text = match self {
            Hand::HighCard(a) => a,
            Hand::OnePair(a) => a,
            Hand::TwoPair(a) => a,
            Hand::ThreeOfAKind(a) => a,
            Hand::FullHouse(a) => a,
            Hand::FourOfAKind(a) => a,
            Hand::FiveOfAKind(a) => a,
        };
        text.to_string()
    }
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let av = self.get_value();
        let bv = other.get_value();
        if av == bv {
            return compare_hand_text(&self.get_text(), &other.get_text());
        }
        if av > bv {
            return Ordering::Greater;
        }
        Ordering::Less
    }
}

fn compare_hand_text(a: &str, b: &str) -> Ordering {
    for (ca, cb) in a.chars().zip(b.chars()) {
        let va = char_value(ca);
        let vb = char_value(cb);
        if va > vb {
            return Ordering::Greater;
        }
        if va < vb {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn char_value(c: char) -> usize {
    if let Some(d) = c.to_digit(10) {
        return d as usize;
    }
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Wrong char"),
    }
}

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut bets: Vec<Bet> = vec![];
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        bets.push(line_to_bet(&line)?);
    }
    //println!("Bets before [{:?}]", bets);
    bets.sort_by(|a, b| a.cmp(b));
    //println!("Bets after [{:?}]", bets);
    let result = bets
        .iter()
        .enumerate()
        .map(|(i, bet)| (i + 1) * bet.bid)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
    //println!("Result [{}]", result);
    Ok(result)
}

fn line_to_bet(line: &str) -> Result<Bet> {
    //println!("Parsing line [{}]", line);
    let mut parts = line.split(" ").into_iter();
    ////println!("Parts [{:?}]", parts);
    let hand_text = parts.next().unwrap();
    let bid = parts.next().unwrap().parse::<usize>()?;
    Ok(Bet {
        hand: text_to_hand(hand_text)?,
        bid,
    })
}

fn text_to_hand(text: &str) -> Result<Hand> {
    let s = text.to_string();
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    //println!("Text map for [{}], [{:?}]", text, map);
    if map.len() == 1 {
        return Ok(Hand::FiveOfAKind(s));
    }
    let values: Vec<usize> = map.values().map(|a| *a).collect();
    //println!("Values [{:?}]", values);
    if values.contains(&4) {
        return Ok(Hand::FourOfAKind(s));
    }
    if values.contains(&3) && values.contains(&2) {
        return Ok(Hand::FullHouse(s));
    }
    if values.contains(&3) {
        return Ok(Hand::ThreeOfAKind(s));
    }
    if values.contains(&2) && map.len() == 3 {
        return Ok(Hand::TwoPair(s));
    }
    if values.contains(&2) {
        return Ok(Hand::OnePair(s));
    }
    Ok(Hand::HighCard(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn test_simple_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(6440, result.unwrap());
    }

    #[test]
    fn test_compare_hand_text() {
        assert_eq!(Ordering::Greater, compare_hand_text("3AAA", "2AAA"));
        assert_eq!(Ordering::Equal, compare_hand_text("2AAA", "2AAA"));
        assert_eq!(Ordering::Less, compare_hand_text("2AAA", "3AAA"));

        assert_eq!(Ordering::Greater, compare_hand_text("TAAA", "9AAA"));
        assert_eq!(Ordering::Equal, compare_hand_text("TAAA", "TAAA"));
        assert_eq!(Ordering::Less, compare_hand_text("TAAA", "AAAA"));
    }

    #[test]
    fn test_compare_simple_hands() {
        let a = Hand::FourOfAKind("33332".to_string());
        let b = Hand::FourOfAKind("2AAAA".to_string());
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let a = Hand::ThreeOfAKind("77888".to_string());
        let b = Hand::ThreeOfAKind("77788".to_string());
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let a = Hand::ThreeOfAKind("QQQJA".to_string());
        let b = Hand::ThreeOfAKind("T55J5".to_string());
        assert_eq!(Ordering::Greater, a.cmp(&b));
    }
}
