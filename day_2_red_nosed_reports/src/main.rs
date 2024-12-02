use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_safe(line: &str) -> bool {
    let readings: Vec<i32> = match line.split_whitespace().map(|s| s.parse::<i32>()).collect() {
        Ok(v) => v,
        Err(_) => return false,
    };

    if readings.len() < 2 {
        return false;
    }

    let first_diff = readings[1] - readings[0];
    let is_increasing = first_diff > 0;

    for i in 1..readings.len() {
        let diff = readings[i] - readings[i - 1];
        if diff.abs() > 3 || (is_increasing && diff <= 0) || (!is_increasing && diff >= 0) {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() && is_safe(&line) {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);
    Ok(())
}
