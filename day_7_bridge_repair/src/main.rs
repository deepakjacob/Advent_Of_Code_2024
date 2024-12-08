use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let test_value = parts[0].trim().parse().ok()?;
        let numbers: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().ok())
            .collect::<Option<_>>()?;

        Some(Equation {
            test_value,
            numbers,
        })
    }

    fn evaluate(&self, operators: &[char]) -> i64 {
        let mut result = self.numbers[0];
        for (i, &op) in operators.iter().enumerate() {
            match op {
                '+' => result += self.numbers[i + 1],
                '*' => result *= self.numbers[i + 1],
                _ => unreachable!(),
            }
        }
        result
    }

    fn find_valid_combinations(&self) -> Vec<Vec<char>> {
        let operator_count = self.numbers.len() - 1;
        let mut valid_combinations = Vec::new();

        let total_combinations = 2_i32.pow(operator_count as u32);
        for i in 0..total_combinations {
            let mut combination = Vec::with_capacity(operator_count);
            for j in 0..operator_count {
                let op = if (i & (1 << j)) == 0 { '+' } else { '*' };
                combination.push(op);
            }

            if self.evaluate(&combination) == self.test_value {
                valid_combinations.push(combination);
            }
        }

        valid_combinations
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("input");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match Equation::parse(&line) {
            Some(equation) => {
                let valid_combinations = equation.find_valid_combinations();

                if !valid_combinations.is_empty() {
                    println!("\nEquation: {}", line);

                    for combination in valid_combinations {
                        let mut expression = String::new();
                        for (i, num) in equation.numbers.iter().enumerate() {
                            if i > 0 {
                                expression.push_str(&format!(" {} ", combination[i - 1]));
                            }
                            expression.push_str(&num.to_string());
                        }
                        println!("{} = {}", expression, equation.test_value);
                    }

                    total += equation.test_value;
                }
            }
            None => println!("invalid equation format: {}", line),
        }
    }

    println!("total calibration result: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_equation() {
        let eq = Equation::parse("190: 10 19").unwrap();
        assert_eq!(eq.test_value, 190);
        assert_eq!(eq.numbers, vec![10, 19]);
    }

    #[test]
    fn test_evaluate() {
        let eq = Equation::parse("190: 10 19").unwrap();
        assert_eq!(eq.evaluate(&['*']), 190);
        assert_eq!(eq.evaluate(&['+']), 29);
    }

    #[test]
    fn test_find_valid_combinations() {
        let eq1 = Equation::parse("190: 10 19").unwrap();
        let valid1 = eq1.find_valid_combinations();
        assert_eq!(valid1.len(), 1);
        assert_eq!(valid1[0], vec!['*']);

        let eq2 = Equation::parse("3267: 81 40 27").unwrap();
        let valid2 = eq2.find_valid_combinations();
        assert_eq!(valid2.len(), 2);
    }
}
