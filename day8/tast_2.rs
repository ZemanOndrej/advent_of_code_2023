use crate::helper::{count_steps, parse_input};

pub fn task_2(input: String) -> u64 {
    let (instructions, network) = parse_input(input);
    let current_list: Vec<_> = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|v| v.clone())
        .collect();

    current_list
        .iter()
        .map(|v| count_steps(&instructions, &network, v, &is_end))
        .fold(1, |acc, v| lcm(acc, v))
}

fn is_end(current: &str) -> bool {
    current.ends_with("Z")
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (_, input2) = get_inputs();
        let input_task2 = std::fs::read_to_string("./input_task2.txt")
            .expect("something went wrong");
        let res = task_2(input_task2);
        assert!(res == 6);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 22289513667691);
        println!("result input 2 = {}", res);
    }
}
