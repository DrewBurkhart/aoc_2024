use std::fs::File;
use std::io::{self, BufRead};

fn read_and_sort(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(a), Some(b)) = (parts.get(0), parts.get(1)) {
            col1.push(a.parse::<i32>().unwrap());
            col2.push(b.parse::<i32>().unwrap());
        }
    }

    col1.sort_unstable();
    col2.sort_unstable();

    Ok((col1, col2))
}

fn next_lowest_difference(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> Option<i32> {
    if col1.is_empty() || col2.is_empty() {
        return None;
    }

    let lowest1 = col1.remove(0);
    let lowest2 = col2.remove(0);

    Some((lowest1 - lowest2).abs())
}

fn get_next_similarity_score(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> Option<i32> {
    if col1.is_empty() || col2.is_empty() {
        return None;
    }

    let next = col1.remove(0);
    let occurences = col2.iter().filter(|item| **item == next).count() as i32;

    Some(next*occurences)
}

fn main() -> io::Result<()> {
    let mut total_distance = 0;
    let mut similarity_score = 0;

    let file_path = "input.txt";
    let (mut col1, mut col2) = read_and_sort(file_path)?;
    
    while !col1.is_empty() && !col2.is_empty() {
        if let Some(diff) = next_lowest_difference(&mut col1, &mut col2) {
            total_distance += diff;
        }
    }
    
    let (mut col1, mut col2) = read_and_sort(file_path)?;

    while !col1.is_empty() && !col2.is_empty() {
        if let Some(score) = get_next_similarity_score(&mut col1, &mut col2) {
            similarity_score += score;
        }
    }

    println!("Total Distance: {}", total_distance);
    println!("Similarity Score: {}", similarity_score);

    Ok(())
}
