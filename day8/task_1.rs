use crate::helper::parse_input;
const END: &str = "ZZZ";
const START: &str = "AAA";
pub fn task_1(input: String) -> u64 {
    let (instructions, network) = parse_input(input);
    let mut current = START;
    let mut step_counter = 0;
    while current != END {
        for instruction in &instructions {
            let (left, right) = network.get(current).unwrap();
            step_counter += 1;
            match instruction.as_str() {
                "R" => current = right,
                "L" => current = left,
                _ => {}
            }
        }
    }
    step_counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 2);
        println!("result input 1 = {}", res);

        let res = task_1(input2);
        assert!(res == 23147);
        println!("result input 2 = {}", res);
    }
}
