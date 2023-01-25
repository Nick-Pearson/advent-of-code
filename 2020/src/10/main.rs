use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/10/input.txt") {
        let mut input:Vec<i64> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();    
        input.sort();

        let diffs = calc_diffs(&input);
        println!("Result: {} * {} = {:?}", diffs[0], diffs[2], diffs[0] * diffs[2]);
        let perms = permutations(&input);
        println!("Permuations: = {:?}", perms);
    }
}

pub fn calc_diffs(input: &Vec<i64>) -> [i64; 3]
{
    let mut diffs = [0, 0, 0];
    let mut prev = 0;
    for i in input.iter().copied()
    {
        let diff:usize = (i - prev - 1) as usize;
        diffs[diff] = diffs[diff] + 1;
        prev = i;
    }
    diffs[2] = diffs[2] + 1;
    return diffs;
}

pub fn permutations(input: &Vec<i64>) -> i64
{
    let mut diffs = Vec::new();
    let mut prev = 0;
    for i in input.iter().copied()
    {
        diffs.push(i - prev);
        prev = i;
    }

    let mut perms = Vec::new();
    perms.push(1);
    let mut count = 1;
    for i in 0..diffs.len()
    {
        let d = diffs[diffs.len() - i - 1] == 1;
        let d2 = diffs.get(diffs.len() - i).map_or(false, |x| *x == 1);
        let d3 = diffs.get(diffs.len() - i + 1).map_or(false, |x| *x == 1);
        if d && d2 && d3
        {
            count = perms[perms.len() - 1] + perms[perms.len() - 2] + perms[perms.len() - 3];
        }
        else if d & d2
        {
            count = perms[perms.len() - 1] + perms[perms.len() - 2];
        }

        perms.push(count);
    }
    return perms[perms.len() - 1];
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
    fn test_calc_diffs() 
    {
        assert_eq!([1, 0, 1], calc_diffs(&vec![1]));
        assert_eq!([2, 0, 1], calc_diffs(&vec![1, 2]));
        assert_eq!([2, 0, 2], calc_diffs(&vec![1, 2, 5]));
        assert_eq!([0, 0, 2], calc_diffs(&vec![3]));
        assert_eq!([3, 0, 2], calc_diffs(&vec![1, 2, 5, 6]));
        let mut sml = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ];
        sml.sort();
        assert_eq!([7, 0, 5], calc_diffs(&sml));
        let mut big = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3            
        ];
        big.sort();
        assert_eq!([22, 0, 10], calc_diffs(&big));
    }

    
    #[test]
    fn test_permutations() 
    {
        assert_eq!(1, permutations(&vec![1]));
        assert_eq!(1, permutations(&vec![1,4]));
        let mut sml = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ];
        sml.sort();
        assert_eq!(8, permutations(&sml));
        let mut big = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3            
        ];
        big.sort();
        assert_eq!(19208, permutations(&big));
    }
}