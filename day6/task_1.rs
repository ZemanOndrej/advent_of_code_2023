use crate::helper::calculate_number_of_posibilities;

pub fn task_1(input: String) -> u64 {
    let races = parse_input(input.lines());
    races
        .into_iter()
        .fold(1, |acc, curr| acc * calculate_number_of_posibilities(curr))
}

pub fn parse_input(mut lines: std::str::Lines<'_>) -> Vec<(u64, u64)> {
    let time_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let distance_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let times: Vec<_> = time_line[1]
        .trim()
        .split(" ")
        .filter(|v| v.len() > 0)
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<_> = distance_line[1]
        .trim()
        .split(" ")
        .filter(|v| v.len() > 0)
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    times.into_iter().zip(distances.into_iter()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 288);
        println!("result input 1 = {}", res);

        let res = task_1(input2);
        assert!(res == 316800);
        println!("result input 2 = {}", res);
    }
}
