use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/9/input.txt") {
        let input:Vec<Vec<u8>> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.chars().map(|v| v.to_string().parse::<u8>().unwrap()).collect())
            .collect();

        println!("Part 1: {}", part_one(&input));
        println!("Part 2: {}", part_two(&input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_row(input: &Vec<u8>, val: u8, x: usize) -> bool
{
    let mut result = input[x] >= val &&
        input.get(x + 1).map(|v| *v >= val).unwrap_or(true);

    if x > 0
    {
        result = result && input.get(x - 1).map(|v| *v >= val).unwrap_or(true);
    }
    return result;
}

fn is_low_point(input: &Vec<Vec<u8>>, x: usize, y: usize) -> bool
{
    let target = input[y][x];

    let mut result = check_row(&input[y], target, x) &&
        input.get(y + 1).map(|v| check_row(v, target, x)).unwrap_or(true);

    if y > 0
    {
        result = result && input.get(y - 1).map(|v| check_row(v, target, x)).unwrap_or(true);
    }
    return result;
}

fn get_low_points(input: &Vec<Vec<u8>>) -> Vec<(usize, usize)>
{
    let mut low_points = Vec::new();
    for y in 0..input.len()
    {
        for x in 0..input[0].len()
        {
            if is_low_point(input, x, y)
            {
                low_points.push((x,y));
            }
        }
    }
    return low_points;
}

fn part_one(input: &Vec<Vec<u8>>) -> usize
{
    return get_low_points(input).iter()
        .map(|c| input[c.1][c.0])
        .map(|v| (v + 1) as usize)
        .fold(0, |a,b| a + b);
}

fn get_basin(input: &Vec<Vec<u8>>, point: &(usize,usize)) -> Vec<(usize, usize)>
{
    // println!("checking {:?}", point);
    let mut candidates = Vec::new();

    if point.1 < input.len()-1
    {
        candidates.push((point.0, point.1 + 1));
    }
    if point.1 > 0
    {
        candidates.push((point.0, point.1 - 1));
    }
    if point.0 < input[0].len()-1
    {
        candidates.push((point.0 + 1, point.1));
    }
    if point.0 > 0
    {
        candidates.push((point.0 - 1, point.1));
    }

    let current = input[point.1][point.0];
    let mut result:Vec<(usize, usize)> = candidates.iter()
        .filter(|c| {
            let val = input[c.1][c.0];
            return val != 9 && val > current;
        })
        .map(|c| get_basin(input, c))
        .flatten()
        .collect();
    result.push(*point);
    result.sort_unstable();
    result.dedup();
    // println!("returning {:?}", result);
    return result;
}

fn part_two(input: &Vec<Vec<u8>>) -> usize
{
    let mut sizes:Vec<usize> = get_low_points(input).iter()
        .map(|c| get_basin(input, c).len())
        .collect();
    sizes.sort();

    return sizes[sizes.len() - 3..].iter().fold(1, |a,b| a * b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8]
        ];
        assert_eq!(15, part_one(&input));
    }

    #[test]
    fn test_get_basin() 
    {
        let input = vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8]
        ];
        // assert_eq!(3, get_basin(&input, &(1, 0)).len());
        // assert_eq!(9, get_basin(&input, &(9, 0)).len());
        assert_eq!(14, get_basin(&input, &(2, 2)).len());
        // assert_eq!(9, get_basin(&input, &(6, 4)).len());
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8]
        ];
        assert_eq!(1134, part_two(&input));
    }
}