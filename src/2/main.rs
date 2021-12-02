use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/2/input.txt") {
        let list = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let pos = get_pos(&list);
        println!("Position: {} * {} = {}", pos.0, pos.1, pos.0*pos.1);
        let pos2 = get_pos2(&list);
        println!("Position 2: {} * {} = {}", pos2.0, pos2.1, pos2.0*pos2.1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_pos(commands: &Vec<String>) -> (i32, i32)
{
    let mut pos = (0, 0);

    for cmd in commands
    {
        if cmd.starts_with("forward ")
        {
            pos.0 = pos.0 + cmd["forward ".len()..].parse::<i32>().unwrap();
        }
        else if cmd.starts_with("down ")
        {
            pos.1 = pos.1 + cmd["down ".len()..].parse::<i32>().unwrap();
        }
        else if cmd.starts_with("up ")
        {
            pos.1 = pos.1 - cmd["up ".len()..].parse::<i32>().unwrap();
        }
        else
        {
            panic!("invalid value: {:?}", cmd);
        }
    }
    return pos;
}

pub fn get_pos2(commands: &Vec<String>) -> (i32, i32)
{
    let mut pos = (0, 0);
    let mut aim = 0;

    for cmd in commands
    {
        if cmd.starts_with("forward ")
        {
            let x = cmd["forward ".len()..].parse::<i32>().unwrap();
            pos.0 = pos.0 + x;
            pos.1 = pos.1 + (aim * x);
        }
        else if cmd.starts_with("down ")
        {
            aim = aim + cmd["down ".len()..].parse::<i32>().unwrap();
        }
        else if cmd.starts_with("up ")
        {
            aim = aim - cmd["up ".len()..].parse::<i32>().unwrap();
        }
        else
        {
            panic!("invalid value: {:?}", cmd);
        }
    }
    return pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pos() 
    {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2")
        ];
        assert_eq!((15, 10), get_pos(&input));
    }

    #[test]
    fn test_get_pos2() 
    {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2")
        ];
        assert_eq!((15, 60), get_pos2(&input));
    }
}