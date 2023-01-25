use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/3/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        println!("Part 1: {}", part_one(&list));
        println!("Part 2: {}", part_two(&list));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one(bags: &Vec<String>) -> usize
{
    return bags.iter()
        .map(|x| score_bag(x))
        .fold(0, |x,acc| x + acc);
}

pub fn score_bag(bag: &String) -> usize
{
    let mut scores: [u8; 52] = [0; 52];

    let len = bag.chars().count() / 2;
    let mut i = 0;
    for c in bag.chars()
    {
        let index = get_priority(c);
        let score = scores[index];

        if i >= len
        {
            if score > 0
            {
                return index + 1;
            }
        }
        else
        {
            scores[index] = score + 1;
            i = i + 1;
        }
    }

    panic!("ahhhh");
}

pub fn get_priority(c : char) -> usize
{
    let val = c as usize;
    let a = 'a' as usize;
    let b = 'A' as usize;
    if val > a
    {
        return val - a;
    }
    else
    {
        return (val - b) + 26;
    }
}

pub fn part_two(bags: &Vec<String>) -> usize
{
    let mut it = bags.iter();
    let mut score = 0;
    while let Some(line) = it.next()
    {
        score = score + sticker_score(line, it.next().unwrap(), it.next().unwrap());
    }
    return score;
}

pub fn sticker_score(bag1: &String, bag2: &String, bag3: &String) -> usize
{
    let mut mask:u64 = 0xffffffffffffffff;
    mask = mask & mask_bag(bag1);
    mask = mask & mask_bag(bag2);
    mask = mask & mask_bag(bag3);

    let mut i = 1;
    while mask & 1 == 0
    {
        mask = mask >> 1;
        i = i + 1;
    }
    return i;
}

pub fn mask_bag(bag: &String) -> u64
{
    let mut mask:u64 = 0;

    for c in bag.chars()
    {
        let index = get_priority(c);
        mask = mask | (1 << index);
    }

    return mask;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() 
    {
        assert_eq!(15, get_priority('p'));
        assert_eq!(37, get_priority('L'));
        assert_eq!(41, get_priority('P'));
        assert_eq!(21, get_priority('v'));
        assert_eq!(19, get_priority('t'));
        assert_eq!(18, get_priority('s'));
    }

    #[test]
    fn test_score_bag() 
    {
        // vJrwpWtwJgWr hcsFMMfFFhFp
        assert_eq!(16, score_bag(&String::from("vJrwpWtwJgWrhcsFMMfFFhFp")));
        assert_eq!(38, score_bag(&String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")));
        assert_eq!(42, score_bag(&String::from("PmmdzqPrVvPwwTWBwg")));
        assert_eq!(22, score_bag(&String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")));
        assert_eq!(20, score_bag(&String::from("ttgJtRGJQctTZtZT")));
        assert_eq!(19, score_bag(&String::from("CrZsJsPPZsGzwwsLwLmpwMDw")));
    }

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
        ];
        assert_eq!(157, part_one(&input));
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
        ];
        assert_eq!(70, part_two(&input));
    }
}