use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;

fn main() {
    if let Ok(lines) = read_lines("src/5/input.txt") {
        let heap: BinaryHeap<_> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|line| find_seat(&line))
            .map(|seat| seat_id(seat))
            .collect();

        println!("Max: {}", heap.peek().unwrap());

        let my_seat_id = find_missing_seat(&heap.into_sorted_vec());
        println!("My Seat ID: {}", my_seat_id);
        let col = my_seat_id % 8;
        let row = (my_seat_id - col) / 8;
        println!("Col: {}   Row: {}", col, row);
    }
}

pub fn find_missing_seat(input: &Vec<u32>) -> u32
{
    let mut iter = input.iter();
    let mut last = iter.next().unwrap();
    for seat_id in iter
    {
        if (seat_id - last) > 1
        {
            return seat_id - 1;
        }
        last = seat_id;
    }
    return 0;
}

pub fn find_seat(input: &str) -> (u32, u32)
{
    return (parse_bsp(&input[0..7], 'B'), parse_bsp(&input[7..10], 'R'));
}

pub fn parse_bsp(input: &str, upper_char: char) -> u32
{
    let mut val = 0;
    for c in input.chars()
    {
        val = val << 1;
        if c == upper_char
        {
            val = val + 1;
        }
    }
    return val;
}

pub fn seat_id(seat: (u32, u32)) -> u32
{
    return (seat.0 * 8) + seat.1;
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
    fn test_find_seat() 
    {
        assert_eq!((70, 7), find_seat("BFFFBBFRRR"));
        assert_eq!((14, 7), find_seat("FFFBBBFRRR"));
        assert_eq!((102, 4), find_seat("BBFFBBFRLL"));
    }

    #[test]
    fn test_parse_bsp() 
    {
        assert_eq!(1, parse_bsp("R", 'R'));
        assert_eq!(0, parse_bsp("F", 'R'));
        assert_eq!(70, parse_bsp("BFFFBBF", 'B'));
        assert_eq!(14, parse_bsp("FFFBBBF", 'B'));
        assert_eq!(102, parse_bsp("BBFFBBF", 'B'));
        assert_eq!(7, parse_bsp("RRR", 'R'));
        assert_eq!(4, parse_bsp("RLL", 'R'));
    }

    #[test]
    fn test_seat_id() 
    {
        assert_eq!(357, seat_id((44, 5)));
        assert_eq!(567, seat_id((70, 7)));
        assert_eq!(119, seat_id((14, 7)));
        assert_eq!(820, seat_id((102, 4)));
    }
}