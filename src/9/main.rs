use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/9/input.txt") {
        let input:Vec<i64> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let f = find_first_failure(&input, 25).unwrap();
        println!("Invalid Value: {:?}", f);
        let r = find_range_that_sums_to(&input, f).unwrap();
        let hi = input[r.0..r.1].iter().max().unwrap();
        let low = input[r.0..r.1].iter().min().unwrap();
        println!("Range: {:?}", &input[r.0..r.1]);
        println!("Sum: {} + {} = {}", low, hi, low + hi);
    }
}

pub fn has_sum_to(input: &[i64], target: i64) -> bool
{
    for i in 0..input.len()
    {
        for j in i+1..input.len()
        {
            if input[j] + input[i] == target
            {
                return true;
            }
        }
    }
    return false;
}

pub fn find_first_failure(input: &Vec<i64>, preamble_size: usize) -> Option<i64>
{
    for i in preamble_size..input.len()
    {
        let slice = &input[(i - preamble_size)..i];
        let cur = input[i];
        if !has_sum_to(slice, cur)
        {
            return Some(cur);
        }
    }
    return None;
}

pub fn find_range_that_sums_to(input: &Vec<i64>, target: i64) -> Option<(usize, usize)>
{
    let mut low_idx = 0;
    let mut hi_idx = 1;
    while hi_idx < input.len()
    {
        let total:i64 = input[low_idx..hi_idx].iter().sum();
        if total == target
        {
            return Some((low_idx, hi_idx));
        }
        else if total > target
        {
            low_idx = low_idx + 1;
            hi_idx = low_idx + 1;
        }
        else
        {
            hi_idx = hi_idx + 1;
        }
    }
    return None;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_sum_to() 
    {
        assert_eq!(true, has_sum_to(&[1, 2, 3, 4, 5], 5));
        assert_eq!(true, has_sum_to(&[35, 20, 15, 25, 47], 40));
        assert_eq!(false, has_sum_to(&[95, 102, 117, 150, 182], 127));
    }
    
    #[test]
    fn test_find_first_failure() 
    {
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        assert_eq!(Some(127), find_first_failure(&input, 5));
    }
    
    #[test]
    fn test_find_range_that_sums_to() 
    {
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        let range = find_range_that_sums_to(&input, 127);
        assert_eq!(Some((2, 6)), range);
        let r = range.unwrap();
        assert_eq!(Some(15), input[r.0..r.1].iter().min().copied());
        assert_eq!(Some(47), input[r.0..r.1].iter().max().copied());
    }
}