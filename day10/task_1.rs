use std::ops::Index;

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

fn find_next_step(
    lines: &Vec<Vec<char>>,
    ((x, y), angle): ((usize, usize), String),
) -> ((usize, usize), String) {
    let current = lines.index(x).index(y);
    let new_angle = get_connection(current, angle);
    let next_step = match new_angle.as_str() {
        "up" => ((x - 1, y), "down".to_string()),
        "down" => ((x + 1, y), "up".to_string()),
        "left" => ((x, y - 1), "right".to_string()),
        "right" => ((x, y + 1), "left".to_string()),
        _ => panic!("wrong side"),
    };
    println!("current = {:?}, next_step = {:?}", current, next_step);
    next_step
}

fn get_connection(symbol: &char, side: String) -> String {
    match symbol {
        '|' => match side.as_str() {
            "up" => "down".to_string(),
            "down" => "up".to_string(),
            _ => panic!("wrong side"),
        },
        '-' => match side.as_str() {
            "left" => "right".to_string(),
            "right" => "left".to_string(),
            _ => panic!("wrong side"),
        },
        'J' => match side.as_str() {
            "up" => "left".to_string(),
            "left" => "up".to_string(),
            _ => panic!("wrong side"),
        },
        'L' => match side.as_str() {
            "up" => "right".to_string(),
            "right" => "up".to_string(),
            _ => panic!("wrong side"),
        },
        'F' => match side.as_str() {
            "down" => "right".to_string(),
            "right" => "down".to_string(),
            _ => panic!("wrong side"),
        },
        '7' => match side.as_str() {
            "down" => "left".to_string(),
            "left" => "down".to_string(),
            _ => panic!("wrong side"),
        },
        _ => side,
    }
}

fn find_first_step(
    lines: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (prev_x, prev_y): (usize, usize),
) -> ((usize, usize), String) {
    let up = if x > 0 {
        lines.index(x - 1).get(y)
    } else {
        None
    };
    let down = if x < lines.len() - 1 {
        lines.index(x + 1).get(y)
    } else {
        None
    };
    let left = if y > 0 {
        lines.index(x).get(y - 1)
    } else {
        None
    };
    let right = if y < lines.index(x).len() - 1 {
        lines.index(x).get(y + 1)
    } else {
        None
    };
    if (up == Some(&'|')
        || up == Some(&'7')
        || up == Some(&'F')
        || up == Some(&'S'))
        && (prev_x != x - 1 || prev_y != y)
    {
        println!("up = {:?}", up);
        return ((x - 1, y), "down".to_string());
    }
    if (down == Some(&'|')
        || down == Some(&'L')
        || down == Some(&'J')
        || down == Some(&'S'))
        && (prev_x != x + 1 || prev_y != y)
    {
        println!("down= {:?}", down);
        return ((x + 1, y), "up".to_string());
    }
    if (right == Some(&'-')
        || right == Some(&'J')
        || right == Some(&'7')
        || right == Some(&'S'))
        && (prev_x != x || prev_y != y + 1)
    {
        println!("right = {:?}", right);
        return ((x, y + 1), "left".to_string());
    }
    if (left == Some(&'-')
        || left == Some(&'L')
        || left == Some(&'F')
        || left == Some(&'S'))
        && (prev_x != x || prev_y != y - 1)
    {
        println!("left = {:?}", left);
        return ((x, y - 1), "right".to_string());
    }

    panic!("No next step found");
}

fn find_start(lines: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in lines.iter().enumerate() {
        if let Some(start) = line.iter().position(|x| *x == 'S') {
            println!("line= {}, start= {}", i, start);
            return (i, start);
        }
    }
    panic!("No start found");
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
