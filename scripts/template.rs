use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    0
}

fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "";
        assert_eq!(part_one(input), 0);
    }

    #[test]
    fn test_part_two() {
        let input = "";
        assert_eq!(part_two(input), 0);
    }
}
