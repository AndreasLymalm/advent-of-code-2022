use std::env;

mod common;
mod day_1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    println!("Running solution for day {}", day);

    let filename = format!("input/day_{}.txt", day);
    let solution = &day_1::SolutionDay1 { lines: common::read_input_for_day(filename) };
    display_solution(solution);
}

fn display_solution(solution: &impl common::Solution) {
    match solution.part_1() {
        Ok(answer) => println!("Answer for part 1 is: {}", answer),
        Err(e) => println!("Error for part 1: {}", e)
    }
    match solution.part_2() {
        Ok(answer) => println!("Answer for part 2 is: {}", answer),
        Err(e) => println!("Error for part 2: {}", e)
    }
}
