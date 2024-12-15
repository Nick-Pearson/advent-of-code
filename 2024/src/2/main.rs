use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let count = input.lines().filter(|line| safe(line)).count();
    println!("Part 1: {}", count);
    let count_damp = input.lines().filter(|line| safe_damp(line)).count();
    println!("Part 2: {}", count_damp);
}

fn parse(line: &str) -> Vec<i32> {
    line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect()
}

fn check_rules(numbers: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = numbers.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let all_positive = diffs.iter().all(|&n| n > 0);
    let all_negative = diffs.iter().all(|&n| n < 0);
    let all_valid = diffs.iter().map(|&n| n.abs()).all(|n| (1..=3).contains(&n));
    (all_positive || all_negative) && all_valid
}

fn safe(line: &str) -> bool {
    check_rules(&parse(line))
}

fn safe_damp(line: &str) -> bool {
    let parsed = parse(line);
    if check_rules(&parsed) {
        return true;
    }

    for i in 0..parsed.len() {
        let mut copy = parsed.clone();
        copy.remove(i);
        if check_rules(&copy) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        assert!(safe("7 6 4 2 1"));
        assert!(!safe("1 2 7 8 9"));
        assert!(!safe("9 7 6 2 1"));
        assert!(!safe("1 3 2 4 5"));
        assert!(!safe("8 6 4 4 1"));
        assert!(safe("1 3 6 7 9"));
    }

    #[test]
    fn test_safe_damp() {
        assert!(safe_damp("7 6 4 2 1"));
        assert!(!safe_damp("1 2 7 8 9"));
        assert!(!safe_damp("9 7 6 2 1"));
        assert!(safe_damp("1 3 2 4 5"));
        assert!(safe_damp("8 6 4 4 1"));
        assert!(safe_damp("1 3 6 7 9"));
    }
}
