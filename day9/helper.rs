
pub fn parse_input(
    input: String,
) -> Vec<Vec<i32>> {
    let lines = input.lines();
   lines.fold(Vec::new(), |mut acc, line| {
		let mut line = line.split_whitespace();
		let mut numbers = Vec::new();
		while let Some(number) = line.next() {
			numbers.push(number.trim().parse::<i32>().unwrap());
		}
		acc.push(numbers);
		acc
	})
    
}
