use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/12/input.txt") {
        let commands:Vec<(char, i32)> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| parse_command(&r))
            .collect();

        let pos1 = follow_commands(&commands);
        println!("[{:?}, {:?}]", pos1.0, pos1.1);
        println!("Distance: {:?}", pos1.0.abs() + pos1.1.abs());
        let pos2 = follow_waypoint(&commands);
        println!("[{:?}, {:?}]", pos2.0, pos2.1);
        println!("Distance: {:?}", pos2.0.abs() + pos2.1.abs());
    }
}

pub fn parse_command(input: &String) -> (char, i32)
{
    return (input.chars().next().unwrap(), input[1..].parse::<i32>().unwrap());
}

pub fn follow_commands(commands: &Vec<(char, i32)>) -> (i32, i32)
{
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    for cmd in commands
    {
        match cmd.0
        {
            'N' => pos.1 = pos.1 + cmd.1,
            'S' => pos.1 = pos.1 - cmd.1,
            'E' => pos.0 = pos.0 + cmd.1,
            'W' => pos.0 = pos.0 - cmd.1,
            'F' => {
                pos.0 = pos.0 + (dir.0*cmd.1);
                pos.1 = pos.1 + (dir.1*cmd.1);
            },
            'L' => dir = rotate_vec(&dir, -cmd.1),
            'R' => dir = rotate_vec(&dir, cmd.1),
            _ => panic!("Unknown command: {}{}", cmd.0, cmd.1)
        }
    }
    return pos;
}


pub fn follow_waypoint(commands: &Vec<(char, i32)>) -> (i32, i32)
{
    let mut pos = (0, 0);
    let mut dir = (10, 1);
    for cmd in commands
    {
        match cmd.0
        {
            'N' => dir.1 = dir.1 + cmd.1,
            'S' => dir.1 = dir.1 - cmd.1,
            'E' => dir.0 = dir.0 + cmd.1,
            'W' => dir.0 = dir.0 - cmd.1,
            'F' => {
                pos.0 = pos.0 + (dir.0*cmd.1);
                pos.1 = pos.1 + (dir.1*cmd.1);
            },
            'L' => dir = rotate_vec(&dir, -cmd.1),
            'R' => dir = rotate_vec(&dir, cmd.1),
            _ => panic!("Unknown command: {}{}", cmd.0, cmd.1)
        }
    }
    return pos;
}

// rotates right
pub fn rotate_vec(vec: &(i32, i32), angle_deg: i32) -> (i32, i32)
{
    let sc = (-angle_deg as f32).to_radians().sin_cos();
    let fvec = (vec.0 as f32, vec.1 as f32);
    return (
        ((fvec.0 * sc.1) - (fvec.1 * sc.0)).round() as i32,
        ((fvec.0 * sc.0) + (fvec.1 * sc.1)).round() as i32
    );
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
    fn test_parse_command() 
    {
        assert_eq!(('F', 10), parse_command(&String::from("F10")));
        assert_eq!(('N', 3), parse_command(&String::from("N3")));
        assert_eq!(('S', 15), parse_command(&String::from("S15")));
        assert_eq!(('E', 30), parse_command(&String::from("E30")));
        assert_eq!(('W', 50), parse_command(&String::from("W50")));
        assert_eq!(('F', 7), parse_command(&String::from("F7")));
        assert_eq!(('R', 90), parse_command(&String::from("R90")));
        assert_eq!(('F', 11), parse_command(&String::from("F11")));
        assert_eq!(('L', 45), parse_command(&String::from("L45")));
    }

    #[test]
    fn test_follow_commands() 
    {
        let commands = vec![
            ('F', 10)
        ];
        assert_eq!((10, 0), follow_commands(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3)
        ];
        assert_eq!((10, 3), follow_commands(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7)
        ];
        assert_eq!((17, 3), follow_commands(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90)
        ];
        assert_eq!((17, 3), follow_commands(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90),
            ('F', 11)
        ];
        assert_eq!((17, -8), follow_commands(&commands));
    }

    #[test]
    fn test_follow_waypoint() 
    {
        let commands = vec![
            ('F', 10)
        ];
        assert_eq!((100, 10), follow_waypoint(&commands));
        let commands = vec![
            ('F', 5),
            ('R', 90),
            ('L', 90),
            ('F', 5)
        ];
        assert_eq!((100, 10), follow_waypoint(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3)
        ];
        assert_eq!((100, 10), follow_waypoint(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7)
        ];
        assert_eq!((170, 38), follow_waypoint(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90)
        ];
        assert_eq!((170, 38), follow_waypoint(&commands));
        let commands = vec![
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90),
            ('F', 11)
        ];
        assert_eq!((214, -72), follow_waypoint(&commands));
    }

    
    #[test]
    fn test_rotate_vec() 
    {
        assert_eq!((0, -1), rotate_vec(&(1, 0), 90));
        assert_eq!((0, -1), rotate_vec(&(1, 0), -270));
        assert_eq!((0, 1), rotate_vec(&(1, 0), -90));
        assert_eq!((0, 1), rotate_vec(&(1, 0), 270));
        assert_eq!((-1, 0), rotate_vec(&(1, 0), 180));
        assert_eq!((1, -10), rotate_vec(&(10, 1), 90));
    }
}