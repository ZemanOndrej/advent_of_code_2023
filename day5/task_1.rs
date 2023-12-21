use crate::helper::{get_mapped_value, get_ranges};

pub fn task_1(input: String) -> u64 {
    let mut lines = input.lines();
    let seed_line: Vec<_> = lines.next().unwrap().split(":").collect();
    let seeds: Vec<_> = seed_line[1].trim().split(" ").collect();
    lines.next().unwrap();

    let (lines, seed_to_soil) = get_ranges(lines);
    let (lines, soil_to_fertilizer) = get_ranges(lines);
    let (lines, fertilizer_to_water) = get_ranges(lines);
    let (lines, water_to_light) = get_ranges(lines);
    let (lines, light_to_temperature) = get_ranges(lines);
    let (lines, temperature_to_humidity) = get_ranges(lines);
    let (_, humidity_to_location) = get_ranges(lines);

    seeds
        .iter()
        .map(|val| {
            let mut val = val.parse::<u64>().unwrap();
            val = get_mapped_value(val, &seed_to_soil);
            val = get_mapped_value(val, &soil_to_fertilizer);
            val = get_mapped_value(val, &fertilizer_to_water);
            val = get_mapped_value(val, &water_to_light);
            val = get_mapped_value(val, &light_to_temperature);
            val = get_mapped_value(val, &temperature_to_humidity);
            val = get_mapped_value(val, &humidity_to_location);
            val
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_inputs;

    #[test]
    fn test_fask_1() {
        let (input, input2) = get_inputs();
        let res = task_1(input);
        assert!(res == 35);
        println!("result{}", res);

        let res = task_1(input2);
        println!("result{}", res);
    }
}
