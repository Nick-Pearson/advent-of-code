use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/18/input.txt") {
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

fn get_parts(input: &str) -> (&str, &str)
{
    let mut idx = 0;
    let mut level = 0;
    for c in input.chars().enumerate()
    {
        if c.1 == '['
        {
            level = level + 1;
        }
        else if c.1 == ']'
        {
            level = level - 1;
        }
        else if c.1 == ',' && level <= 1
        {
            idx = c.0;
        }
    }
    return (&input[1..idx], &input[idx+1..input.len()-1]);
}

fn magnetude(input: &str) -> usize
{
    if input.chars().nth(0).unwrap() != '['
    {
        return input.parse::<usize>().unwrap()
    }

    let (lhs, rhs) = get_parts(input);
    return (magnetude(lhs)*3) + (magnetude(rhs)*2);
}

fn find_prev(input: &str, idx: usize) -> usize
{
    let mut index = idx;
    while index > 0
    {
        let is_number = input.chars().nth(index)
            .map(|f| f.is_numeric())
            .unwrap_or(false);
        if !is_number
        {
            return index+1;
        }
        index = index - 1;
    }
    return idx;
}

fn find_next(input: &str, idx: usize) -> usize
{
    let mut index = idx;
    while index > 0
    {
        let is_number = input.chars().nth(index)
            .map(|f| f.is_numeric())
            .unwrap_or(false);
        if !is_number
        {
            return index;
        }
        index = index + 1;
    }
    return idx;
}

fn explode_half(input: &str, base_val: i32, right: bool) -> String
{
    let opt;
    if right
    {
        opt = input.rfind(|c:char| c.is_numeric())
            .map(|x| (find_prev(input, x), x+1));
    }
    else
    {
        opt = input.find(|c:char| c.is_numeric())
            .map(|x| (x, find_next(input, x)));
    }

    if let Some(idx) =  opt
    {
        let val = input[idx.0..idx.1].parse::<i32>().unwrap()+base_val;
        return format!("{}{}{}", &input[..idx.0], val, &input[idx.1..]);
    }
    return String::from(input);
}

fn explode(input: &String, index: usize) -> String
{
    let end_idx = input[index..].find(|c:char| c == ']').unwrap()+1+index;
    // println!("exploding {:?}", &input[index..end_idx]);
    let (lhs_str, rhs_str) = get_parts(&input[index..end_idx]);

    let lhs = lhs_str.parse::<i32>().unwrap();
    let left = explode_half(&input[..index], lhs, true);

    let rhs = rhs_str.parse::<i32>().unwrap();
    let right = explode_half(&input[end_idx..], rhs, false);

    return left + "0" + &right;
}

fn split(input: &String, index: usize) -> String
{
    // println!("splitting {:?}", &input[index..index+2]);
    let val = input[index..index+2].parse::<i32>().unwrap();
    let lhs = val / 2;
    let rhs = (val as f64 / 2.0).ceil() as i32;
    return format!("{}[{},{}]{}", &input[..index], lhs, rhs, &input[index+2..]);
}

fn reduce_once(input: &String) -> Option<String>
{
    let mut level = 0;
    let mut last_open = 0;
    for c in input.chars().enumerate()
    {
        if c.1 == '['
        {
            level = level + 1;
            last_open = c.0;
        }
        else if c.1 == ']'
        {
            level = level - 1;
        }
        else if level > 4 && c.1 == ',' &&
            input.chars().nth(c.0 - 1).unwrap().is_numeric() &&
            input.chars().nth(c.0 + 1).unwrap().is_numeric()
        {
            return Some(explode(input, last_open));
        }
    }

    let mut prev = '*';
    for c in input.chars().enumerate()
    {
        if c.1.is_numeric() && prev.is_numeric()
        {
            return Some(split(input, c.0-1));
        }
        prev = c.1;
    }

    return None;
}

fn reduce(input: &String) -> String
{
    let mut result:String = input.clone();
    for _i in 0..1000
    {
        // println!("reducing {:?}", &result);
        let r = reduce_once(&result);
        if r.is_none()
        {
            // println!("done reduction {:?}", &result);
            return result;
        }
        result = r.unwrap();
    }
    panic!("too many iterations");
}

fn part_one(input: &Vec<String>) -> usize
{
    let mut it = input.iter();
    let mut prev = it.next().unwrap().clone();

    for line in it
    {
        let val = format!("[{},{}]", prev, line);
        prev = reduce(&val);
    }
    return magnetude(&prev);
}

fn part_two(input: &Vec<String>) -> usize
{
    return input.iter()
        .map(|line| 
            input.iter().filter(|l| *l != line).map(|l| {
                let val = format!("[{},{}]", line, l);
                return magnetude(&reduce(&val));
            }).max().unwrap()
        )
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetude() 
    {
        assert_eq!(143, magnetude(&String::from("[[1,2],[[3,4],5]]")));
        assert_eq!(1384, magnetude(&String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
        assert_eq!(445, magnetude(&String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")));
        assert_eq!(791, magnetude(&String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]")));
        assert_eq!(1137, magnetude(&String::from("[[[[5,0],[7,4]],[5,5]],[6,6]]")));
        assert_eq!(3488, magnetude(&String::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));
    }

    #[test]
    fn test_reduce_once()
    {
        let step_one = String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let step_two = String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        let step_three = String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        let step_four = String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        let step_five = String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        let step_six = String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        assert_eq!(Some(step_two.clone()), reduce_once(&step_one));
        assert_eq!(Some(step_three.clone()), reduce_once(&step_two));
        assert_eq!(Some(step_four.clone()), reduce_once(&step_three));
        assert_eq!(Some(step_five.clone()), reduce_once(&step_four));
        assert_eq!(Some(step_six.clone()), reduce_once(&step_five));
        assert_eq!(None, reduce_once(&step_six));
    }

    #[test]
    fn test_reduce()
    {
        assert_eq!(String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), reduce(&String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")));
        assert_eq!(String::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"), reduce(&String::from("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")));
    }

    #[test]
    fn test_part_one()
    {
        let input = vec![
            String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
            String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
            String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
            String::from("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
            String::from("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
            String::from("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
            String::from("[[[[5,4],[7,7]],8],[[8,3],8]]"),
            String::from("[[9,3],[[9,9],[6,[4,9]]]]"),
            String::from("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
            String::from("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")
        ];
        assert_eq!(4140, part_one(&input));
    }

    #[test]
    fn test_part_two()
    {
        let input = vec![
            String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
            String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
            String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
            String::from("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
            String::from("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
            String::from("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
            String::from("[[[[5,4],[7,7]],8],[[8,3],8]]"),
            String::from("[[9,3],[[9,9],[6,[4,9]]]]"),
            String::from("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
            String::from("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")
        ];
        assert_eq!(3993, part_two(&input));
    }
}