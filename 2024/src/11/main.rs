use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input, 25));
    println!("Part two: {}", part_one(input, 75));
}

fn part_one(input: &str, blinks: usize) -> usize {
    let input: Vec<i64> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect_vec();

    let mut result = 0;
    for stone in input {
        result += count_blinks(stone, blinks, &mut HashMap::new());
    }
    result
}

fn count_blinks(
    mut stone: i64,
    mut blinks: usize,
    memory: &mut HashMap<(i64, usize), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }

    let m = memory.get(&(stone, blinks));
    if let Some(&value) = m {
        return value;
    }

    for _ in 0..blinks {
        if stone == 0 {
            stone = 1;
            blinks -= 1;
        } else {
            let num_digits = digits(stone);
            if num_digits % 2 == 0 {
                let half = num_digits / 2;
                let ten_pow = 10_i64.pow(half as u32);
                let a = stone / ten_pow;
                let b = stone - (a * ten_pow);
                let result =
                    count_blinks(a, blinks - 1, memory) + count_blinks(b, blinks - 1, memory);
                memory.insert((stone, blinks), result);
                return result;
            } else {
                stone *= 2024;
                blinks -= 1;
            }
        }
    }
    1
}

fn digits(n: i64) -> usize {
    if n == 0 {
        1
    } else {
        (n.abs() as f64).log10().floor() as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("125 17", 6), 22);
        assert_eq!(part_one("125 17", 25), 55312);
    }
}
