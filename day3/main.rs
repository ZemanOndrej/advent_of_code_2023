use std::{collections::HashSet, fs};
mod fn_2;
use fn_2::fn_2;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("something went wrong");

    println!("{}", fn_1(contents.clone()));

    println!("{}", fn_2(contents.clone()));
}
fn fn_1(input: String) -> i32 {
    let lines = input.lines();
    let mut result = 0;

    let map = max_value_map();
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
                let current_number = number_string.parse::<i32>().unwrap();
                let number_len = number_string.len();
                number_string = String::new();

                if j - number_len >= 1 {
                    let char = char_lines[i][j - number_len - 1];
                    if map.contains(char) {
                        result += current_number;
                        println!("{} added", current_number);
                        continue;
                    }
                }
                let char = char_lines[i][j];
                if map.contains(char) {
                    result += current_number;
                    println!("{} added", current_number);
                    continue;
                }
                for k in j - number_len - 1..j + 1 {
                    if i > 0 {
                        let char = char_lines[i - 1][k];
                        if map.contains(char) {
                            result += current_number;
                            println!("{} added", current_number);
                            break;
                        }
                    }
                    if i + 1 < char_lines.len() {
                        let char = char_lines[i + 1][k];
                        if map.contains(char) {
                            result += current_number;
                            println!("{} added", current_number);
                            break;
                        }
                    }
                }
            }
        }
    }
    result
}
fn max_value_map() -> HashSet<String> {
    [
        "*".to_string(),
        "&".to_string(),
        "+".to_string(),
        "-".to_string(),
        "%".to_string(),
        "!".to_string(),
        "@".to_string(),
        "#".to_string(),
        "$".to_string(),
        "^".to_string(),
        "(".to_string(),
        "_".to_string(),
        "=".to_string(),
        "/".to_string(),
    ]
    .iter()
    .cloned()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fn_1() {
        let input = String::from(
            r#"2
."#,
        );

        assert_eq!(fn_1(input), 0);

        let input = String::from(
            r#"2
*"#,
        );

        assert_eq!(fn_1(input), 2);
        let input = String::from(r#"*2"#);
        assert_eq!(fn_1(input), 2);
        let input = String::from(r#".2*"#);
        assert_eq!(fn_1(input), 2);
        let input = String::from(
            r#"...
.2.
..*"#,
        );
        assert_eq!(fn_1(input), 2);
        let input = String::from(
            r#"...
.2.
*.."#,
        );
        assert_eq!(fn_1(input), 2);
        let input = String::from(
            r#"..*
.2.
..."#,
        );
        assert_eq!(fn_1(input), 2);
        let input = String::from(
            r#"*..
.2.
..."#,
        );
        assert_eq!(fn_1(input), 2);
    }
}
