use std::{collections::HashMap, cmp::Ordering};

use crate::helper::{Hand, HandTypes};

fn is_full_house(counted: &HashMap<String, i32>) -> bool {
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

fn is_n_of_a_kind(counted: &HashMap<String, i32>, n: i32) -> bool {
    let mut has_n = false;
    for (_, &count) in counted.iter() {
        if count == n {
            has_n = true;
        }
    }
    has_n
}

fn is_n_pairs(counted: &HashMap<String, i32>, n: i32) -> bool {
    let mut n_pairs = 0;
    for (_, &count) in counted.iter() {
        if count == 2 {
            n_pairs += 1;
        }
    }
    n_pairs == n
}

fn is_high_card(counted: &HashMap<String, i32>) -> bool {
    for (_, &count) in counted.iter() {
        if count != 1 {
            return false;
        }
    }
    return true;
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

pub fn card_compare(a: &String, b: &String) -> Ordering {
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
