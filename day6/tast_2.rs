use crate::helper::calculate_number_of_posibilities;

pub fn task_2(input: String) -> u64 {
    let race = parse_input(input.lines());
 calculate_number_of_posibilities(race)
}

pub fn parse_input(mut lines: std::str::Lines<'_>) -> (u64, u64) {
    let time_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let distance_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let time = time_line[1].trim().replace(" ", "");
    let time = time.parse::<u64>().unwrap();

    let distance = distance_line[1].trim().replace(" ", "");
    let distance = distance.parse::<u64>().unwrap();
    (time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        assert!(res == 71503);
        println!("result input 1 = {}", res);

        let res = task_2(input2);
        assert!(res == 45647654);
        println!("result input 2 = {}", res);
    }
}
