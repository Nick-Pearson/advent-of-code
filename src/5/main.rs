use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/5/input.txt") {
        let input:Vec<LineSegment> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.replace(" -> ", ","))
            .map(|r| r.split(',').map(|r| r.parse::<i32>().unwrap()).collect())
            .map(|r| LineSegment::from_vec(&r))
            .collect();

        let orth_lines = input.iter()
            .filter(|r| is_orth(r))
            .cloned()
            .collect();
        println!("Num Intersections: {}", num_crosses(&orth_lines));
        println!("Num Intersections2: {}", num_crosses(&input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct LineSegment
{
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32
}

impl LineSegment
{
    fn from_vec(vals: &Vec<i32>) -> LineSegment
    {
        return  LineSegment{
            x0: vals[0],
            y0: vals[1],
            x1: vals[2],
            y1: vals[3]
        };
    }
}

impl Clone for LineSegment
{
    fn clone(&self) -> LineSegment
    {
        return  LineSegment{
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1
        };
    }
}

fn is_orth(input: &LineSegment) -> bool
{
    return input.x0 == input.x1 || 
        input.y0 == input.y1;
}

fn draw_line(map: &mut Vec<Vec<usize>>, line: &LineSegment)
{
    let dir = (line.x1 - line.x0, line.y1 - line.y0);
    let len ;
    if dir.0 != 0
    {
        len = i32::abs(dir.0);
    }
    else
    {
        len = i32::abs(dir.1);
    }
    let step = (dir.0 / len, dir.1 / len);

    for i in 0..len+1
    {
        let x = (line.x0 + (step.0 * i)) as usize;
        let y = (line.y0 + (step.1 * i)) as usize;
        map[y][x] = map[y][x] + 1;
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<usize>>)
{
    let max_y = map.len();
    let max_x = map[0].len();
    for y in 0..max_y
    {
        for x in 0..max_x
        {
            let val = map[y][x];
            if val > 1
            {
                print!("{}", val);
            }
            else
            {
                print!(".");
            }
        }
        println!("");
    }
}

fn num_crosses(input: &Vec<LineSegment>) -> usize
{
    let max_x = input.iter()
        .map(|r| i32::max(r.x0, r.x1))
        .map(|r| r + 1)
        .fold(-1, |a, b| i32::max(a, b)) as usize;
    let max_y = input.iter()
        .map(|r| i32::max(r.y0, r.y1))
        .map(|r| r + 1)
        .fold(-1, |a, b| i32::max(a, b)) as usize;

    let mut map = Vec::new();
    for _y in 0..max_y 
    {
        map.push(vec![0; max_x]);
    }

    for line in input
    {
        draw_line(&mut map, line);
    }

    // print_map(&map);

    return map.iter()
        .flatten()
        .filter(|x| **x > 1)
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_crosses() 
    {
        let input = vec![
            LineSegment::from_vec(&vec![0,9, 5,9]),
            LineSegment::from_vec(&vec![9,4, 3,4]),
            LineSegment::from_vec(&vec![2,2, 2,1]),
            LineSegment::from_vec(&vec![7,0, 7,4]),
            LineSegment::from_vec(&vec![0,9, 2,9]),
            LineSegment::from_vec(&vec![3,4, 1,4])
        ];
        assert_eq!(5, num_crosses(&input));
    }

    #[test]
    fn test_num_crosses2() 
    {
        let input = vec![
            LineSegment::from_vec(&vec![0,9, 5,9]),
            LineSegment::from_vec(&vec![8,0, 0,8]),
            LineSegment::from_vec(&vec![9,4, 3,4]),
            LineSegment::from_vec(&vec![2,2, 2,1]),
            LineSegment::from_vec(&vec![7,0, 7,4]),
            LineSegment::from_vec(&vec![6,4, 2,0]),
            LineSegment::from_vec(&vec![0,9, 2,9]),
            LineSegment::from_vec(&vec![3,4, 1,4]),
            LineSegment::from_vec(&vec![0,0, 8,8]),
            LineSegment::from_vec(&vec![5,5, 8,2])
        ];
        assert_eq!(12, num_crosses(&input));
    }
}