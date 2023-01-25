use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

fn main() {
    if let Ok(lines) = read_lines("src/11/input.txt") {
        let initial_state:Vec<Vec<char>> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|s| s.chars().collect())
            .collect();

        let count1 = count_final_state(&initial_state);
        println!("Count Part 1: {}", count1);
        let count2 = count_final_state2(&initial_state);
        println!("Count Part 2: {}", count2);
    }
}

#[allow(dead_code)]
fn print_state(state: &Vec<Vec<char>>)
{
    for line in state
    {
        println!("{}", String::from_iter(line.iter()));
    }
}

pub fn count_final_state2(input: &Vec<Vec<char>>) -> i64
{
    let final_state = iterate_until_stable2(input);
    return final_state.iter()
        .map(|l| l.iter().filter(|c| **c == '#').count() as i64)
        .sum();
}

fn iterate_until_stable2(layout: &Vec<Vec<char>>) -> Vec<Vec<char>>
{
    let mut input = &mut layout.clone();
    let mut output = &mut layout.clone();
    let mut changes = 1;

    while changes != 0
    {
        let tmp = output;
        output = input;
        input = tmp;
        changes = iterate_state2(input, output);
    }
    return output.to_vec();
}

pub fn iterate_state2(input: &Vec<Vec<char>>, output: &mut Vec<Vec<char>>) -> i32
{
    let mut changes = 0;
    for y in 0..input.len()
    {
        let line = &input[y];

        for x in 0..line.len()
        {
            let ch = line[x];
            let count = count_occupied2(x as i32, y as i32, input);
            if ch == 'L' && count == 0
            {
                output[y][x] = '#';
                changes = changes + 1;
            }
            else if ch == '#' && count >= 5
            {
                output[y][x] = 'L';
                changes = changes + 1;
            }
            else
            {
                output[y][x] = ch;
            }
        }
    }
    return changes;
}

pub fn count_occupied2(x: i32, y: i32, input: &Vec<Vec<char>>) -> i32
{
    let mut count: i32 = 0;
    count = count + count_in_direction((x, y), (1, 0), input);
    count = count + count_in_direction((x, y), (1, -1), input);
    count = count + count_in_direction((x, y), (0, -1), input);
    count = count + count_in_direction((x, y), (-1, -1), input);
    count = count + count_in_direction((x, y), (-1, 0), input);
    count = count + count_in_direction((x, y), (-1, 1), input);
    count = count + count_in_direction((x, y), (0, 1), input);
    count = count + count_in_direction((x, y), (1, 1), input);
    return count;
}

pub fn count_in_direction(start: (i32, i32), direction: (i32, i32), input: &Vec<Vec<char>>) -> i32
{
    let mut pos = (start.0 + direction.0, start.1 + direction.1);
    loop
    {
        if pos.0 < 0 || pos.1 < 0
        {
            return 0;
        }

        let ch = input.get(pos.1 as usize).map_or(None, |l| l.get(pos.0 as usize));
        if ch.is_none()
        {
            return 0;
        }

        let c = *ch.unwrap();
        if c == 'L'
        {
            return 0;
        }
        else if c == '#'
        {
            return 1;
        }

        pos.0 = pos.0 + direction.0;
        pos.1 = pos.1 + direction.1;
    }
}

pub fn count_final_state(input: &Vec<Vec<char>>) -> i64
{
    let final_state = iterate_until_stable(input);
    return final_state.iter()
        .map(|l| l.iter().filter(|c| **c == '#').count() as i64)
        .sum();
}

fn iterate_until_stable(layout: &Vec<Vec<char>>) -> Vec<Vec<char>>
{
    let mut input = &mut layout.clone();
    let mut output = &mut layout.clone();
    let mut changes = 1;

    while changes != 0
    {
        let tmp = output;
        output = input;
        input = tmp;
        changes = iterate_state(input, output);
    }
    return output.to_vec();
}

pub fn iterate_state(input: &Vec<Vec<char>>, output: &mut Vec<Vec<char>>) -> i32
{
    let mut changes = 0;
    for y in 0..input.len()
    {
        let prev = if y > 0 { input.get(y - 1) } else { None };
        let line = &input[y];
        let next = input.get(y + 1);

        for x in 0..line.len()
        {
            let ch = line[x];
            let count = count_occupied(x, prev, line, next);
            if ch == 'L' && count == 0
            {
                output[y][x] = '#';
                changes = changes + 1;
            }
            else if ch == '#' && count >= 4
            {
                output[y][x] = 'L';
                changes = changes + 1;
            }
            else
            {
                output[y][x] = ch;
            }
        }
    }
    return changes;
}

pub fn count_occupied(x: usize, line_above: Option<&Vec<char>>, line: &Vec<char>, line_below: Option<&Vec<char>>) -> i32
{
    let mut count: i32 = 0;
    count = count + line_above.map_or(0, |l| count_line(x, l));
    count = count + line_below.map_or(0, |l| count_line(x, l));
    count = count + count_line(x, line);
    if line[x] == '#'
    {
        count = count - 1;
    }
    return count;
}

fn count_line(x: usize, line: &Vec<char>) -> i32
{
    let mut count: i32 = 0;
    if x > 0 
    {
        count = count + line.get(x - 1).map_or(0, |c| if *c == '#' { 1 } else { 0 });
    }
    count = count + (if line[x] == '#' { 1 } else { 0 });
    count = count + line.get(x + 1).map_or(0, |c| if *c == '#' { 1 } else { 0 });
    return count;
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
    fn test_initial_iteration() 
    {
        let input:Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect()
        ];
        let mut output = input.clone();
        iterate_state(&input, &mut output);

        let expected:Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect()
        ];
        assert_eq!(expected, output);
    }

    

    #[test]
    fn test_second_iteration() 
    {
        let input:Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect()
        ];
        let mut output = input.clone();
        iterate_state(&input, &mut output);

        let expected:Vec<Vec<char>> = vec![
            "#.LL.L#.##".chars().collect(),
            "#LLLLLL.L#".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "#LLL.LL.L#".chars().collect(),
            "#.LL.LL.LL".chars().collect(),
            "#.LLLL#.##".chars().collect(),
            "..L.L.....".chars().collect(),
            "#LLLLLLLL#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.#LLLL.##".chars().collect()
        ];
        assert_eq!(expected, output);
    }

    #[test]
    fn test_count_occupied() 
    {
        let line_one:Vec<char> = "L.LL.LL.LL".chars().collect();
        let line_two:Vec<char> = "..L.L.....".chars().collect();
        let line_three:Vec<char> = "#.#####.##".chars().collect();
        assert_eq!(0, count_occupied(0, None, &line_one, Some(&line_two)));
        assert_eq!(0, count_occupied(0, Some(&line_two), &line_three, Some(&line_two)));
        assert_eq!(8, count_occupied(3, Some(&line_three), &line_three, Some(&line_three)));
        assert_eq!(2, count_occupied(1, Some(&line_one), &line_three, Some(&line_one)));
        assert_eq!(5, count_occupied(9, Some(&line_three), &line_three, Some(&line_three)));
    }

    #[test]
    fn test_count_final_state() 
    {
        let input:Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect()
        ];
        assert_eq!(37, count_final_state(&input));
    }

    #[test]
    fn test_count_occupied2_full() 
    {
        let input:Vec<Vec<char>> = vec![
            ".......#.".chars().collect(),
            "...#.....".chars().collect(),
            ".#.......".chars().collect(),
            ".........".chars().collect(),
            "..#L....#".chars().collect(),
            "....#....".chars().collect(),
            ".........".chars().collect(),
            "#........".chars().collect(),
            "...#.....".chars().collect()
        ];
        assert_eq!(8, count_occupied2(3, 4, &input));
    }

    #[test]
    fn test_count_occupied2_empty1() 
    {
        let input:Vec<Vec<char>> = vec![
            ".............".chars().collect(),
            ".L.L.#.#.#.#.".chars().collect(),
            ".............".chars().collect()
        ];
        assert_eq!(0, count_occupied2(1, 1, &input));
    }

    #[test]
    fn test_count_occupied2_empty2() 
    {
        let input:Vec<Vec<char>> = vec![
            ".##.##.".chars().collect(),
            "#.#.#.#".chars().collect(),
            "##...##".chars().collect(),
            "...L...".chars().collect(),
            "##...##".chars().collect(),
            "#.#.#.#".chars().collect(),
            ".##.##.".chars().collect()
        ];
        assert_eq!(0, count_occupied2(3, 3, &input));
    }
}