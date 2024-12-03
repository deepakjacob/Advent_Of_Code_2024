use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();

    let mut total = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("line: {}", line);
                for caps in re.captures_iter(&line) {
                    match (caps[1].parse::<i32>(), caps[2].parse::<i32>()) {
                        (Ok(first), Ok(second)) => {
                            let product = first * second;
                            println!("{}  × {} = {}", first, second, product);
                            total += product;
                        }
                        _ => eprintln!("error parsing numbers: {}", &caps[0]),
                    }
                }
            }
            Err(e) => eprintln!("error reading line: {}", e),
        }
    }

    println!("\nresult of multiplications: {}", total);
    Ok(())
}
