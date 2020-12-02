use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/1/input.txt") {
        let mut list = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let my_int = ip.parse::<u32>().unwrap();
                list.push(my_int);
            }
        }
        let result = find_pair_that_sums_to(&mut list, 2020);
        println!("{} * {} = {}", result.0, result.1, result.0*result.1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_pair_that_sums_to(numbers: &mut Vec<u32>, target: u32) -> (u32, u32)
{
    numbers.sort();
    let mut low_idx = 0;
    let mut high_idx = numbers.len() - 1;

    while low_idx < high_idx
    {
        let low_val = numbers[low_idx];
        let high_val = numbers[high_idx];
        let result = low_val + high_val;

        if result == target
        {
            return (low_val, high_val);
        }
        else if result > target
        {
            high_idx = high_idx - 1;
        }
        else
        {
            low_idx = low_idx + 1;
        }
    }

    return (0, 0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_items_in_list() 
    {
        let mut vec = Vec::new();
        vec.push(2019);
        vec.push(1);
        assert_eq!((1, 2019), find_pair_that_sums_to(&mut vec, 2020));
    }

    #[test]
    fn multiple_items_in_list_returns_match() 
    {
        let mut vec = Vec::new();
        vec.push(1520);
        vec.push(1000);
        vec.push(17);
        vec.push(1020);
        assert_eq!((1000, 1020), find_pair_that_sums_to(&mut vec, 2020));
    }
}