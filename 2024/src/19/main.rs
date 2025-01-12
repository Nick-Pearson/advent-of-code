use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    let (towels, required) = parse(input);
    required
        .iter()
        .filter(|f| num_ways_to_create(f, &towels) > 0)
        .count()
}

fn part_two(input: &str) -> usize {
    let (towels, required) = parse(input);
    required
        .iter()
        .map(|f| num_ways_to_create(f, &towels))
        .sum()
}

fn num_ways_to_create(required: &str, towels: &Vec<&str>) -> usize {
    num_ways_to_create_memoized(required, towels, &mut HashMap::new())
}

fn num_ways_to_create_memoized(
    required: &str,
    towels: &Vec<&str>,
    mem: &mut HashMap<String, usize>,
) -> usize {
    if required.is_empty() {
        return 1;
    }

    if let Some(&ways) = mem.get(required) {
        return ways;
    }

    let mut ways = 0;
    for t in towels {
        if required.starts_with(t) {
            ways += num_ways_to_create_memoized(&required[t.len()..], towels, mem);
        }
    }

    if required.len() > 5 {
        mem.insert(required.to_string(), ways);
    }
    ways
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .sorted_by_key(|x| 100000 - x.len())
        .collect();
    lines.next().unwrap();
    let required = lines.collect();
    (towels, required)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part_one(input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part_two(input), 16);
    }
}
