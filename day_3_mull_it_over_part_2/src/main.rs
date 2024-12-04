use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Operation {
    position: usize,
    op_type: String,
    values: Option<(i32, i32)>,
}

fn parse_operations(line: &str) -> Vec<Operation> {
    let mul_re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();
    let do_re = Regex::new(r"\w*do\(\)").unwrap(); // matches any word ending in do()
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut operations = Vec::new();

    for mat in do_re.find_iter(line) {
        operations.push(Operation {
            position: mat.start(),
            op_type: "do".to_string(),
            values: None,
        });
    }

    for mat in dont_re.find_iter(line) {
        operations.push(Operation {
            position: mat.start(),
            op_type: "dont".to_string(),
            values: None,
        });
    }

    for caps in mul_re.captures_iter(line) {
        if let (Some(first), Some(second)) =
            (caps[1].parse::<i32>().ok(), caps[2].parse::<i32>().ok())
        {
            operations.push(Operation {
                position: caps.get(0).unwrap().start(),
                op_type: "mul".to_string(),
                values: Some((first, second)),
            });
        }
    }

    operations.sort_by_key(|op| op.position);
    operations
}

fn process_line(line: &str) -> i32 {
    let mut total = 0;
    let mut enabled = true;
    let mut count = 0;

    for op in parse_operations(line) {
        match op.op_type.as_str() {
            "do" => enabled = true,
            "dont" => enabled = false,
            "mul" => {
                if enabled {
                    if let Some((x, y)) = op.values {
                        let product = x * y;
                        count += 1;
                        println!("{}. {} × {} = {}", count, x, y, product);
                        total += product;
                    }
                }
            }
            _ => {}
        }
    }

    println!("\ntotal sum: {}", total);
    total
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let content = reader
        .lines()
        .collect::<io::Result<Vec<String>>>()?
        .join("\n");

    process_line(&content);
    Ok(())
}
