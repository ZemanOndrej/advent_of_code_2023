use crate::helper::parse_input;

pub fn task_2(input: String) -> u64 {
    let (instructions, network) = parse_input(input);
    println!("{:?}", network);
    println!("{:?}", instructions);

    let mut step_counter = 0;

    step_counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        assert!(res == 2);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 45647654);
        println!("result input 2 = {}", res);
    }
}
