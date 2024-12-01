use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn process_line(line: &str, line_num: usize) -> Result<(i64, i64), String> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| {
            s.parse::<i64>()
                .map_err(|_| format!("unable to parse number on line {}", line_num))
        })
        .collect::<Result<Vec<i64>, String>>()?;

    if numbers.len() != 2 {
        return Err(format!("expecting 2 numbers on line {}", line_num));
    }

    Ok((numbers[0], numbers[1]))
}

fn read_numbers_from_file<P: AsRef<Path>>(path: P) -> Result<(Vec<i64>, Vec<i64>), String> {
    let file = File::open(path).map_err(|e| format!("file cannot be opened: {}", e))?;
    let reader = BufReader::new(file);
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    let mut line_count = 0;

    for line in reader.lines() {
        line_count += 1;
        let line = line.map_err(|e| format!("failed to read line {}: {}", line_count, e))?;
        let (first, second) = process_line(&line, line_count)?;
        first_numbers.push(first);
        second_numbers.push(second);
    }

    println!("total lines        : {}", line_count);
    println!("first column count : {}", first_numbers.len());
    println!("second column count: {}", second_numbers.len());

    if first_numbers.len() != second_numbers.len() {
        return Err(format!(
            "expecped to match number count in colums: first={}, second={}",
            first_numbers.len(),
            second_numbers.len()
        ));
    }

    Ok((first_numbers, second_numbers))
}

fn create_sorted_tuples(mut first: Vec<i64>, mut second: Vec<i64>) -> Vec<(i64, i64)> {
    first.sort_unstable();
    second.sort_unstable();
    first.into_iter().zip(second.into_iter()).collect()
}

fn calculate_abs_diff((a, b): &(i64, i64)) -> i64 {
    if a > b {
        // println!("{} - {} = {}", a, b, a - b);
        a - b
    } else {
        // println!("{} - {} = {}", b, a, b - a);
        b - a
    }
}

fn sum_differences(tuples: &[(i64, i64)]) -> i64 {
    // println!("\ncalculating the diff:");
    let sum = tuples.iter().map(calculate_abs_diff).sum();
    // println!("\sum of differences: {}", sum);
    sum
}

fn process_file<P: AsRef<Path>>(path: P) -> Result<i64, String> {
    let (first_numbers, second_numbers) = read_numbers_from_file(path)?;
    let tuples = create_sorted_tuples(first_numbers, second_numbers);
    Ok(sum_differences(&tuples))
}

fn main() {
    match process_file("input") {
        Ok(result) => println!("\ntotal distance: {}\n\n", result),
        Err(e) => eprintln!("error occurred: {}", e),
    }
}
