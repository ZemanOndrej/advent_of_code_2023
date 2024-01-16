use std::char;

pub fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        let line: Vec<_> = line.chars().collect();
        acc.push(line);
        acc
    })
}
pub fn get_planet_locations(input: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, curr)| {
            for (j, char) in curr.iter().enumerate() {
                if char == &'#' {
                    acc.push((i, j));
                }
            }
            acc
        })
}
pub fn print_space(parsed_input: &Vec<Vec<char>>) {
    for row in parsed_input.iter() {
        for c in row {
            print!("{}", c);
        }
        println!()
    }
}

pub fn calculate_dist(
    (x1, y1): &(usize, usize),
    (x2, y2): &(usize, usize),
) -> u64 {
    let x_diff = x1.abs_diff(*x2);
    let y_diff = y1.abs_diff(*y2);
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
