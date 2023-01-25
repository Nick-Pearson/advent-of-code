use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/2/input.txt") {
        let mut total1 = 0;
        let mut total2 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<_> = ip.split(": ").collect();
                let pattern_parts: Vec<_> = parts[0].split(" ").collect();

                let qtys = parse_min_max_string(pattern_parts[0]);
                let ch = pattern_parts[1].chars().next().unwrap();
                let password = parts[1];
                if is_valid_1(password, qtys, ch)
                {
                    total1 = total1 + 1
                }
                if is_valid_2(password, qtys, ch)
                {
                    total2 = total2 + 1
                }
            }
        }
        println!("1: {}", total1);
        println!("2: {}", total2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn is_valid_1(password: &str, qtys: (usize, usize), ch: char) -> bool
{
    let count = count_chars(password, ch);
    return count >= qtys.0 && count <= qtys.1
}

pub fn is_valid_2(password: &str, qtys: (usize, usize), ch: char) -> bool
{
    let pos1 = password.chars().nth(qtys.0 - 1).map_or(false, |v| ch == v);
    let pos2 = password.chars().nth(qtys.1 - 1).map_or(false, |v| ch == v);
    return pos1 ^ pos2;
}

pub fn parse_min_max_string(input: &str) -> (usize, usize)
{
    let parts: Vec<_> = input.split('-').collect();
    return (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap())
}

pub fn count_chars(input: &str, ch: char) -> usize
{
    let mut count = 0;
    for c in input.chars()
    {
        if c == ch
        {
            count = count + 1;
        }
    }
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_min_max_string() 
    {
        assert_eq!((1, 5), parse_min_max_string("1-5"));
        assert_eq!((8, 9), parse_min_max_string("8-9"));
        assert_eq!((1, 3), parse_min_max_string("1-3"));
    }
    #[test]
    fn test_count_chars() 
    {
        assert_eq!(0, count_chars("bcdefghijk", 'a'));
        assert_eq!(3, count_chars("hello world", 'l'));
    }
    #[test]
    fn test_valid_2() 
    {
        assert!(is_valid_2("baaaaaaaa", (1, 2), 'b'));
        assert!(is_valid_2("abaaaaaaa", (1, 2), 'b'));
        assert!(is_valid_2("aaaaabaaa", (6, 7), 'b'));
        assert!(!is_valid_2("aaaaaaaaa", (1, 2), 'b'));
        assert!(!is_valid_2("bbaaabaaa", (1, 2), 'b'));
        assert!(!is_valid_2("aaaaaaaaa", (998, 999), 'b'));
    }
}