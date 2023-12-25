use std::collections::HashMap;

use crate::{
    task_1_helpers::card_compare, task_2_helpers::card_compare_with_joker,
};

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
    pub fn sort_grouped_hands(&mut self, with_joker: bool) {
        let compare =
            |a: &(String, HashMap<String, i32>, u64),
             b: &(String, HashMap<String, i32>, u64)| {
                if with_joker {
                    card_compare_with_joker(&a.0, &b.0)
                } else {
                    card_compare(&a.0, &b.0)
                }
            };
        self.five_of_kind.sort_by(compare);
        self.four_of_kind.sort_by(compare);
        self.full_house.sort_by(compare);
        self.three_of_kind.sort_by(compare);
        self.two_pair.sort_by(compare);
        self.one_pair.sort_by(compare);
        self.high_card.sort_by(compare);
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
