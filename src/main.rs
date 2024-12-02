use std::{collections::HashMap, env, io::Error, time::Instant};

mod day1;

fn main() {
    let problem = env::args()
        .nth(1)
        .expect("Please provide problem number as {day}{problem} i.e. 11");
    get_solution(problem);
}

fn get_solution(problem: String) {
    // Define a lookup table for problem functions
    let mut problems: HashMap<&str, fn() -> Result<(), Error>> = HashMap::new();
    problems.insert("11", day1::problem1);
    problems.insert("12", day1::problem2);

    let time = Instant::now();

    if let Some(problem_fn) = problems.get(problem.as_str()) {
        // Call the corresponding function and handle errors
        if let Err(e) = problem_fn() {
            panic!("Problem {} failed: {}. Be better.", problem, e);
        }
    } else {
        panic!("Unknown problem: {}", problem);
    }

    println!("Completed in {} ms", time.elapsed().as_millis());
}
