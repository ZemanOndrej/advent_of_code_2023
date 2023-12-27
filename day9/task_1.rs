use crate::helper::parse_input;

pub fn task_1(input: String) -> i64 {
    let parsed_input = parse_input(input);
	let mut res = 0;
    for history in parsed_input {
        let mut history_changes = Vec::new();
        history_changes.push(history.clone());

        let mut current_list = history.clone();
        while current_list.iter().any(|v| *v != 0) {
            let mut iter = current_list.iter();
            let mut current = iter.next().unwrap();
            let change: Vec<_> = iter
                .map(|v| {
                    let res = v - current;
                    current = v;
                    res
                })
                .collect();
            history_changes.push(change.clone());
            current_list = change.clone();
        }

        println!("history_changes = {:?}", history_changes);

        let mut predicted_history =
            *history_changes[history_changes.len() - 2].last().unwrap();
        println!("predicted_history = {:?}", predicted_history);
        for i in (0..history_changes.len() - 2).rev() {
            let history_change = history_changes[i].last().unwrap();
            predicted_history += *history_change;
            println!("predicted_history = {:?}", predicted_history);
        }
		res +=predicted_history;
        // println!("predicted_history = {:?}", predicted_history);
    }
    res.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 114);
        println!("result input 1 = {}", res);

        let res = task_1(input2);
        // assert!(res == 23147);
        println!("result input 2 = {}", res);
    }
}
