use crate::common::Solution;
use std::env;

mod common;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    println!("Running solution for day {}", day);

    let filename = format!("input/day_{}.txt", day);
    let lines = common::read_input_for_day(filename);
    if day == "1" { display_solution(&day_1::SolutionDay1 { lines }); }
    else if day == "2" { display_solution(&day_2::SolutionDay2 { lines }); }
    else if day == "3" { display_solution(&day_3::SolutionDay3 { lines }); }
}

fn display_solution(solution: &impl Solution) {
    match solution.part_1() {
        Ok(answer) => println!("Answer for part 1 is: {}", answer),
        Err(e) => println!("Error for part 1: {}", e)
    }
    match solution.part_2() {
        Ok(answer) => println!("Answer for part 2 is: {}", answer),
        Err(e) => println!("Error for part 2: {}", e)
    }
}
