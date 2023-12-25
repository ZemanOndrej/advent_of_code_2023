use std::{cmp::Ordering, collections::HashMap};

pub type Hand = (String, HashMap<String, i32>, u64);

pub fn parse_input_lines(
    lines: &Vec<&str>,
) -> Vec<(String, HashMap<String, i32>, u64)> {
    lines.iter().fold(vec![], |mut acc, line| {
        let words: Vec<_> = line.split(" ").filter(|v| v.len() > 0).collect();
        let hand: Vec<_> = words[0].split("").filter(|v| v.len() > 0).collect();
        let bid = words[1].parse::<u64>().unwrap();
        let element_counter =
            hand.iter().fold(HashMap::new(), |mut acc, curr| {
                *acc.entry(curr.to_string()).or_insert(0) += 1;
                acc
            });
        acc.push((words[0].to_string(), element_counter, bid));
        acc
    })
}
pub fn is_full_house(counted: &HashMap<String, i32>) -> bool {
    let mut has_two = false;
    let mut has_three = false;
    for (_, &count) in counted.iter() {
        if count == 2 {
            has_two = true;
        }
        if count == 3 {
            has_three = true;
        }
    }
    has_two && has_three
}

pub fn is_n_of_a_kind(counted: &HashMap<String, i32>, n: i32) -> bool {
    let mut has_n = false;
    for (_, &count) in counted.iter() {
        if count == n {
            has_n = true;
        }
    }
    has_n
}

pub fn is_n_pairs(counted: &HashMap<String, i32>, n: i32) -> bool {
    let mut n_pairs = 0;
    for (_, &count) in counted.iter() {
        if count == 2 {
            n_pairs += 1;
        }
    }
    n_pairs == n
}

pub fn is_high_card(counted: &HashMap<String, i32>) -> bool {
    for (_, &count) in counted.iter() {
        if count != 1 {
            return false;
        }
    }
    return true;
}
pub struct HandTypes {
    pub five_of_kind: Vec<Hand>,
    pub four_of_kind: Vec<Hand>,
    pub full_house: Vec<Hand>,
    pub three_of_kind: Vec<Hand>,
    pub two_pair: Vec<Hand>,
    pub one_pair: Vec<Hand>,
    pub high_card: Vec<Hand>,
}
impl HandTypes {
    pub fn new() -> HandTypes {
        HandTypes {
            five_of_kind: vec![],
            four_of_kind: vec![],
            full_house: vec![],
            three_of_kind: vec![],
            two_pair: vec![],
            one_pair: vec![],
            high_card: vec![],
        }
    }
    pub fn sort_grouped_hands(&mut self) {
        self.five_of_kind.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.four_of_kind.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.full_house.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.three_of_kind.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.two_pair.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.one_pair.sort_by(|a, b| card_compare(&a.0, &b.0));
        self.high_card.sort_by(|a, b| card_compare(&a.0, &b.0));
    }

    pub fn calculate_result(&self, line_count: u64) -> u64 {
        let mut result = 0;
        let mut current_rank = line_count;
        for (_, _, bid) in self.five_of_kind.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.four_of_kind.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.full_house.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.three_of_kind.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.two_pair.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.one_pair.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        for (_, _, bid) in self.high_card.iter() {
            result += bid * current_rank;
            current_rank -= 1;
        }
        result
    }
}
fn card_compare(a: &String, b: &String) -> Ordering {
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if get_card_value(a_char) > get_card_value(b_char) {
            return Ordering::Less;
        } else if get_card_value(a_char) < get_card_value(b_char) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn get_card_value(a: char) -> u32 {
    if let Some(val) = a.to_digit(10) {
        val
    } else {
        match a {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card value: {}", a),
        }
    }
}

pub fn group_hands(hands: &Vec<Hand>) -> HandTypes {
    hands
        .iter()
        .fold(HandTypes::new(), |mut acc, (str, hand, bid)| {
            if is_n_of_a_kind(hand, 5) {
                acc.five_of_kind.push((str.clone(), hand.clone(), *bid));
            } else if is_n_of_a_kind(hand, 4) {
                acc.four_of_kind.push((str.clone(), hand.clone(), *bid));
            } else if is_full_house(hand) {
                acc.full_house.push((str.clone(), hand.clone(), *bid));
            } else if is_n_of_a_kind(hand, 3) {
                acc.three_of_kind.push((str.clone(), hand.clone(), *bid));
            } else if is_n_pairs(hand, 2) {
                acc.two_pair.push((str.clone(), hand.clone(), *bid));
            } else if is_n_pairs(hand, 1) {
                acc.one_pair.push((str.clone(), hand.clone(), *bid));
            } else if is_high_card(hand) {
                acc.high_card.push((str.clone(), hand.clone(), *bid));
            }
            acc
        })
}
