use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/1/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let result = highest_elf(&list);
        println!("Part 1: {}", result[0]);
        println!("Part 2: {}", result[0]+result[1]+result[2]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn highest_elf(numbers: &Vec<String>) -> Vec<u32>
{
    let mut items = Vec::new();
    let mut max = Vec::new();

    for item in numbers
    {
        if item.is_empty()
        {
            let sum = items.iter().fold(0, |acc, x| acc + x);
            items.clear();

            max.push(sum);
        }
        else
        {
            items.push(item.parse::<u32>().unwrap());
        }
    }
    let sum = items.iter().fold(0, |acc, x| acc + x);
    max.push(sum);

    max.sort_by(|a, b| b.cmp(a));

    return max;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_elf() 
    {
        let input = vec![
            String::from("1000"),
            String::from("2000"),
            String::from("3000"),
            String::from(""),
            String::from("4000"),
            String::from(""),
            String::from("5000"),
            String::from("6000"),
            String::from(""),
            String::from("7000"),
            String::from("8000"),
            String::from("9000"),
            String::from(""),
            String::from("10000")
        ];
        assert_eq!(24000, highest_elf(&input)[0]);
        assert_eq!(11000, highest_elf(&input)[1]);
        assert_eq!(10000, highest_elf(&input)[2]);
    }
}