use std::fs;

pub fn get_inputs() -> (String, String) {
    let input =
        fs::read_to_string("./input.txt").expect("something went wrong");
    let input2 =
        fs::read_to_string("./input2.txt").expect("something went wrong");

    (input, input2)
}
pub fn get_custom_input(file_name: &str) -> String {
	fs::read_to_string(file_name).expect("something went wrong")
}