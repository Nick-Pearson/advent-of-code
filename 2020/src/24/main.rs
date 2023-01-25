use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() 
{
    if let Ok(lines) = read_lines("src/24/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        println!("num tiles: {}", process(&input));
        println!("num tiles 2: {}", process2(&input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_commands(line: &String) -> Vec<String>
{
    let mut it = line.chars();
    let mut result = Vec::new();

    while let Some(c) = it.next()
    {
        match c
        {
            'e' | 'w' => result.push(String::from(c)),
            'n' | 's' => result.push(String::from(c) + &String::from(it.next().unwrap())),
            _ => panic!("invalid char {}", c)
        }
    }

    return result;
}

fn get_coords(commands: &Vec<String>) -> (i32, i32)
{
    let mut coords = (0, 0);

    for cmd in commands
    {
        if cmd == "e"
        {
            coords.0 = coords.0 + 2;
        }
        else if cmd == "w"
        {
            coords.0 = coords.0 - 2;
        }
        else if cmd == "ne"
        {
            coords.1 = coords.1 + 1;
            coords.0 = coords.0 + 1;
        }
        else if cmd == "nw"
        {
            coords.1 = coords.1 + 1;
            coords.0 = coords.0 - 1;
        }
        else if cmd == "se"
        {
            coords.1 = coords.1 - 1;
            coords.0 = coords.0 + 1;
        }
        else if cmd == "sw"
        {
            coords.1 = coords.1 - 1;
            coords.0 = coords.0 - 1;
        }
        else
        {
            panic!("invalid cmd {}", cmd);
        }
    }

    return coords;
}

fn get_initial_state(input: &Vec<String>) -> Vec<(i32, i32)>
{
    let mut flipped_tiles:Vec<(i32,i32)> = Vec::new();

    for line in input
    {
        let coords = get_coords(&get_commands(&line));
        
        let was_flipped = flipped_tiles.iter().position(|x| x.0 == coords.0 && x.1 == coords.1);
        if was_flipped.is_some()
        {
            flipped_tiles.swap_remove(was_flipped.unwrap());
        }
        else
        {
            flipped_tiles.push(coords);
        }
    }
    return flipped_tiles;
}

fn process(input: &Vec<String>) -> usize
{
    return get_initial_state(input).len();
}

fn get_count_adj(board: &Vec<Vec<u8>>, x: usize, y: usize) -> u8
{
    return board[y][x-2] + board[y+1][x-1] + board[y+1][x+1] +
        board[y][x+2] + board[y-1][x+1] + board[y-1][x-1];
}

const MAX:usize = 500;
const MID:i32 = MAX as i32 / 2;

fn process2(input: &Vec<String>) -> usize
{
    let initial_state = get_initial_state(input);
    let mut read_board = Vec::new();
    for _i in 0..MAX
    {
        read_board.push(vec![0; MAX]);
    }
    for tile in initial_state
    {
        read_board[(tile.1+MID) as usize][(tile.0+MID) as usize] = 1;
    }
    let mut write_board:Vec<Vec<u8>> = read_board.iter().cloned().collect();

    for _day in 0..100
    {
        for y in 2..MAX-2
        {
            for x in 2..MAX-2
            {
                let count = get_count_adj(&read_board, x, y);
                let mut result = read_board[y][x];
                if result == 1 && (count == 0 || count > 2)
                {
                    result = 0;
                }
                else if result == 0 && count == 2
                {
                    result = 1;
                }
                write_board[y][x] = result;
            }
        }

        for y in 2..MAX-2
        {
            for x in 2..MAX-2
            {
                read_board[y][x] = write_board[y][x];
            }
        }
    }

    return read_board.iter()
        .flatten()
        .filter(|v| **v == 1)
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() 
    {
        let input = vec![
            String::from("sesenwnenenewseeswwswswwnenewsewsw"),
            String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
            String::from("seswneswswsenwwnwse"),
            String::from("nwnwneseeswswnenewneswwnewseswneseene"),
            String::from("swweswneswnenwsewnwneneseenw"),
            String::from("eesenwseswswnenwswnwnwsewwnwsene"),
            String::from("sewnenenenesenwsewnenwwwse"),
            String::from("wenwwweseeeweswwwnwwe"),
            String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
            String::from("neeswseenwwswnwswswnw"),
            String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
            String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
            String::from("sweneswneswneneenwnewenewwneswswnese"),
            String::from("swwesenesewenwneswnwwneseswwne"),
            String::from("enesenwswwswneneswsenwnewswseenwsese"),
            String::from("wnwnesenesenenwwnenwsewesewsesesew"),
            String::from("nenewswnwewswnenesenwnesewesw"),
            String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
            String::from("neswnwewnwnwseenwseesewsenwsweewe"),
            String::from("wseweeenwnesenwwwswnew")
        ];
        assert_eq!(10, process(&input));
    }

    #[test]
    fn test_process2() 
    {
        let input = vec![
            String::from("sesenwnenenewseeswwswswwnenewsewsw"),
            String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
            String::from("seswneswswsenwwnwse"),
            String::from("nwnwneseeswswnenewneswwnewseswneseene"),
            String::from("swweswneswnenwsewnwneneseenw"),
            String::from("eesenwseswswnenwswnwnwsewwnwsene"),
            String::from("sewnenenenesenwsewnenwwwse"),
            String::from("wenwwweseeeweswwwnwwe"),
            String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
            String::from("neeswseenwwswnwswswnw"),
            String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
            String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
            String::from("sweneswneswneneenwnewenewwneswswnese"),
            String::from("swwesenesewenwneswnwwneseswwne"),
            String::from("enesenwswwswneneswsenwnewswseenwsese"),
            String::from("wnwnesenesenenwwnenwsewesewsesesew"),
            String::from("nenewswnwewswnenesenwnesewesw"),
            String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
            String::from("neswnwewnwnwseenwseesewsenwsweewe"),
            String::from("wseweeenwnesenwwwswnew")
        ];
        assert_eq!(2208, process2(&input));
    }

    #[test]
    fn test_get_commands() 
    {
        let expected = vec![
            String::from("se"),
            String::from("se"),
            String::from("nw"),
            String::from("ne"),
            String::from("ne"),
            String::from("ne"),
            String::from("w"),
            String::from("se"),
            String::from("e"),
            String::from("sw"),
            String::from("w"),
            String::from("sw"),
            String::from("sw"),
            String::from("w"),
            String::from("ne"),
            String::from("ne"),
            String::from("w"),
            String::from("se"),
            String::from("w"),
            String::from("sw")
        ];
        assert_eq!(expected, get_commands(&String::from("sesenwnenenewseeswwswswwnenewsewsw")));
    }
}