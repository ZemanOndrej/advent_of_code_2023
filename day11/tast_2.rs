use crate::helper::parse_input;

pub fn task_2(input: String) -> i64 {
    let parsed_input = parse_input(input);
    let mut res = 0;

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
        assert!(res == 2);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 1062);
        println!("result input 2 = {}", res);
    }
}
