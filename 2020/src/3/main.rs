use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/3/input.txt") {
        let mut list = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                list.push(parse_line(&ip))
            }
        }

        let res1 = num_trees(&list, 1, 1);
        println!("1,1 = {}", res1);
        let res2 = num_trees(&list, 3, 1);
        println!("3,1 = {}", res2);
        let res3 = num_trees(&list, 5, 1);
        println!("5,1 = {}", res3);
        let res4 = num_trees(&list, 7, 1);
        println!("7,1 = {}", res4);
        let res5 = num_trees(&list, 1, 2);
        println!("1,2 = {}", res5);
        println!("total = {}", res1*res2*res3*res4*res5);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_line(input: &str) -> Vec<bool>
{
    let mut data = Vec::new();
    for c in input.chars()
    {
        data.push(c == '#');
    }
    return data
}

pub fn num_trees(map: &Vec<Vec<bool>>, x_inc: usize, y_inc: usize) -> u32
{
    let max_x = map[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut num = 0;
    while y < map.len()
    {
        if map[y][x]
        {
            num = num + 1;
        }
        x = (x + x_inc) % max_x;
        y = y + y_inc;
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() 
    {
        assert_eq!([false, false, false, false, false, false], parse_line("......").as_slice());
        assert_eq!([true, true, true, true, true, true], parse_line("######").as_slice());
        assert_eq!([false, false, true, true, false, false], parse_line("..##..").as_slice());
    }

    #[test]
    fn test_num_trees() 
    {
        let mut list = Vec::new();
        list.push(parse_line("......"));
        list.push(parse_line("...#.."));
        list.push(parse_line("#....."));
        list.push(parse_line("...#.."));
        list.push(parse_line("#....."));

        assert_eq!(4, num_trees(&list, 3, 1));
    }

    #[test]
    fn test_num_trees_y() 
    {
        let mut list = Vec::new();
        list.push(parse_line("......"));
        list.push(parse_line("......"));
        list.push(parse_line("...#.."));
        list.push(parse_line("......"));
        list.push(parse_line("#....."));

        assert_eq!(2, num_trees(&list, 3, 2));
    }
}