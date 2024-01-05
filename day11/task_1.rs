use crate::helper::{get_planet_locations, parse_input, print_space};

pub fn task_1(input: String) -> u64 {
    let mut parsed_input = parse_input(input);

    let empty_rows_cols = get_empty_rows_cols(&parsed_input);
    println!("{:?}", empty_rows_cols);

    add_empty_columns(&mut parsed_input, &empty_rows_cols);
    add_empty_rows(&mut parsed_input, &empty_rows_cols);

    print_space(&parsed_input);

    let planets = get_planet_locations(parsed_input);
    let distances = compute_distances_between_all(planets);
    distances.iter().sum()
}

fn compute_distances_between_all(planets: Vec<(usize, usize)>) -> Vec<u64> {
    let mut distances = vec![];
    for i in 0..planets.len() {
        let p1 = planets.get(i).unwrap();
        for j in i + 1..planets.len() {
            let p2 = planets.get(j).unwrap();
            let dist = calculate_dist(p1, p2);
            distances.push(dist);
        }
    }
    distances
}

fn add_empty_rows(
    parsed_input: &mut Vec<Vec<char>>,
    empty_rows_cols: &(Vec<usize>, Vec<usize>),
) {
    let col_count = parsed_input.get(0).unwrap().len();
    for row in empty_rows_cols.0.iter() {
        parsed_input.insert(row + 1, vec!['.'; col_count]);
    }
}

fn add_empty_columns(
    parsed_input: &mut Vec<Vec<char>>,
    empty_rows_cols: &(Vec<usize>, Vec<usize>),
) {
    for space in parsed_input {
        for (i, col) in empty_rows_cols.1.iter().enumerate() {
            space.insert(*col + i, '.');
        }
    }
}

fn calculate_dist((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> u64 {
    let x_diff = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let y_diff = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    x_diff as u64 + y_diff as u64
}

pub fn get_empty_rows_cols(space: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_cols: Vec<_> = vec![false; space.len()];

    for (i, line) in space.iter().enumerate() {
        if !line.contains(&'#') {
            empty_rows.push(i);
        }

        for (j, _) in line.iter().enumerate() {
            if space[i][j] == '#' {
                empty_cols[j] = true;
            }
        }
    }

    (
        empty_rows,
        empty_cols
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if !*v { Some(i) } else { None })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        println!("result input 1 = {}", res);
        assert!(res == 374);

        let res = task_1(input2);
        // assert!(res == 23147);
        println!("result input 2 = {}", res);
    }
}
