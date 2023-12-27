use crate::helper::parse_input;

pub fn task_2(input: String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();

        let res = task_2(input);
        assert!(res == 6);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 22289513667691);
        println!("result input 2 = {}", res);
    }
}
