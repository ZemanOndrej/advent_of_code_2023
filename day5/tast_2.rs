use crate::helper::{get_mapped_ranges, get_ranges};

pub fn task_2(input: String) -> u64 {
    let mut lines = input.lines();
    let seed_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let seeds: Vec<u64> = seed_line[1]
        .trim()
        .split(" ")
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let ranges = seeds.chunks(2).map(|chunk| chunk[0]..chunk[0] + chunk[1]);

    lines.next().unwrap();

    let (lines, seed_to_soil) = get_ranges(lines);
    let (lines, soil_to_fertilizer) = get_ranges(lines);
    let (lines, fertilizer_to_water) = get_ranges(lines);
    let (lines, water_to_light) = get_ranges(lines);
    let (lines, light_to_temperature) = get_ranges(lines);
    let (lines, temperature_to_humidity) = get_ranges(lines);
    let (_, humidity_to_location) = get_ranges(lines);

    let mut mapped_ranges: Vec<_> = ranges.collect();

    mapped_ranges = get_mapped_ranges(mapped_ranges, &seed_to_soil);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &soil_to_fertilizer);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &fertilizer_to_water);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &water_to_light);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &light_to_temperature);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &temperature_to_humidity);
    mapped_ranges = get_mapped_ranges(mapped_ranges, &humidity_to_location);

    mapped_ranges
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fn_2() {
        let (input, input2) = get_inputs();
        let res = task_2(input);
        assert!(res == 46);
        println!("result{}", res);

        let res = task_2(input2);
        assert!(res == 125742456);
        println!("result{}", res);
    }
}
