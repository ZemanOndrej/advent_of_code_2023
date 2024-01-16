use crate::helper;

const MULTIPLIER: usize = 1_000_000;
pub fn task_2(input: String) -> u64 {
    let parsed_input = helper::parse_input(input);

    let empty_rows_cols = helper::get_empty_rows_cols(&parsed_input);
    println!("{:?}", empty_rows_cols);

    let planets = helper::get_planet_locations(parsed_input);
    let distances = compute_distances_between_all(&planets, &empty_rows_cols);
    distances.iter().sum()
}

fn compute_distances_between_all(
    planets: &Vec<(usize, usize)>,
    empty_rows_cols: &(Vec<usize>, Vec<usize>),
) -> Vec<u64> {
    let mut distances = vec![];
    for i in 0..planets.len() {
        let p1 = planets.get(i).unwrap();
        for j in i + 1..planets.len() {
            let p2 = planets.get(j).unwrap();
            let dist = helper::calculate_dist(p1, p2);
            let count_empty = count_empty_between(p1, p2, empty_rows_cols);
            distances
                .push(dist - count_empty + count_empty * MULTIPLIER as u64);
        }
    }
    distances
}

fn count_empty_between(
    p1: &(usize, usize),
    p2: &(usize, usize),
    empty_rows_cols: &(Vec<usize>, Vec<usize>),
) -> u64 {
    let mut count = 0;
    for row in empty_rows_cols.0.iter() {
        if p1.0 < *row && *row < p2.0 {
            count += 1;
        } else if p2.0 < *row && *row < p1.0 {
            count += 1;
        }
    }
    for col in empty_rows_cols.1.iter() {
        if p1.1 < *col && *col < p2.1 {
            count += 1;
        } else if p2.1 < *col && *col < p1.1 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_task_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        println!("result input 1 = {}", res);
        // assert!(res == 8410);

        let res = task_2(input2);
        assert!(res == 904633799472);
        println!("result input 2 = {}", res);
    }

    #[test]
    fn test_task_2_2_planets() {
        let mult = MULTIPLIER as u64;
        let input = String::from(
            "##.
...
...",
        );
        let res = task_2(input);
        assert!(res == 1);
        //////
        let input = String::from(
            "#.#
...
...",
        );
        let res = task_2(input);
        assert!(res == mult + 2);
        //////

        let input = String::from(
            "#..
#..
...",
        );
        let res = task_2(input);
        assert!(res == 1);
        //////

        let input = String::from(
            "#..
.#.
...",
        );
        let res = task_2(input);
        assert!(res == 2);
        //////

        let input = String::from(
            "#..
..#
...",
        );
        let res = task_2(input);
        assert!(res == mult + 3);
        //////

        let input = String::from(
            "#..
...
#..",
        );
        let res = task_2(input);
        assert!(res == mult + 2);
        //////

        let input = String::from(
            "#..
...
.#.",
        );
        let res = task_2(input);
        assert!(res == mult + 3);
        //////

        let input = String::from(
            "#..
...
..#",
        );
        let res = task_2(input);
        assert!(res == 4 + (2 * mult));
    }
}
