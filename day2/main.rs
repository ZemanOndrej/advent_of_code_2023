use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("something went wrong");
    fn_1(contents.clone());
    fn_2(contents.clone());
}

fn fn_1(input: String) {
    let lines = input.lines();
    let mut result = 0;
    let max_value_map = max_value_map();
    for (i, line) in lines.enumerate() {
        println!("{}: ", line);
        let game_sets: Vec<&str> = line
            .split(":")
            .collect::<Vec<&str>>()
            .get_mut(1)
            .unwrap()
            .split(";")
            .collect();
        let mut is_possible = true;
        for game_set in game_sets {
            let game_turns: Vec<&str> = game_set.split(",").collect();
            for turn in game_turns {
                let turn = turn.trim().split(" ").collect::<Vec<&str>>();
                let num = turn.get(0).unwrap().parse::<i32>().unwrap();
                let color = turn.get(1).unwrap().to_string();
                if let Some(max_value) = max_value_map.get(&color) {
                    if num > *max_value {
                        is_possible = false;
                        break;
                    }
                }
            }
        }
        if is_possible {
            result += i + 1;
            println!("row {} is possible", i);
        }
    }

    println!("{}", result)
}

fn max_value_map() -> HashMap<String, i32> {
    [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect()
}
fn default_max_value_map() -> HashMap<String, i32> {
    [
        ("red".to_string(), 0),
        ("green".to_string(), 0),
        ("blue".to_string(), 0),
    ]
    .iter()
    .cloned()
    .collect()
}

fn fn_2(mut input: String) {
    let lines = input.lines();
    let mut result = 0;
    for (i, line) in lines.enumerate() {
        let mut max_value_map = default_max_value_map();
        println!("line: {}", line);
        let game_sets: Vec<&str> = line
            .split(":")
            .collect::<Vec<&str>>()
            .get_mut(1)
            .unwrap()
            .split(";")
            .collect();
        for game_set in game_sets {
            let game_turns: Vec<&str> = game_set.split(",").collect();
            for turn in game_turns {
                let turn = turn.trim().split(" ").collect::<Vec<&str>>();
                let num = turn.get(0).unwrap().parse::<i32>().unwrap();
                let color = turn.get(1).unwrap().to_string();
                let max_value = max_value_map.get(&color).unwrap();
                if num > *max_value {
                    max_value_map.insert(color, num);
                }
            }
        }
        println!("max_value_map:  {:?} ", max_value_map);
        let line_res = max_value_map.values().fold(1, |acc, val| acc * val);
        result += line_res;
        println!("line result:  {} ", line_res)
    }

    println!("{}", result)
}
