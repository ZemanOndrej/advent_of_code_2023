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
pub fn get_mapped_ranges(
    ranges: Vec<Range<u64>>,
    mapping_ranges: &Vec<(Range<u64>, Range<u64>)>,
) -> Vec<Range<u64>> {
    ranges.iter().fold(vec![], |mut mapped_ranges, range| {
        let mut not_mapped_ranges = mapping_ranges.iter().fold(
            vec![range.clone()],
            |not_mapped_ranges, (source, dest)| {
                not_mapped_ranges.iter().fold(
                    vec![],
                    |mut new_not_mapped_ranges, not_mapped_range| {
                        if source.start <= not_mapped_range.start
                            && source.end >= not_mapped_range.end
                        {
                            let dest_start = dest.start
                                + not_mapped_range.start
                                - source.start;
                            let dest_end = dest_start + not_mapped_range.end
                                - not_mapped_range.start;
                            mapped_ranges.push(dest_start..dest_end);
                        } else if source.start > not_mapped_range.start
                            && source.end < not_mapped_range.end
                        {
                            let low_range =
                                not_mapped_range.start..source.start;
                            let high_range = source.end..not_mapped_range.end;
                            mapped_ranges.push(dest.start..dest.end);
                            new_not_mapped_ranges.push(low_range);
                            new_not_mapped_ranges.push(high_range);
                        } else if source.start >= not_mapped_range.start
                            && source.start <= not_mapped_range.end
                            && source.end >= not_mapped_range.end
                        {
                            let dest_end = dest.start + not_mapped_range.end
                                - source.start;
                            mapped_ranges.push(dest.start..dest_end);
                            new_not_mapped_ranges
                                .push(not_mapped_range.start..source.start);
                        } else if source.end >= not_mapped_range.start
                            && source.end <= not_mapped_range.end
                            && source.start <= not_mapped_range.start
                        {
                            let dest_start = dest.end
                                - (source.end - not_mapped_range.start);
                            mapped_ranges.push(dest_start..dest.end);
                            new_not_mapped_ranges
                                .push(source.end..not_mapped_range.end);
                        } else {
                            new_not_mapped_ranges
                                .push(not_mapped_range.clone());
                        }
                        new_not_mapped_ranges
                    },
                )
            },
        );
        mapped_ranges.append(&mut not_mapped_ranges);
        mapped_ranges
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ranges() {
        let source = 0..10;
        let dest = 10..20;
        let mapping_ranges = vec![(source.clone(), dest)];
        let result = get_mapped_ranges(vec![source.clone()], &mapping_ranges);
        assert_eq!(result, vec![10..20]);
    }
    #[test]
    fn test_get_ranges_left() {
        let source = 5..10;
        let dest = 15..20;
        let mapping_ranges = vec![(source, dest)];
        let result = get_mapped_ranges(vec![0..10], &mapping_ranges);
        assert_eq!(result, vec![15..20, 0..5]);
    }

    #[test]
    fn test_get_ranges_right() {
        let source = 5..10;
        let dest = 15..20;
        let mapping_ranges = vec![(source, dest)];
        let result = get_mapped_ranges(vec![7..13], &mapping_ranges);
        assert_eq!(result, vec![17..20, 10..13]);
    }

    #[test]
    fn test_get_ranges_both() {
        let source = 5..10;
        let dest = 15..20;
        let mapping_ranges = vec![(source, dest)];
        let result = get_mapped_ranges(vec![0..15], &mapping_ranges);
        assert_eq!(result, vec![15..20, 0..5, 10..15]);
    }
}
