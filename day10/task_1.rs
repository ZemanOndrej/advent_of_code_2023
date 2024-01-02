use crate::helper::{find_start, find_first_step, find_next_step};


pub fn task_1(input: String) -> i64 {
    let lines: Vec<_> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let start_pos = find_start(&lines);
    let mut next_step = find_first_step(&lines, start_pos, start_pos.clone());
    let mut steps = 1;
    println!("next_step = {:?}", next_step);

    while next_step.0 != start_pos {
        // let current = next_step;
        next_step = find_next_step(&lines, next_step);
        // prev_step = current;
        steps += 1;
        println!("next_step = {:?}", next_step);
    }

    steps / 2
}



#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 8);
        println!("result input 1 = {}", res);

        let res = task_1(input2);
        assert!(res == 6860);
        println!("result input 2 = {}", res);
    }
}
