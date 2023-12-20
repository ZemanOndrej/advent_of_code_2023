use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helper::split;

pub fn task_2(input: String) -> u32 {
    let lines = input.lines();
    let mut result_map: HashMap<u32, u32> = HashMap::new();

    for (i, line) in lines.enumerate() {
        let line_number = i as u32 + 1;
        let numbers = split(line, ":");
        let number_groups = split(numbers[1], "|");
        let winning_numbers = split(number_groups[0], " ")
            .into_iter()
            .collect::<HashSet<_>>();
        let used_numbers = split(number_groups[1], " ");

        println!("line: {}", line_number);
        let current = result_map.entry(line_number).or_insert(1).clone();
        let matched_cards = used_numbers
            .iter()
            .filter(|num| winning_numbers.contains(*num));
        let matched_count = matched_cards.count() as u32;
        println!(
            "matched_count: {}, number of current cards:{}",
            matched_count, current
        );
        if matched_count > 0 {
            for j in 1..matched_count + 1 {
                let entry = *result_map.entry(line_number + j).or_insert(1);
                result_map.insert(line_number + j, entry + current);
            }
        }
    }
    let mut result = 0;
    for (_, v) in &result_map {
        result += v;
    }
    println!("result_map with sum {}:", result);
    for k in result_map.keys().sorted().into_iter() {
        let v = result_map.get(k).unwrap();
        println!("line: {}, result: {}", k, v);
    }

    result
}

#[cfg(test)]
mod tests {

    use common::get_inputs;

    use super::*;
    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        assert!(res == 30);
        println!("result{}", res);

        let res = task_2(input2);
        println!("result{}", res);
    }
}
