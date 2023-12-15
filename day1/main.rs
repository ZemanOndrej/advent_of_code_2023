use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("something went wrong");
    // fn_1(contents.clone());
    fn_2(contents.clone());
}

fn fn_1(input: String) {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut first: Option<i32> = None;
        let mut last: i32 = 0;
        for char in line.split("") {
            if let Ok(number) = char.parse::<i32>() {
                if first.is_none() {
                    first = Some(number);
                }
                last = number;
            }
        }
        if let Some(first) = first {
            result += first * 10 + last;
        }
    }

    println!("{}", result)
}

fn fn_2(mut input: String) {
    let map = create_map();
    for (key, value) in map {
        let replacer = format!("{}{}{}", key, value, key);
        input = input.replace(key, &replacer);
    }
    fn_1(input);
}

fn create_map<'a>() -> HashMap<&'a str, &'a str> {
    [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]
    .iter()
    .cloned()
    .collect()
}
