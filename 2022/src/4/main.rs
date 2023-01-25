use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/4/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| parse_input(&r))
            .collect();

        println!("Part 1: {}", part_one(&list));
        println!("Part 2: {}", part_two(&list));
    }
}

fn parse_input(input: &str) -> (Area, Area)
{
    let mut split = input.split(',');
    return (parse_area(split.next().unwrap()), parse_area(split.next().unwrap()));
}

fn parse_area(input: &str) -> Area
{
    let mut split = input.split('-');
    return Area::of(split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
}

#[derive(Debug)]
pub struct Area
{
    start : i32, 
    end : i32
}
impl Area
{
    fn of(s: i32, e: i32) -> Area
    {
        return  Area{
            start: s,
            end: e
        };
    }

    fn contains(&self, b: &Area) -> bool
    {
        return self.start <= b.start && self.end >= b.end;
    }

    fn overlaps(&self, b: &Area) -> bool
    {
        return (b.start >= self.start && b.start <= self.end) ||
            (b.end >= self.start && b.end <= self.end);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one(areas: &Vec<(Area, Area)>) -> usize
{
    return areas.iter()
        .filter(|a| a.0.contains(&a.1) || a.1.contains(&a.0))
        .count();
}

pub fn part_two(areas: &Vec<(Area, Area)>) -> usize
{
    return areas.iter()
        .filter(|a| a.0.overlaps(&a.1) || a.1.overlaps(&a.0))
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            (Area::of(2,4),Area::of(6,8)),
            (Area::of(2,3),Area::of(4,5)),
            (Area::of(5,7),Area::of(7,9)),
            (Area::of(2,8),Area::of(3,7)),
            (Area::of(6,6),Area::of(4,6)),
            (Area::of(2,6),Area::of(4,8))
        ];
        assert_eq!(2, part_one(&input));
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            (Area::of(2,4),Area::of(6,8)),
            (Area::of(2,3),Area::of(4,5)),
            (Area::of(5,7),Area::of(7,9)),
            (Area::of(2,8),Area::of(3,7)),
            (Area::of(6,6),Area::of(4,6)),
            (Area::of(2,6),Area::of(4,8))
        ];
        assert_eq!(4, part_two(&input));
    }
}