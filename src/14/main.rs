use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/14/input.txt") {
        let mut itr = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        let init = itr.next().unwrap();
        itr.next();

        let mappings = get_mappings(&itr.collect());
        println!("Part 1: {}", parts(&init, &mappings, 10));
        println!("Part 2: {}", parts(&init, &mappings, 40));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_mappings(input: &Vec<String>) -> Vec<Vec<usize>>
{
    let mut mappings = Vec::new();
    for _i in 0..36
    {
        mappings.push(vec![0; 36]);
    }
    for line in input
    {
        let mut i = line.split(" -> ");
        let key = i.next().unwrap();
        let val = i.next().unwrap().chars().nth(0).unwrap();
        
        let idx_1 = key.chars().nth(0).unwrap().to_digit(36).unwrap() as usize;
        let idx_2 = key.chars().nth(1).unwrap().to_digit(36).unwrap() as usize;
        mappings[idx_1][idx_2] = val.to_digit(36).unwrap() as usize;
    }
    return mappings;
}

#[allow(dead_code)]
fn to_char(digit: usize) -> char
{
    return (digit - 10 + 'A' as usize) as u8 as char;
}

fn process(init: &String, mappings: &Vec<Vec<usize>>, steps: usize) -> Vec<usize>
{
    let mut pair_counts = Vec::new();
    for _i in 0..36
    {
        pair_counts.push(vec![0; 36]);
    }
    let mut prev = init.chars().next().unwrap();
    for c in init.chars().skip(1)
    {
        let i = prev.to_digit(36).unwrap() as usize;
        let j = c.to_digit(36).unwrap() as usize;
        pair_counts[i][j] = pair_counts[i][j] + 1;
        prev = c;
    }

    for _step in 0..steps
    {
        let mut new_counts = Vec::new();
        for _i in 0..36
        {
            new_counts.push(vec![0; 36]);
        }

        for i in 0..36
        {
            for j in 0..36
            {
                let cur = pair_counts[i][j];
                let mapping = mappings[i][j];
                if cur > 0
                {
                    if mapping != 0
                    {
                        new_counts[i][mapping] = new_counts[i][mapping] + cur;
                        new_counts[mapping][j] = new_counts[mapping][j] + cur;
                    }
                    else
                    {
                        new_counts[i][j] = cur;
                    }
                }
            }
        }
        pair_counts = new_counts;
    }

    let mut counts = vec![0; 36];
    counts[init.chars().last().unwrap().to_digit(36).unwrap() as usize] = 1;
    for i in 0..36
    {
        for j in 0..36
        {
            counts[i] = counts[i] + pair_counts[i][j];
        }
    }


    return counts;
}

fn parts(init: &String, mappings: &Vec<Vec<usize>>, steps: usize) -> usize
{
    let mut counts = process(init, mappings, steps);
    counts.retain(|x| *x > 0);
    counts.sort();
    return counts[counts.len() - 1] - counts[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let mappings = get_mappings(&vec![
            String::from("CH -> B"),
            String::from("HH -> N"),
            String::from("CB -> H"),
            String::from("NH -> C"),
            String::from("HB -> C"),
            String::from("HC -> B"),
            String::from("HN -> C"),
            String::from("NN -> C"),
            String::from("BH -> H"),
            String::from("NC -> B"),
            String::from("NB -> B"),
            String::from("BN -> B"),
            String::from("BB -> N"),
            String::from("BC -> B"),
            String::from("CC -> N"),
            String::from("CN -> C")
        ]);
        let init = String::from("NNCB");

        assert_eq!(1588, parts(&init, &mappings, 10));
    }
    #[test]
    fn test_part_two() 
    {
        let mappings = get_mappings(&vec![
            String::from("CH -> B"),
            String::from("HH -> N"),
            String::from("CB -> H"),
            String::from("NH -> C"),
            String::from("HB -> C"),
            String::from("HC -> B"),
            String::from("HN -> C"),
            String::from("NN -> C"),
            String::from("BH -> H"),
            String::from("NC -> B"),
            String::from("NB -> B"),
            String::from("BN -> B"),
            String::from("BB -> N"),
            String::from("BC -> B"),
            String::from("CC -> N"),
            String::from("CN -> C")
        ]);
        let init = String::from("NNCB");

        assert_eq!(2188189693529, parts(&init, &mappings, 40));
    }
}