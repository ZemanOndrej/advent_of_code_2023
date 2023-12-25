use crate::helper::{parse_input_lines, group_hands};

pub fn task_2(input: String) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let hands = parse_input_lines(&lines);
    let mut grouped_hands = group_hands(&hands);

    grouped_hands.sort_grouped_hands();
    grouped_hands.calculate_result(lines.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        assert!(res == 71503);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 45647654);
        println!("result input 2 = {}", res);
    }
}
