use std::{cmp::Ordering, collections::HashMap};

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

    (has_two && has_three)
        || (counted.values().filter(|&&v| v == 2).count() == 2
            && counted.get("J").unwrap_or(&0) == &1)
}

fn is_n_of_a_kind(counted: &HashMap<String, i32>, n: i32) -> bool {
    let joker_count = counted.get("J").unwrap_or(&0);
    let max_count = *counted
        .iter()
        .filter_map(|(key, val)| if key != "J" { Some(val) } else { None })
        .max()
        .unwrap_or(&0);
    max_count == n || max_count + joker_count == n
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
pub fn group_hands_with_joker(hands: &Vec<Hand>) -> HandTypes {
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
            } else if is_n_of_a_kind(hand, 2) {
                acc.one_pair.push((str.clone(), hand.clone(), *bid));
            } else if is_high_card(hand) {
                acc.high_card.push((str.clone(), hand.clone(), *bid));
            }
            acc
        })
}
pub fn card_compare_with_joker(a: &String, b: &String) -> Ordering {
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
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card value: {}", a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_hands_with_joker() {
        let hands: Vec<Hand> = vec![(
            "JJJJJ".to_string(),
            vec![("J".to_string(), 5)].into_iter().collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.five_of_kind.len(), 1);
        let hands: Vec<Hand> = vec![(
            "KKKJJ".to_string(),
            vec![("J".to_string(), 2), ("K".to_string(), 3)]
                .into_iter()
                .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.five_of_kind.len(), 1);
        let hands: Vec<Hand> = vec![(
            "KK2JJ".to_string(),
            vec![
                ("J".to_string(), 2),
                ("K".to_string(), 2),
                ("2".to_string(), 1),
            ]
            .into_iter()
            .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.four_of_kind.len(), 1);
        let hands: Vec<Hand> = vec![(
            "KK2KJ".to_string(),
            vec![
                ("J".to_string(), 1),
                ("K".to_string(), 3),
                ("2".to_string(), 1),
            ]
            .into_iter()
            .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.four_of_kind.len(), 1);
        let hands: Vec<Hand> = vec![(
            "KK22J".to_string(),
            vec![
                ("J".to_string(), 1),
                ("K".to_string(), 2),
                ("2".to_string(), 2),
            ]
            .into_iter()
            .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.full_house.len(), 1);
        let hands: Vec<Hand> = vec![(
            "1234J".to_string(),
            vec![
                ("1".to_string(), 1),
                ("2".to_string(), 1),
                ("3".to_string(), 1),
                ("4".to_string(), 1),
                ("J".to_string(), 1),
            ]
            .into_iter()
            .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.one_pair.len(), 1);
    }

    #[test]
    fn test_three_of_kind_with_joker() {
        let hands: Vec<Hand> = vec![(
            "1J34J".to_string(),
            vec![
                ("1".to_string(), 1),
                ("J".to_string(), 2),
                ("3".to_string(), 1),
                ("4".to_string(), 1),
            ]
            .into_iter()
            .collect(),
            1,
        )];
        let grouped_hands = group_hands_with_joker(&hands);
        assert_eq!(grouped_hands.three_of_kind.len(), 1);
    }

    #[test]
    fn test_is_full_house_with_joker() {
        let counted: HashMap<String, i32> = [
            ("2".to_string(), 2),
            ("3".to_string(), 2),
            ("J".to_string(), 2),
        ]
        .iter()
        .cloned()
        .collect();
        assert!(is_n_of_a_kind(&counted, 4));
    }

    #[test]
    fn test_is_full_house() {
        let counted: HashMap<String, i32> =
            [("2".to_string(), 2), ("3".to_string(), 3)]
                .iter()
                .cloned()
                .collect();
        assert!(is_full_house(&counted));
        let counted: HashMap<String, i32> = [
            ("2".to_string(), 1),
            ("3".to_string(), 3),
            ("4".to_string(), 1),
        ]
        .iter()
        .cloned()
        .collect();
        assert!(!is_full_house(&counted));
    }
}
