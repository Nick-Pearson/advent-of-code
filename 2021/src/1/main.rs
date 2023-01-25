use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/1/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.parse::<u32>().unwrap())
            .collect();

        println!("Num Increases: {}", num_increases(&list));
        println!("Num Increases2: {}", num_increases2(&list));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn num_increases(numbers: &Vec<u32>) -> u32
{
    let mut count = 0;
    for i in 1..numbers.len()
    {
        if numbers[i - 1] < numbers[i]
        {
            count = count + 1;
        }
    }
    return count;
}

pub fn num_increases2(numbers: &Vec<u32>) -> u32
{
    let mut count = 0;
    for i in 1..numbers.len()-2
    {
        if numbers[i - 1] < numbers[i + 2]
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
    fn test_num_increases() 
    {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, num_increases(&input));
    }

    #[test]
    fn test_num_increases2() 
    {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, num_increases2(&input));
    }
}