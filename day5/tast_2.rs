

pub fn task_2(input: String) -> u32 {
    let lines = input.lines();
	let mut result = 0;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

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
