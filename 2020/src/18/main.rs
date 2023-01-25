use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() 
{
    if let Ok(lines) = read_lines("src/18/input.txt") {
        let list:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let result1:i64 = list.iter()
            .map(|s| evaluate_exp(&s[..]))
            .sum();
        println!("Total of answers 1: {}", result1);

        let result2:i64 = list.iter()
            .map(|s| evaluate_exp2(&s[..]))
            .sum();
        println!("Total of answers 2: {}", result2);
    }
}

pub fn evaluate_exp(exp: &str) -> i64
{
    let mut idx = find_exp_part_end(0, exp);
    let mut lhs = get_value(&exp[0..idx]);
    
    while idx < exp.len()
    {
        let cmd = &exp[idx+1..idx+2];
        let second_exp_end = find_exp_part_end(idx+3, exp);
        let second_exp = &exp[idx+3..second_exp_end];

        let second_value = get_value(second_exp);
        
        match cmd
        {
            "*" => lhs = lhs * second_value,
            "+" => lhs = lhs + second_value,
            _ => panic!("invalid command '{}' in '{}'", cmd, exp)
        }
        idx = second_exp_end;
    }
    return lhs;
}

fn get_value(exp: &str) -> i64
{
    if &exp[0..1] == "("
    {
        return evaluate_exp(&exp[1..exp.len()-1]);
    }
    else
    {
        return exp.parse::<i64>().unwrap();
    }
}

pub fn evaluate_exp2(exp: &str) -> i64
{
    let mut idx = find_exp_part_end(0, exp);
    let mut stack = Vec::new();
    stack.push(get_value2(&exp[0..idx]));
    
    while idx < exp.len()
    {
        let cmd = &exp[idx+1..idx+2];
        let second_exp_end = find_exp_part_end(idx+3, exp);
        let second_exp = &exp[idx+3..second_exp_end];

        let second_value = get_value2(second_exp);
        
        match cmd
        {
            "*" => stack.push(second_value),
            "+" => {
                let lhs = stack.pop().unwrap();
                stack.push(lhs + second_value)
            },
            _ => panic!("invalid command '{}' in '{}'", cmd, exp)
        }
        idx = second_exp_end;
    }
    
    return stack.iter().product();
}

fn get_value2(exp: &str) -> i64
{
    if &exp[0..1] == "("
    {
        return evaluate_exp2(&exp[1..exp.len()-1]);
    }
    else
    {
        return exp.parse::<i64>().unwrap();
    }
}

pub fn find_exp_part_end(start: usize, exp: &str) -> usize
{
    let mut layers = 0;
    let mut i = start;
    for c in exp.chars().skip(start)
    {
        if c == '('
        {
            layers = layers + 1;
        }
        else if c == ')'
        {
            layers = layers - 1;
        }
        else if layers == 0
        {
            if c == '*' || c == '+' || c == ' '
            {
                return i;
            }
        }
        i = i + 1;
    }
    return i;
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
    fn test_evaluate_exp() 
    {
        assert_eq!(6, evaluate_exp(&"2 * 3"));
        assert_eq!(20, evaluate_exp(&"4 * 5"));
        assert_eq!(25, evaluate_exp(&"4 * 5 + 5"));
        assert_eq!(432, evaluate_exp(&"8 * 3 + 9 + 3 * 4 * 3"));
        assert_eq!(71, evaluate_exp(&"1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_evaluate_exp_brackets() 
    {
        assert_eq!(26, evaluate_exp(&"2 * 3 + (4 * 5)"));
        assert_eq!(437, evaluate_exp(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, evaluate_exp(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, evaluate_exp(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_evaluate_exp2() 
    {
        assert_eq!(6, evaluate_exp2(&"2 * 3"));
        assert_eq!(20, evaluate_exp2(&"4 * 5"));
        assert_eq!(40, evaluate_exp2(&"4 * 5 + 5"));
        assert_eq!(1440, evaluate_exp2(&"8 * 3 + 9 + 3 * 4 * 3"));
        assert_eq!(231, evaluate_exp2(&"1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_evaluate_exp2_brackets() 
    {
        assert_eq!(51, evaluate_exp2(&"1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, evaluate_exp2(&"2 * 3 + (4 * 5)"));
        assert_eq!(1445, evaluate_exp2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, evaluate_exp2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23340, evaluate_exp2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}