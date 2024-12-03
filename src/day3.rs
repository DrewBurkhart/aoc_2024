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

fn cond_extract_mul_patterns(input: &str) -> Vec<(i32, i32)> {
    let re_combined = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut add_to_vec = true;
    let mut results = Vec::new();

    for cap in re_combined.captures_iter(input) {
        match &cap[0] {
            "do()" => add_to_vec = true,
            "don't()" => add_to_vec = false,
            _ if add_to_vec => {
                if let (Some(x), Some(y)) = (cap.get(2), cap.get(3)) {
                    results.push((
                        x.as_str().parse::<i32>().unwrap(),
                        y.as_str().parse::<i32>().unwrap(),
                    ));
                }
            }
            _ => {}
        }
    }

    results
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

pub fn problem2(file_path: &str) -> io::Result<()> {
    let file_content = read_file(file_path)?;

    let mul_patterns = cond_extract_mul_patterns(&file_content);
    println!("Number of patterns: {}", mul_patterns.len());

    println!(
        "Total: {:?}",
        mul_patterns.iter().fold(0, |acc, (x, y)| acc + x * y)
    );

    Ok(())
}
