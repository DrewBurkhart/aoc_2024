use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn extract_mul_patterns(input: &str) -> Vec<(i32, i32)> {
    // Define the regex pattern
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all matches and parse them into a vector of tuples
    re.captures_iter(input)
        .filter_map(|cap| {
            let x = cap[1].parse::<i32>().ok()?;
            let y = cap[2].parse::<i32>().ok()?;
            Some((x, y))
        })
        .collect()
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn problem1(file_path: &str) -> io::Result<()> {
    let file_content = read_file(file_path)?;

    let mul_patterns = extract_mul_patterns(&file_content);
    println!("Number of patterns: {}", mul_patterns.len());

    println!(
        "Total: {:?}",
        mul_patterns.iter().fold(0, |acc, (x, y)| acc + x * y)
    );

    Ok(())
}
