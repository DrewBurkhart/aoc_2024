use std::fs::File;
use std::io::{self, BufRead, Result};

fn read_lines(file_path: &str) -> Result<Vec<Vec<i32>>> {
    let mut lines = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        lines.push(levels);
    }

    Ok(lines)
}

fn is_valid_line(values: &[i32]) -> bool {
    // Check if the line is strictly ascending or descending
    let is_ascending = values.windows(2).all(|w| w[1] > w[0]);
    let is_descending = values.windows(2).all(|w| w[1] < w[0]);

    // Check if all differences are within the range [1, 3]
    let are_differences_valid = values.windows(2).all(|w| {
        let diff = (w[1] - w[0]).abs();
        diff >= 1 && diff <= 3
    });

    (is_ascending || is_descending) && are_differences_valid
}

pub fn problem1(file_path: &str) -> Result<()> {
    let mut total_safe = 0;

    let mut lines = read_lines(file_path)?;

    while !lines.is_empty() {
        let values = lines.remove(0);

        if is_valid_line(&values) {
            println!("Valid line: {:?}", values);
            total_safe += 1;
        } else {
            println!("Invalid line: {:?}", values);
        }
    }

    println!("Total Safe: {}", total_safe);

    Ok(())
}

pub fn problem2(file_path: &str) -> Result<()> {
    let mut total_safe = 0;

    let mut lines = read_lines(file_path)?;

    while !lines.is_empty() {
        let values = lines.remove(0);

        // Check if the line is valid as is
        if is_valid_line(&values) {
            println!("Valid line: {:?}", values);
            total_safe += 1;
        } else {
            // Check if removing one item makes it valid
            let mut is_valid_with_removal = false;

            for i in 0..values.len() {
                let mut temp_line = values.clone();
                temp_line.remove(i);
                if is_valid_line(&temp_line) {
                    println!(
                        "Line {:?} becomes valid after removing {}: {:?}",
                        values, values[i], temp_line
                    );
                    is_valid_with_removal = true;
                    total_safe += 1;
                    break;
                }
            }

            if !is_valid_with_removal {
                println!("Invalid line: {:?}", values);
            }
        }
    }

    println!("Total Safe: {}", total_safe);

    Ok(())
}
