use std::fs::read_to_string;

fn check_order(before: i64, after: i64, sequence: &[i64]) -> bool {
    let pos_before = sequence.iter().position(|&x| x == before);
    let pos_after = sequence.iter().position(|&x| x == after);

    match (pos_before, pos_after) {
        (Some(pb), Some(pa)) => pb < pa,
        _ => true,
    }
}

fn is_valid_sequence(sequence: &[i64], rules: &[(i64, i64)]) -> bool {
    rules
        .iter()
        .all(|&(before, after)| check_order(before, after, sequence))
}

fn solve(input: &str) -> i64 {
    let mut parts = input.trim().split("\n\n");

    let rules: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('|').map(|n| n.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let sequences: Vec<Vec<i64>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let valid_sequences: Vec<_> = sequences
        .iter()
        .filter(|seq| is_valid_sequence(seq, &rules))
        .collect();

    for seq in &valid_sequences {
        println!("valid: {:?} (middle: {})", seq, seq[seq.len() / 2]);
    }

    valid_sequences.iter().map(|seq| seq[seq.len() / 2]).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input")?;
    let result = solve(&input);
    println!("sum of middle numbers: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve(input), 143);
    }
}
