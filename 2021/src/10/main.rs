use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/10/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
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

fn part_one(lines: &Vec<String>) -> usize
{
    return lines.iter()
        .map(|l| first_illegal_character(l))
        .filter(|o| o.is_some())
        .map(|o| 
            return match o.unwrap()
            {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("invalid character {}", o.unwrap())
            }
        )
        .fold(0, |a,b| a + b);
}

fn first_illegal_character(line: &String) -> Option<char>
{
    let mut stack = Vec::new();
    for c in line.chars()
    {
        if c == '(' || c == '[' || c == '{' || c == '<'
        {
            stack.push(c);
        }
        else
        {
            let o = stack.pop().unwrap_or(' ');
            if !(o == '(' && c == ')') &&
                !(o == '[' && c == ']') &&
                !(o == '{' && c == '}') &&
                !(o == '<' && c == '>')
            {
                return Some(c);
            }
        }
    }
    return None;
}

fn part_two(lines: &Vec<String>) -> usize
{
    let mut items:Vec<usize> = lines.iter()
        .map(|l| find_replacement(l))
        .filter(|o| o.is_some())
        .map(|l| {
            l.unwrap().iter()
                .map(|o| 
                    return match o
                    {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("invalid character {}", o)
                    })
                .fold(0, |a,b| (a*5) + b)
        })
        .collect();

    items.sort();
    return items[items.len() / 2];
}

fn find_replacement(line: &String) -> Option<Vec<char>>
{
    let mut stack = Vec::new();
    for c in line.chars()
    {
        if c == '(' || c == '[' || c == '{' || c == '<'
        {
            stack.push(c);
        }
        else
        {
            let o = stack.pop().unwrap_or(' ');
            if !(o == '(' && c == ')') &&
                !(o == '[' && c == ']') &&
                !(o == '{' && c == '}') &&
                !(o == '<' && c == '>')
            {
                return None;
            }
        }
    }

    if stack.is_empty()
    {
        return None;
    }
    else
    {
        stack.reverse();
        return Some(stack);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]")

        ];
        assert_eq!(26397, part_one(&input));
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]")

        ];
        assert_eq!(288957, part_two(&input));
    }
}