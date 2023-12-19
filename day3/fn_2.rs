use std::collections::HashMap;

pub fn fn_2(input: String) -> u32 {
    let lines = input.lines();

    let mut map: HashMap<String, Vec<u32>> = HashMap::new();
    let char_lines: Vec<Vec<&str>> =
        lines.map(|line| line.split("").collect()).collect();
    for (i, line) in char_lines.iter().enumerate() {
        let mut number_string = String::new();
        for (j, char) in line.into_iter().enumerate() {
            let current = char.parse::<i32>();
            if current.is_ok() {
                number_string.push_str(char);
            } else if number_string.len() > 0 {
                println!(
                    "line {}, start:{}, end: {}, number: {}",
                    i,
                    j - number_string.len(),
                    j,
                    number_string
                );
                let current_number = number_string.parse::<u32>().unwrap();
                let number_len = number_string.len();
                number_string = String::new();

                if j - number_len >= 1 {
                    let char = char_lines[i][j - number_len - 1];
                    if char == "*" {
                        let coordinate =
                            format!("{}-{}", i, j - number_len - 1);
                        map.entry(coordinate)
                            .or_insert(vec![])
                            .push(current_number);
                    }
                }
                let char = char_lines[i][j];
                if char == "*" {
                    let coordinate = format!("{}-{}", i, j);
                    map.entry(coordinate)
                        .or_insert(vec![])
                        .push(current_number);
                }

                for k in j - number_len - 1..j + 1 {
                    if i > 0 {
                        let char = char_lines[i - 1][k];
                        if char == "*" {
                            let coordinate = format!("{}-{}", i - 1, k);
                            map.entry(coordinate)
                                .or_insert(vec![])
                                .push(current_number);
                        }
                    }
                    if i + 1 < char_lines.len() {
                        let char = char_lines[i + 1][k];
                        if char == "*" {
                            let coordinate = format!("{}-{}", i + 1, k);
                            map.entry(coordinate)
                                .or_insert(vec![])
                                .push(current_number);
                        }
                    }
                }
            }
        }
    }
    let mut result: u32 = 1;

    for (_, values) in map.iter() {
        if values.len() == 2 {
            result += values.iter().fold(1 as u32, |acc, val| acc * val);
        }
    }
    result - 1
}
