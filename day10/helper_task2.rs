use std::collections::HashSet;

pub fn clean_input(
    input: &Vec<Vec<char>>,
    loop_points: &HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, val)| {
                    if loop_points.contains(&(i, j)) {
                        val.clone()
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect()
}
