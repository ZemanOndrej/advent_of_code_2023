use crate::helper::{parse_input, count_steps};
const END: &str = "ZZZ";
const START: &str = "AAA";
pub fn task_1(input: String) -> u64 {
    let (instructions, network) = parse_input(input);
    count_steps(&instructions, &network, START, &is_end)
}
fn is_end(current: &str) -> bool {
	current == END
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
