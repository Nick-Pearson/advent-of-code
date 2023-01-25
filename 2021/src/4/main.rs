use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/4/input.txt") {
        let mut list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());

        let input = list.next().unwrap()
            .split(',')
            .map(|r| r.parse::<i32>().unwrap())
            .collect();

        let mut boards = parse_boards(&list.skip(1).collect());

        let score = get_score(&input, &mut boards);
        println!("Score: {}*{} = {}", score.0, score.1, score.0*score.1);
        let score_last = last_winner(&input, &mut boards);
        println!("Last: {}*{} = {}", score_last.0, score_last.1, score_last.0*score_last.1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Board
{
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
}

impl Board 
{
    fn see_number(&mut self, num: i32)
    {
        for row in self.rows.iter_mut()
        {
            row.retain(|&x| x != num);
        }
        for col in self.cols.iter_mut()
        {
            col.retain(|&x| x != num);
        }
    }

    fn is_bingo(&self) -> bool 
    {
        return self.rows.iter().find(|x| x.is_empty()).is_some() ||
            self.cols.iter().find(|x| x.is_empty()).is_some();
    }

    fn score(&self) -> i32 
    {
        return self.rows.iter()
            .map(|row| row.iter().fold(0, |acc, x| acc + x))
            .fold(0, |acc, x| acc + x);
    }
}

fn derive_cols(rows: &Vec<Vec<i32>>) -> Vec<Vec<i32>>
{
    let mut cols = Vec::new();

    for i in 0..rows[0].len()
    {
        let mut col = Vec::new();
        for row in rows
        {
            col.push(row[i]);
        }
        cols.push(col);
    }
    return cols;
}

pub fn parse_boards(input: &Vec<String>) -> Vec<Board>
{
    let mut result = Vec::new();
    let mut rows = Vec::new();
    for line in input
    {
        if line == ""
        {
            let cols = derive_cols(&rows);
            result.push(Board {
                rows: rows,
                cols: cols
            });
            rows = Vec::new();
        }
        else
        {
            let row = line.split(' ')
                .filter(|r| r.len() > 0)
                .map(|r| r.parse::<i32>().unwrap())
                .collect();
            rows.push(row);
        }
    }
    let cols = derive_cols(&rows);
    result.push(Board {
        rows: rows,
        cols: cols
    });
    return result;
}

pub fn get_score(input: &Vec<i32>, boards: &mut Vec<Board>) -> (i32, i32)
{
    for number in input
    {
        for board in boards.iter_mut()
        {
            board.see_number(*number);
            if board.is_bingo()
            {
                return (board.score(), *number);
            }
        }

    }
    panic!("no winner");
}

pub fn last_winner(input: &Vec<i32>, boards: &mut Vec<Board>) -> (i32, i32)
{
    for number in input
    {
        let size = boards.len();
        for board in boards.iter_mut()
        {
            board.see_number(*number);
            if board.is_bingo() && size == 1
            {
                return (board.score(), *number);
            }
        }
        boards.retain(|x| !x.is_bingo());
    }
    panic!("no winner");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score() 
    {
        let input = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        let mut boards = parse_boards(&vec![
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7")
        ]);
        assert_eq!((188, 24), get_score(&input, &mut boards));
    }

    #[test]
    fn test_last_winner() 
    {
        let input = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        let mut boards = parse_boards(&vec![
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7")
        ]);
        assert_eq!((148, 13), last_winner(&input, &mut boards));
    }
}