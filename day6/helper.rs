


pub fn calculate_number_of_posibilities((time, distance): (u64, u64)) -> u64 {
    let mut possible_times: Vec<u64> = vec![];
    for remaining_time in 0..time {
        possible_times.push((time - remaining_time) * remaining_time);
    }

    possible_times.into_iter().filter(|v| *v > distance).count() as u64
}