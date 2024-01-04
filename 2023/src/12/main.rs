use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, space1},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let (_, parsed) = parse(input).unwrap();
    let part_one: usize = parsed
        .iter()
        .map(|t| create_valid_arrangements(t.0, &t.1))
        .sum();
    println!("Part one: {}", part_one);
    let part_two: usize = parsed
        .into_iter()
        .map(|(input, layout)| duplicate(input, layout, 5))
        .map(|t| create_valid_arrangements(t.0.as_str(), &t.1))
        .sum();
    println!("Part two: {}", part_two);
}

fn duplicate(input: &str, layout: Vec<usize>, n: usize) -> (String, Vec<usize>) {
    let s = vec![input].repeat(n).join("?");
    (s, layout.repeat(n))
}

fn create_valid_arrangements(input: &str, layout: &[usize]) -> usize {
    create_arrangements_recursive(input, layout, &mut HashMap::new(), 0, 0, 0)
}

fn create_arrangements_recursive(
    input: &str,
    layout: &[usize],
    memory: &mut HashMap<(usize, usize, usize), usize>,
    mut input_idx: usize,
    mut layout_idx: usize,
    mut num_damaged: usize,
) -> usize {
    let memory_key = (input_idx, layout_idx, num_damaged);
    if let Some(cached) = memory.get(&memory_key) {
        return *cached;
    }

    for (i, c) in input.chars().enumerate() {
        input_idx = input_idx + 1;
        match c {
            '.' => {
                if num_damaged > 0 {
                    if layout.len() <= layout_idx || layout[layout_idx] != num_damaged {
                        memory.insert(memory_key, 0);
                        return 0;
                    }
                    layout_idx = layout_idx + 1;
                    num_damaged = 0;
                }
            }
            '#' => {
                num_damaged = num_damaged + 1;
            }
            '?' => {
                let remaining = &input[i + 1..];

                let mut total = 0;

                if layout.len() > layout_idx && layout[layout_idx] > num_damaged {
                    // path as #
                    total = total
                        + create_arrangements_recursive(
                            remaining,
                            layout,
                            memory,
                            input_idx,
                            layout_idx,
                            num_damaged + 1,
                        );
                }

                if num_damaged == 0 {
                    // path as .
                    total = total
                        + create_arrangements_recursive(
                            remaining, layout, memory, input_idx, layout_idx, 0,
                        );
                } else if layout.len() > layout_idx && layout[layout_idx] == num_damaged {
                    // path as .
                    total = total
                        + create_arrangements_recursive(
                            remaining,
                            layout,
                            memory,
                            input_idx,
                            layout_idx + 1,
                            0,
                        );
                }
                memory.insert(memory_key, total);
                return total;
            }
            _ => panic!("invalid char {}", c),
        }
    }
    if num_damaged > 0 {
        if layout.len() <= layout_idx || layout[layout_idx] != num_damaged {
            memory.insert(memory_key, 0);
            return 0;
        }
        layout_idx = layout_idx + 1;
    }
    if layout_idx == layout.len() {
        memory.insert(memory_key, 1);
        1
    } else {
        // we did not consume all the input
        memory.insert(memory_key, 0);
        0
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, Vec<usize>)>> {
    many1(terminated(parse_line, multispace0))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<usize>)> {
    separated_pair(
        take_until(" "),
        space1,
        many1(terminated(parse_usize, opt(tag(",")))),
    )(input)
}
fn parse_usize(l: &str) -> IResult<&str, usize> {
    map(recognize(digit1), |o: &str| o.parse::<usize>().unwrap())(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(1, create_valid_arrangements("???.###", &vec![1, 1, 3]));
        assert_eq!(
            4,
            create_valid_arrangements(".??..??...?##.", &vec![1, 1, 3])
        );
        assert_eq!(
            1,
            create_valid_arrangements("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6])
        );
        assert_eq!(
            1,
            create_valid_arrangements("????.#...#...", &vec![4, 1, 1])
        );
        assert_eq!(
            4,
            create_valid_arrangements("????.######..#####.", &vec![1, 6, 5])
        );
        assert_eq!(
            10,
            create_valid_arrangements("?###????????", &vec![3, 2, 1])
        );
    }

    #[test]
    fn test_part_two() {
        let d = duplicate("???.###", vec![1, 1, 3], 5);
        assert_eq!(1, create_valid_arrangements(d.0.as_str(), &d.1));
        let d = duplicate(".??..??...?##.", vec![1, 1, 3], 5);
        assert_eq!(16384, create_valid_arrangements(d.0.as_str(), &d.1));
        let d = duplicate("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 5);
        assert_eq!(1, create_valid_arrangements(d.0.as_str(), &d.1));
        let d = duplicate("????.#...#...", vec![4, 1, 1], 5);
        assert_eq!(16, create_valid_arrangements(d.0.as_str(), &d.1));
        let d = duplicate("????.######..#####.", vec![1, 6, 5], 5);
        assert_eq!(2500, create_valid_arrangements(d.0.as_str(), &d.1));
        let d = duplicate("?###????????", vec![3, 2, 1], 5);
        assert_eq!(506250, create_valid_arrangements(d.0.as_str(), &d.1));
    }
}
