use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/13/input.txt") {
        let mut itr = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        let mut state = Vec::new();
        for line in &mut itr
        {
            if line == ""
            {
                break;
            }
            let mut s = line.split(",");
            state.push((s.next().unwrap().parse::<i32>().unwrap(), s.next().unwrap().parse::<i32>().unwrap()))
        }
        let folds = itr.map(|s| {
                let mut i = s[11..].split("=");
                return (i.next().unwrap().chars().nth(0).unwrap(), i.next().unwrap().parse::<i32>().unwrap());
            })
            .collect();

        println!("Part 1: {}", part_one(&state, &folds));
        let s = part_two(&state, &folds);
        print_state(&s);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fold(state: &mut Vec<(i32, i32)>, along: &(char, i32))
{
    if along.0 == 'x'
    {
        state.retain(|x| x.0 != along.1);
        for coord in state
        {
            if coord.0 > along.1
            {
                coord.0 = along.1 - (coord.0 - along.1);
            }
        }
    }
    else if along.0 == 'y'
    {
        state.retain(|x| x.1 != along.1);
        for coord in state
        {
            if coord.1 > along.1
            {
                coord.1 = along.1 - (coord.1 - along.1);
            }
        }
    }
}

fn part_one(state: &Vec<(i32, i32)>, folds: &Vec<(char, i32)>) -> usize
{
    let mut board = state.clone();
    fold(&mut board, &folds[0]);
    board.sort();
    board.dedup();
    return board.len();
}

fn part_two(state: &Vec<(i32, i32)>, folds: &Vec<(char, i32)>) -> Vec<(i32, i32)>
{
    let mut board = state.clone();
    for f in folds
    {
        fold(&mut board, f);
        board.sort();
        board.dedup();
    }
    return board;
}

fn print_state(state: &Vec<(i32, i32)>)
{
    println!("state: {:?}", state);
    let max_x = state.iter().map(|x| x.0).fold(0, |a,b| i32::max(a,b));
    let max_y = state.iter().map(|x| x.1).fold(0, |a,b| i32::max(a,b));

    for y in 0..=max_y
    {
        for x in 0..=max_x
        {
            if state.contains(&(x, y))
            {
                print!("#");
            }
            else
            {
                print!(".");
            }
        }
        println!("");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let state = vec![
            (6,10),
            (0,14),
            (9,10),
            (0,3),
            (10,4),
            (4,11),
            (6,0),
            (6,12),
            (4,1),
            (0,13),
            (10,12),
            (3,4),
            (3,0),
            (8,4),
            (1,10),
            (2,14),
            (8,10),
            (9,0)
        ];
        let folds = vec![
            ('y', 7),
            ('x', 5)
        ];
        assert_eq!(17, part_one(&state, &folds));
    }
}