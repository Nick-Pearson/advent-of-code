use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Rules {
    rules: HashMap<i64, Vec<i64>>,
    vals: HashMap<i64, Vec<i64>>,
}

impl Rules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
            vals: HashMap::new(),
        }
    }

    fn add_mapping(&mut self, prior: i64, val: i64) {
        let entry = self.rules.entry(prior).or_default();
        entry.push(val);
        let entry = self.vals.entry(val).or_default();
        entry.push(prior);
    }

    fn get_priors(&self, key: &i64) -> Option<&Vec<i64>> {
        self.rules.get(key)
    }

    fn get_vals(&self, key: &i64) -> Option<&Vec<i64>> {
        self.vals.get(key)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let divide = input.find("\n\n").unwrap();
    let rules = parse_rules(&input[..divide]);
    let messages = parse_messages(&input[divide + 2..]);
    println!("Part one: {}", part_one(&rules, &messages));
    println!("Part two: {}", part_two(&rules, &messages));
}

fn parse_rules(input: &str) -> Rules {
    let mut rules = Rules::new();

    for line in input.lines() {
        let (left, right) = line.split('|').collect_tuple().unwrap();
        rules.add_mapping(left.parse().unwrap(), right.parse().unwrap());
    }

    rules
}

fn parse_messages(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn part_one(rules: &Rules, messages: &[Vec<i64>]) -> i64 {
    let mut result = 0;
    for msg in messages {
        if is_valid_message(rules, msg) {
            result += msg[msg.len() / 2];
        }
    }
    result
}

fn is_valid_message(rules: &Rules, message: &[i64]) -> bool {
    let mut seen: HashSet<i64> = HashSet::new();

    for i in message {
        if let Some(priors) = rules.get_priors(i) {
            for prior in priors {
                if seen.contains(prior) {
                    return false;
                }
            }
        }
        seen.insert(*i);
    }
    true
}

fn part_two(rules: &Rules, messages: &[Vec<i64>]) -> i64 {
    let mut result = 0;
    for msg in messages {
        if !is_valid_message(rules, msg) {
            let fixed = fix_message(rules, msg);
            result += fixed[fixed.len() / 2];
        }
    }
    result
}

fn fix_message(rules: &Rules, message: &[i64]) -> Vec<i64> {
    let mut remaining: HashSet<i64> = HashSet::new();
    for i in message {
        remaining.insert(*i);
    }

    let mut fixed = Vec::new();
    while !remaining.is_empty() {
        let mut next = None;
        for i in &remaining {
            if can_be_added_yet(rules, *i, &remaining) {
                next = Some(*i);
                break;
            }
        }
        remaining.remove(&next.unwrap());
        fixed.push(next.unwrap());
    }

    fixed
}

fn can_be_added_yet(rules: &Rules, i: i64, remaining: &HashSet<i64>) -> bool {
    if let Some(vals) = rules.get_vals(&i) {
        for p in vals {
            if remaining.contains(p) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let rules = parse_rules(
            "47|53
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
53|13",
        );
        let messages = parse_messages(
            "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        assert_eq!(part_one(&rules, &messages), 143);
    }

    #[test]
    fn test_part_two() {
        let rules = parse_rules(
            "47|53
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
53|13",
        );
        let messages = parse_messages(
            "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        assert_eq!(part_two(&rules, &messages), 123);
    }
}
