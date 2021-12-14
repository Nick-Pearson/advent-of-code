use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/11/input.txt") {
        let input:Vec<Vec<u8>> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.chars().map(|v| v.to_string().parse::<u8>().unwrap()).collect())
            .collect();

        println!("Part 1: {}", part_one(&input, 100));
        println!("Part 2: {}", part_two(&input));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn trigger_flash(input: &mut Vec<Vec<u8>>, x: usize, y: usize)
{
    input[y][x] = input[y][x] + 1;
    if input[y][x] == 10
    {
        let mods = vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
        for m in mods
        {
            let xm = x as i64 + m.0;
            let ym = y as i64 + m.1;
            if xm >= 0 && xm < input[0].len() as i64 &&
                ym >= 0 && ym < input.len() as i64
            {
                trigger_flash(input, xm as usize, ym as usize);
            }
        }
    }
}

fn run_step(input: &mut Vec<Vec<u8>>) -> usize
{
    for y in 0..input.len()
    {
        for x in 0..input[0].len()
        {
            trigger_flash(input, x, y);
        }
    }

    let mut count = 0;
    for y in 0..input.len()
    {
        for x in 0..input[0].len()
        {
            if input[y][x] > 9
            {
                input[y][x] = 0;
                count = count + 1;
            }
        }
    }
    return count;
}

fn part_one(input: &Vec<Vec<u8>>, steps: usize) -> usize
{
    let mut current = input.clone();

    let mut total = 0;
    for _i in 0..steps
    {
        let num_flashes = run_step(&mut current);
        total = total + num_flashes;
    }
    return total;
}

fn part_two(input: &Vec<Vec<u8>>) -> usize
{
    let mut current = input.clone();
    let num_octopuses = input.len() * input[0].len();

    let mut step = 1;
    loop
    {
        let num_flashes = run_step(&mut current);
        if num_flashes == num_octopuses
        {
            return step;
        }
        step = step + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            vec![5,4,8,3,1,4,3,2,2,3],
            vec![2,7,4,5,8,5,4,7,1,1],
            vec![5,2,6,4,5,5,6,1,7,3],
            vec![6,1,4,1,3,3,6,1,4,6],
            vec![6,3,5,7,3,8,5,4,7,8],
            vec![4,1,6,7,5,2,4,6,4,5],
            vec![2,1,7,6,8,4,1,7,2,1],
            vec![6,8,8,2,8,8,1,1,3,4],
            vec![4,8,4,6,8,4,8,5,5,4],
            vec![5,2,8,3,7,5,1,5,2,6]
        ];
        assert_eq!(0, part_one(&input, 1));
        assert_eq!(204, part_one(&input, 10));
        assert_eq!(1656, part_one(&input, 100));
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            vec![5,4,8,3,1,4,3,2,2,3],
            vec![2,7,4,5,8,5,4,7,1,1],
            vec![5,2,6,4,5,5,6,1,7,3],
            vec![6,1,4,1,3,3,6,1,4,6],
            vec![6,3,5,7,3,8,5,4,7,8],
            vec![4,1,6,7,5,2,4,6,4,5],
            vec![2,1,7,6,8,4,1,7,2,1],
            vec![6,8,8,2,8,8,1,1,3,4],
            vec![4,8,4,6,8,4,8,5,5,4],
            vec![5,2,8,3,7,5,1,5,2,6]
        ];
        assert_eq!(195, part_two(&input));
    }
}