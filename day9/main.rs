mod helper;
mod task_1;
mod tast_2;
use common::get_inputs;
use task_1::task_1;
use tast_2::task_2;

fn main() {
    let (input, _) = get_inputs();

    println!("task 1 result: {}", task_1(input.clone()));

    println!("task 2 result: {}", task_2(input.clone()));
}
