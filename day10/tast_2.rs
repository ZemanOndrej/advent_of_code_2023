use std::collections::HashSet;

use crate::{
    helper::{
        connects_down, connects_left, connects_right, connects_up,
        find_first_step, find_next_step, find_start, get_nearby_points,
    },
    helper_task2::clean_input,
};

pub fn task_2(input: String) -> i64 {
    let lines: Vec<_> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let (loop_points, start_pos) = get_loop_points(&lines);

    let mut lines = clean_input(&lines, &loop_points);
    replace_starting_symbol(&mut lines, start_pos);
    println!("{:?}", lines);

    lines.iter().fold(0, |sum, line| {
		println!("{:?}", line);
        sum + line
            .iter()
            .fold((0, false), |(sum, is_in), c| {
                if *c == '.' && !is_in {
                    (sum, false)
                } else if *c == '.' && is_in {
                    (sum + 1, true)
                } else if *c == '|'
                    // || *c == '7'
                    // || *c == 'F'  ...F7...FJ...LJ...L7
					// there are 3 options to leave polygon on symbol -> combination of FJ and L7
					// if there is F7 or LJ is_in is still the value as before
					// so the state of is_in only depends either on J,L,| or 7,F,|
                    || *c == 'J'
                    || *c == 'L'
                {
                    (sum, !is_in)
                } else {
                    (sum, is_in)
                }
            })
            .0
    })
}

fn replace_starting_symbol(
    lines: &mut Vec<Vec<char>>,
    start_pos: (usize, usize),
) {
    let area = get_nearby_points(start_pos, &lines);
    let replaced_s = get_s_replacement(area);
    let line = lines.get_mut(start_pos.0).unwrap();
    line[start_pos.1] = replaced_s;
}

fn get_s_replacement(
    area: (Option<&char>, Option<&char>, Option<&char>, Option<&char>),
) -> char {
    let up_down = area.0.is_some_and(|v| connects_down(v));
    let down_up = area.1.is_some_and(|v| connects_up(v));
    let left_right = area.2.is_some_and(|v| connects_right(v));
    let right_left = area.3.is_some_and(|v| connects_left(v));
    if up_down {
        if down_up {
            return '|';
        } else if left_right {
            return 'J';
        } else if right_left {
            return 'L';
        }
        unreachable!();
    } else if down_up {
        if left_right {
            return '7';
        } else if right_left {
            return 'F';
        }
        unreachable!();
    } else if left_right {
        if right_left {
            return '-';
        }
        unreachable!();
    }
    unreachable!();
}

pub fn get_loop_points(
    input: &Vec<Vec<char>>,
) -> (HashSet<(usize, usize)>, (usize, usize)) {
    let mut points = HashSet::new();
    let start_pos = find_start(&input);
    points.insert(start_pos);
    let mut next_step = find_first_step(&input, start_pos, start_pos.clone());
    points.insert(next_step.0);
    // println!("next_step = {:?}", next_step);

    while next_step.0 != start_pos {
        next_step = find_next_step(&input, next_step);
        points.insert(next_step.0);
    }
    (points, start_pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{get_inputs, get_custom_input};

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
		
        let res = task_2(input);
        assert!(res == 1);
        println!("result input 1 = {}", res);
		let input_task2 = get_custom_input("./input_task2.txt");
        let res = task_2(input_task2);
        assert!(res == 4);
        println!("result input task2 = {}", res);
		let input_task2 = get_custom_input("./input_task2_2.txt");
        let res = task_2(input_task2);
        assert!(res == 10);
        println!("result input task2_2 = {}", res);

        let res = task_2(input2);
        assert!(res == 343);
        println!("result input 2 = {}", res);
    }
}
