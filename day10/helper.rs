use std::ops::Index;

pub fn find_next_step(
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
    // println!("current = {:?}, next_step = {:?}", current, next_step);
    next_step
}

pub fn get_connection(symbol: &char, side: String) -> String {
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

pub fn find_first_step(
    lines: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (prev_x, prev_y): (usize, usize),
) -> ((usize, usize), String) {
    let (up, down, left, right) = get_nearby_points((x, y), lines);
    if (up == Some(&'|')
        || up == Some(&'7')
        || up == Some(&'F')
        || up == Some(&'S'))
        && (prev_x != x - 1 || prev_y != y)
    {
        // println!("up = {:?}", up);
        return ((x - 1, y), "down".to_string());
    }
    if (down == Some(&'|')
        || down == Some(&'L')
        || down == Some(&'J')
        || down == Some(&'S'))
        && (prev_x != x + 1 || prev_y != y)
    {
        // println!("down= {:?}", down);
        return ((x + 1, y), "up".to_string());
    }
    if (right == Some(&'-')
        || right == Some(&'J')
        || right == Some(&'7')
        || right == Some(&'S'))
        && (prev_x != x || prev_y != y + 1)
    {
        // println!("right = {:?}", right);
        return ((x, y + 1), "left".to_string());
    }
    if (left == Some(&'-')
        || left == Some(&'L')
        || left == Some(&'F')
        || left == Some(&'S'))
        && (prev_x != x || prev_y != y - 1)
    {
        // println!("left = {:?}", left);
        return ((x, y - 1), "right".to_string());
    }

    panic!("No next step found");
}
pub fn connects_down(c: &char) -> bool {
    match *c {
        '|' => true,
        '7' => true,
        'F' => true,
        _ => false,
    }
}

pub fn connects_up(c: &char) -> bool {
    match *c {
        '|' => true,
        'L' => true,
        'J' => true,
        _ => false,
    }
}
pub fn connects_left(c: &char) -> bool {
    match *c {
        '-' => true,
        'J' => true,
        '7' => true,
        _ => false,
    }
}
pub fn connects_right(c: &char) -> bool {
    match *c {
        '-' => true,
        'L' => true,
        'F' => true,
        _ => false,
    }
}

pub fn get_nearby_points(
    (x, y): (usize, usize),
    lines: &Vec<Vec<char>>,
) -> (Option<&char>, Option<&char>, Option<&char>, Option<&char>) {
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
    (up, down, left, right)
}

pub fn find_start(lines: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in lines.iter().enumerate() {
        if let Some(start) = line.iter().position(|x| *x == 'S') {
            println!("line= {}, start= {}", i, start);
            return (i, start);
        }
    }
    panic!("No start found");
}
