use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_integer::binomial;

fn main() {
    if let Ok(lines) = read_lines("src/7/input.txt") {
        let init:Vec<i32> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.split(',').map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .flatten()
            .collect();

        let result = get_fuel_needed(&init);
        println!("Position: {},  Fuel: {}", result.0, result.1);
        let result1 = get_fuel_needed2(&init);
        println!("Position2: {},  Fuel2: {}", result1.0, result1.1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_fuel_needed(initial: &Vec<i32>) -> (usize, usize)
{
    let max = initial.iter().fold(0, |a,b| i32::max(a, *b)) as usize;

    return (0..max)
        .map(|pos| (pos, get_fuel_to(initial, pos as i32)))
        .fold((0, 999999999), |a,b| {
            if a.1 < b.1
            {
                return a;
            }
            else
            {
                return b;
            }
        });
}

fn get_fuel_to(initial: &Vec<i32>, position: i32) -> usize
{
    return initial.iter()
        .map(|v| i32::abs(v - position) as usize)
        .fold(0, |a,b| a + b)
}

fn get_fuel_needed2(initial: &Vec<i32>) -> (usize, usize)
{
    let max = initial.iter().fold(0, |a,b| i32::max(a, *b)) as usize;

    return (0..max)
        .map(|pos| (pos, get_fuel_to2(initial, pos as i32)))
        .fold((0, 999999999), |a,b| {
            if a.1 < b.1
            {
                return a;
            }
            else
            {
                return b;
            }
        });
}

fn get_fuel_to2(initial: &Vec<i32>, position: i32) -> usize
{
    return initial.iter()
        .map(|v| i32::abs(v - position) as usize)
        .map(|v| binomial(v+1, 2))
        .fold(0, |a,b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel_needed() 
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!((2, 37), get_fuel_needed(&input));
    }

    #[test]
    fn test_get_fuel_to() 
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, get_fuel_to(&input, 2));
        assert_eq!(41, get_fuel_to(&input, 1));
        assert_eq!(39, get_fuel_to(&input, 3));
    }

    #[test]
    fn test_get_fuel_needed2() 
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!((5, 168), get_fuel_needed2(&input));
    }

    #[test]
    fn test_get_fuel_to2() 
    {
        let input = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(206, get_fuel_to2(&input, 2));
        assert_eq!(168, get_fuel_to2(&input, 5));
    }
}