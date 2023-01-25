use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/3/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let diag = get_diag(&list);
        println!("Diag: {} * {} = {}", diag.0, diag.1, diag.0*diag.1);
        let oxy = get_oxy(&list);
        println!("Oxy: {} * {} = {}", oxy.0, oxy.1, oxy.0*oxy.1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_diag(commands: &Vec<String>) -> (i32, i32)
{
    let size = commands[0].len();
    let mut count = vec![0; size];
    for cmd in commands
    {
        cmd.chars()
            .enumerate()
            .filter(|x| x.1 == '1')
            .for_each(|x| count[x.0] = count[x.0] + 1)
    }

    let threshold = commands.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..size
    {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if count[i] > threshold
        {
            gamma = gamma + 1;
        }
        else if count[i] < threshold
        {
            epsilon = epsilon + 1;
        }
        else
        {
            panic!("same count");
        }
    }

    return (gamma, epsilon);
}

fn most_common(commands: &Vec<String>) -> String
{
    let size = commands[0].len();
    let mut remaining = commands.clone();

    for i in 0..size
    {
        let threshold = remaining.len() as f32 / 2.0;
        let count_ones = remaining.iter()
            .map(|x| x.chars().nth(i).unwrap())
            .filter(|x| *x == '1')
            .count() as f32;

        if count_ones >= threshold
        {
            remaining = remaining.iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .cloned()
                .collect();
        }
        else
        {
            remaining = remaining.iter()
                .filter(|x| x.chars().nth(i).unwrap() == '0')
                .cloned()
                .collect();
        }

        if remaining.len() <= 1
        {
            break;
        }
    }

    if remaining.len() != 1
    {
        panic!("too big")
    }

    return remaining[0].clone();
}

fn least_common(commands: &Vec<String>) -> String
{
    let size = commands[0].len();
    let mut remaining = commands.clone();

    for i in 0..size
    {
        let threshold = remaining.len() as f32 / 2.0;
        let count_ones = remaining.iter()
            .map(|x| x.chars().nth(i).unwrap())
            .filter(|x| *x == '1')
            .count() as f32;

        if count_ones >= threshold
        {
            remaining = remaining.iter()
                .filter(|x| x.chars().nth(i).unwrap() == '0')
                .cloned()
                .collect();
        }
        else
        {
            remaining = remaining.iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .cloned()
                .collect();
        }

        if remaining.len() <= 1
        {
            break;
        }
    }

    if remaining.len() != 1
    {
        panic!("too big")
    }

    return remaining[0].clone();
}

fn to_bin(s: &String) -> i32
{
    let size = s.len();
    return s.chars()
        .enumerate()
        .filter(|x| x.1 == '1')
        .map(|x| 1 << (size - x.0 - 1))
        .fold(0, |acc, x| acc + x);
}

pub fn get_oxy(commands: &Vec<String>) -> (i32, i32)
{
    let oxy = to_bin(&most_common(commands));
    let co2 = to_bin(&least_common(commands));
    return (oxy, co2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diag() 
    {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        ];
        assert_eq!((22, 9), get_diag(&input));
    }

    #[test]
    fn test_get_oxy() 
    {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        ];
        assert_eq!((23, 10), get_oxy(&input));
    }

    #[test]
    fn test_to_bin() 
    {
        assert_eq!(23, to_bin(&String::from("10111")));
    }
}