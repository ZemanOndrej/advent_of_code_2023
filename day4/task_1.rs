use std::collections::HashSet;

use crate::helper::split;

pub fn task_1(input: String) -> u32 {
    let lines = input.lines();
    let mut result = 0;

    for (i, line) in lines.enumerate() {
        let numbers = split(line, ":");
        let number_groups = split(numbers[1], "|");
        let winning_numbers = split(number_groups[0], " ")
            .into_iter()
            .collect::<HashSet<_>>();

        let used_numbers = split(number_groups[1], " ");
		println!("line: {}",i);
        // println!("winning_numbers: {:?}", winning_numbers);
        // println!("used_numbers: {:?}", used_numbers);
        let matched_cards = used_numbers
            .iter()
            .filter(|num| winning_numbers.contains(*num));
        let matched_count = matched_cards.count() as u32;
		if matched_count > 0 {
			result += 2u32.pow(matched_count - 1);
			println!(
				"matched_count: {}, line score:{}",
				matched_count, 2u32.pow(matched_count - 1)
			);
		}
    }
    result
}