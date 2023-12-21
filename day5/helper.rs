use std::{ops::Range, str::Lines};

pub fn read_until_empty_line(mut lines: Lines<'_>) -> (Vec<String>, Lines<'_>) {
    let mut input = vec![];
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        input.push(line.to_string());
    }
    (input, lines)
}

pub fn get_ranges(
    mut lines: std::str::Lines<'_>,
) -> (std::str::Lines<'_>, Vec<(Range<u64>, Range<u64>)>) {
    lines.next().unwrap();
    let (seed_to_soil, lines) = read_until_empty_line(lines);
    let seed_to_soil = create_ranges(seed_to_soil);
    (lines, seed_to_soil)
}

pub fn get_range(input: &str) -> (Range<u64>, Range<u64>) {
    let range: Vec<_> = input.split(" ").collect();
    let dest = range[0].parse::<u64>().unwrap();
    let source = range[1].parse::<u64>().unwrap();
    let len = range[2].parse::<u64>().unwrap();
    (source..source + len, dest..dest + len)
}

pub fn create_ranges(
    seed_to_soil: Vec<String>,
) -> Vec<(Range<u64>, Range<u64>)> {
    seed_to_soil
        .iter()
        .map(|v| get_range(v))
        .collect::<Vec<_>>()
}

pub fn get_mapped_value(
    value: u64,
    ranges: &Vec<(Range<u64>, Range<u64>)>,
) -> u64 {
    let mut result = value;
    for (source, dest) in ranges {
        if source.contains(&value) {
            result = dest.start + (value - source.start);
            break;
        }
    }
    result
}
