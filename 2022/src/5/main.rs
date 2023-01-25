use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/5/input.txt") {
        let strings = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let input = parse_input(&strings);

        println!("Part 1: {}", part_one(&input.0, &input.1));
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<Move>)
{
    let crates:Vec<Vec<char>> = Vec::new();
    let moves:Vec<Move> = Vec::new();
    return (vec![], vec![]);
}

#[derive(Debug)]
pub struct Move
{
    src : usize, 
    dst : usize,
    qty : usize
}
impl Move
{
    fn of(s: usize, d: usize, q: usize) -> Move
    {
        return Move{
            src : s, 
            dst : d,
            qty : q
        };
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one(crates: &Vec<Vec<char>>, moves: &Vec<Move>) -> String
{
    return crates.iter()
        .map(|x| x[x.len() - 1])
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let crates = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P']
        ];
        let moves = vec![
            Move::of(1, 0, 1),
            Move::of(0, 2, 3),
            Move::of(1, 0, 1),
            Move::of(0, 1, 2),
        ];
        assert_eq!("CMZ", part_one(&crates, &moves));
    }

}