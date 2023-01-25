use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/15/input.txt") {
        let map:Vec<Vec<u8>> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.chars().map(|v| v.to_string().parse::<u8>().unwrap()).collect())
            .collect();

        println!("Part 1: {}", part_one(&map));
        println!("Part 2: {}", part_two(&map));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const MAX:usize = 9999999999;

fn solve(map: &Vec<Vec<u8>>) -> usize
{
    let mut weights = Vec::new();
    for _i in 0..map.len()
    {
        weights.push(vec![MAX; map[0].len()]);
    }
    weights[0][0] = 0;

    let max_y = map.len();
    let max_x = map[0].len();
    let mut step = 0;

    loop
    {
        let mut changed = false;
        for y in 0..max_y
        {
            for x in 0..max_x
            {
                let risk = map[y][x] as usize;
                let current = weights[y][x];

                let mut new_weight = MAX;
                if y+1 < max_y
                {
                    new_weight = usize::min(new_weight, risk + weights[y+1][x]);
                }
                if y > 0
                {
                    new_weight = usize::min(new_weight, risk + weights[y-1][x]);
                }
                if x+1 < max_x
                {
                    new_weight = usize::min(new_weight, risk + weights[y][x+1]);
                }
                if x > 0
                {
                    new_weight = usize::min(new_weight, risk + weights[y][x-1]);
                }

                if new_weight < current
                {
                    weights[y][x] = new_weight;
                    changed = true;
                }
            }
        }

        step = step + 1;
        if !changed
        {
            break;
        }
    }
    println!("converged after {} steps", step);
    // for y in 0..max_y
    // {
    //     for x in 0..max_x
    //     {
    //         print!("{} ", weights[y][x]);
    //     }
    //     println!("");
    // }

    return weights[max_y-1][max_x-1];
}


fn part_one(map: &Vec<Vec<u8>>) -> usize
{
    return solve(map);
}

fn part_two(map: &Vec<Vec<u8>>) -> usize
{
    let mut new_map = Vec::new();
    let max_y = map.len();
    let max_x = map[0].len();

    for y in 0..max_y
    {
        let mut row = Vec::new();
        for step in 0..5
        {
            for x in 0..max_x
            {
                let a = map[y][x] + step;
                if a > 9
                {
                    row.push(a-9);
                }
                else
                {
                    row.push(a);
                }
            }
        }
        new_map.push(row);
    }
    
    for step in 1..5
    {
        for y in 0..max_y
        {
            let row = new_map[y].iter()
                .map(|x| {
                    let a = x + step;
                    if a > 9
                    {
                        return a-9;
                    }
                    else
                    {
                        return a;
                    }
                })
                .collect();
            new_map.push(row);
        }
    }

    return solve(&new_map);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let map = vec![
            vec![1,1,6,3,7,5,1,7,4,2],
            vec![1,3,8,1,3,7,3,6,7,2],
            vec![2,1,3,6,5,1,1,3,2,8],
            vec![3,6,9,4,9,3,1,5,6,9],
            vec![7,4,6,3,4,1,7,1,1,1],
            vec![1,3,1,9,1,2,8,1,3,7],
            vec![1,3,5,9,9,1,2,4,2,1],
            vec![3,1,2,5,4,2,1,6,3,9],
            vec![1,2,9,3,1,3,8,5,2,1],
            vec![2,3,1,1,9,4,4,5,8,1]
        ];

        assert_eq!(40, part_one(&map));
    }

    #[test]
    fn test_part_two() 
    {
        let map = vec![
            vec![1,1,6,3,7,5,1,7,4,2],
            vec![1,3,8,1,3,7,3,6,7,2],
            vec![2,1,3,6,5,1,1,3,2,8],
            vec![3,6,9,4,9,3,1,5,6,9],
            vec![7,4,6,3,4,1,7,1,1,1],
            vec![1,3,1,9,1,2,8,1,3,7],
            vec![1,3,5,9,9,1,2,4,2,1],
            vec![3,1,2,5,4,2,1,6,3,9],
            vec![1,2,9,3,1,3,8,5,2,1],
            vec![2,3,1,1,9,4,4,5,8,1]
        ];

        assert_eq!(315, part_two(&map));
    }
}