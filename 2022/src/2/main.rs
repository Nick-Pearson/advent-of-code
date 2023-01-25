use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/2/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        println!("Part 1: {}", score_strategy(&list));
        println!("Part 2: {}", score_strategy2(&list));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn score_strategy(strat: &Vec<String>) -> u32
{
    let mut score = 0;
    for round in strat
    {
        let theirs = round.chars().nth(0).unwrap();
        let mine = round.chars().nth(2).unwrap();
        score = score + score_mine(mine);
        score = score + score_game(theirs, mine);
    }
    return score;
}

pub fn score_strategy2(strat: &Vec<String>) -> u32
{
    let mut score = 0;
    for round in strat
    {
        let theirs = round.chars().nth(0).unwrap();
        let result = round.chars().nth(2).unwrap();
        let mine = pick_mine(theirs, result);
        score = score + score_mine(mine);
        score = score + score_game2(result);
    }
    return score;
}

pub fn score_mine(c: char) -> u32
{
    return match c
    {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("invalid char {}", c)
    };
}
pub fn score_game2(result: char) -> u32
{
    return match result
    {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("invalid char {}", result)
    };
}

pub fn score_game(theirs: char, mine: char) -> u32
{
    return match mine
    {
        'X' => match theirs {
            'A' => 3,
            'B' => 0,
            'C' => 6,
            _ => panic!("invalid char {}", theirs)
        },
        'Y' => match theirs {
            'A' => 6,
            'B' => 3,
            'C' => 0,
            _ => panic!("invalid char {}", theirs)
        },
        'Z' => match theirs {
            'A' => 0,
            'B' => 6,
            'C' => 3,
            _ => panic!("invalid char {}", theirs)
        },
        _ => panic!("invalid char {}", mine)
    };
}

pub fn pick_mine(theirs: char, result: char) -> char
{
    return match result
    {
        'X' => match theirs {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => panic!("invalid char {}", theirs)
        },
        'Y' => match theirs {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => panic!("invalid char {}", theirs)
        },
        'Z' => match theirs {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => panic!("invalid char {}", theirs)
        },
        _ => panic!("invalid char {}", result)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_strategy() 
    {
        let input = vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z")
        ];
        assert_eq!(15, score_strategy(&input));
    }

    #[test]
    fn test_score_strategy2() 
    {
        let input = vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z")
        ];
        assert_eq!(12, score_strategy2(&input));
    }
}