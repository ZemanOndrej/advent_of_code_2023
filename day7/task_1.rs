use crate::{helper::parse_input_lines, task_1_helpers::group_hands};

pub fn task_1(input: String) -> u64 {
    let lines: Vec<_> = input.lines().collect();

    let hands = parse_input_lines(&lines);
    let mut grouped_hands = group_hands(&hands);

    grouped_hands.sort_grouped_hands(false);
    grouped_hands.calculate_result(lines.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 6440);
        println!("result input 1 = {}", res);

        let res = task_1(input2);
        assert!(res == 248422077);
        println!("result input 2 = {}", res);
    }
}
