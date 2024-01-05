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
