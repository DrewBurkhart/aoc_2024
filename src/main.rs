use std::{collections::HashMap, env, io::Error, time::Instant};

mod day1;
mod day2;

fn main() {
    let problem = env::args()
        .nth(1)
        .expect("Please provide problem number as {day}{problem} i.e. 11");
    get_solution(problem);
}

fn get_solution(problem: String) {
    // Define a lookup table for problem functions
    let mut problems: HashMap<&str, fn(&str) -> Result<(), Error>> = HashMap::new();
    problems.insert("11", day1::problem1);
    problems.insert("12", day1::problem2);
    problems.insert("21", day2::problem1);
    problems.insert("22", day2::problem2);

    // Parse the day number from the problem string
    let day = problem
        .chars()
        .next()
        .expect("Problem identifier cannot be empty")
        .to_string();

    let input_path = format!("inputs/day{}.txt", day);

    let time = Instant::now();

    if let Some(problem_fn) = problems.get(problem.as_str()) {
        // Pass the input file path to the problem function
        if let Err(e) = problem_fn(&input_path) {
            panic!("Problem {} failed: {}. Be better.", problem, e);
        }
    } else {
        panic!("Unknown problem: {}", problem);
    }

    println!("Completed in {} ms", time.elapsed().as_millis());
}
