use crate::re_utils;
use anyhow::Result;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
struct HandValue {
    hand_type: Hand,
    value: String,
}

impl HandValue {
    fn new(text: &str, hand: Hand) -> HandValue {
        HandValue {
            value: text.to_string(),
            hand_type: hand,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Bet {
    bid: usize,
    best_hand: HandValue,
    original_hand: HandValue,
}

trait Value {
    fn get_value(&self) -> usize;
}

impl Value for Hand {
    fn get_value(&self) -> usize {
        match self {
            Hand::HighCard => 1,
            Hand::OnePair => 2,
            Hand::TwoPair => 3,
            Hand::ThreeOfAKind => 4,
            Hand::FullHouse => 5,
            Hand::FourOfAKind => 6,
            Hand::FiveOfAKind => 7,
        }
    }
}

impl Ord for HandValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_cmp.is_eq() {
            return compare_hand_text(&self.value, &other.value);
        }
        hand_cmp
    }
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        let best_cmp = self.best_hand.hand_type.cmp(&other.best_hand.hand_type);
        if best_cmp.is_eq() {
            return compare_hand_text(&self.original_hand.value, &other.original_hand.value);
        }
        best_cmp
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let av = self.get_value();
        let bv = other.get_value();
        av.cmp(&bv)
    }
}

fn compare_hand_text(a: &str, b: &str) -> Ordering {
    //println!("Comparing [{}] [{}]", a, b);
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
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Wrong char [{:?}]", c),
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
    ////println!("Bets before [{:?}]", bets);
    bets.sort_by(|a, b| a.cmp(b));
    ////println!("Bets after [{:?}]", bets);
    let result = bets
        .iter()
        .enumerate()
        .map(|(i, bet)| (i + 1) * bet.bid)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
    ////println!("Result [{}]", result);
    Ok(result)
}

fn line_to_bet(line: &str) -> Result<Bet> {
    ////println!("Parsing line [{}]", line);
    let mut parts = line.split(" ").into_iter();
    //////println!("Parts [{:?}]", parts);
    let hand_text = parts.next().unwrap();
    let bid = parts.next().unwrap().parse::<usize>()?;
    let hand_value = text_to_hand(hand_text)?;
    Ok(Bet {
        best_hand: mutate_js(&hand_text, None)?,
        original_hand: hand_value,
        bid,
    })
}

fn text_to_hand(text: &str) -> Result<HandValue> {
    let s = text.to_string();
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    ////println!("Text map for [{}], [{:?}]", text, map);
    if map.len() == 1 {
        return Ok(HandValue::new(text, Hand::FiveOfAKind));
    }
    let values: Vec<usize> = map.values().map(|a| *a).collect();
    ////println!("Values [{:?}]", values);
    if values.contains(&4) {
        return Ok(HandValue::new(text, Hand::FourOfAKind));
    }
    if values.contains(&3) && values.contains(&2) {
        return Ok(HandValue::new(text, Hand::FullHouse));
    }
    if values.contains(&3) {
        return Ok(HandValue::new(text, Hand::ThreeOfAKind));
    }
    if values.contains(&2) && map.len() == 3 {
        return Ok(HandValue::new(text, Hand::TwoPair));
    }
    if values.contains(&2) {
        return Ok(HandValue::new(text, Hand::OnePair));
    }
    Ok(HandValue::new(text, Hand::HighCard))
}

/*
* Exchange J for the strongest hand
*/
fn mutate_js(s: &str, options_opt: Option<String>) -> Result<HandValue> {
    //println!("Mutating [{}]", s);
    let mut original_text = s.to_string();
    let original = text_to_hand(s)?;
    if !s.contains("J") {
        return Ok(original);
    }
    let mut j_options = match options_opt {
        Some(v) => v,
        None => get_unique_chars(&original_text.replace("J", "")),
    };
    if j_options.len() == 0 {
        j_options = "123456789TQKA".to_string();
    }
    //println!("j_options [{}]", j_options);
    let mut max = original;
    let j_idx = original_text.find("J").unwrap();
    for c in j_options.chars() {
        original_text.replace_range(j_idx..j_idx + 1, &c.to_string());
        let mutated_hand = mutate_js(&original_text, Some(j_options.clone()))?;
        if let Ordering::Greater = mutated_hand.cmp(&max) {
            max = mutated_hand;
        }
    }

    Ok(max)
}

fn get_unique_chars(word: &str) -> String {
    let mut result = String::new();

    for c in word.chars() {
        if !result.contains(c) {
            result.push(c);
        }
    }

    result
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
        assert_eq!(5905, result.unwrap());
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
        let a = HandValue::new("33332", Hand::FourOfAKind);
        let b = HandValue::new("2AAAA", Hand::FourOfAKind);
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let a = HandValue::new("77888", Hand::ThreeOfAKind);
        let b = HandValue::new("77788", Hand::ThreeOfAKind);
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let a = HandValue::new("QQQJA", Hand::ThreeOfAKind);
        let b = HandValue::new("T55J5", Hand::ThreeOfAKind);
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let a = HandValue::new("JKKK2", Hand::ThreeOfAKind);
        let b = HandValue::new("QQQQ2", Hand::ThreeOfAKind);
        assert_eq!(Ordering::Less, a.cmp(&b));
    }

    #[test]
    fn test_mutate_js() {
        let text = "JJJJJ";
        let result = mutate_js(text, None).unwrap();
        //println!("Better hand for [{}], [{:?}]", text, result);
        let expect = HandValue::new("AAAAA", Hand::FiveOfAKind);
        assert_eq!(expect, result);

        let text = "KTJJT";
        let result = mutate_js(text, None).unwrap();
        let expect = HandValue::new("KTTTT", Hand::FourOfAKind);
        //println!("Better hand for [{}], [{:?}]", text, result);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_unique_chars() {
        let abc = "AAABBBCCCD";
        let result = get_unique_chars(abc);
        let expect = "ABCD";
        assert_eq!(expect, result);
    }
}
