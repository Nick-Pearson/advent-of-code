use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Vec2 {
    x: i16,
    y: i16,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Item {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
}

struct Map {
    items: HashMap<Vec2, Item>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

fn part_one(input: &str) -> usize {
    let (mut map, instructions) = parse(input);
    process_instructions(&mut map, &instructions);
    map.items
        .iter()
        .filter(|(_, item)| **item == Item::Box)
        .map(|(pos, _)| gps_coord(pos))
        .sum()
}

fn part_two(input: &str) -> usize {
    let (mut map, instructions) = parse(&scale_up_map(input));
    process_instructions(&mut map, &instructions);
    map.items
        .iter()
        .filter(|(_, item)| **item == Item::LeftBox)
        .map(|(pos, _)| gps_coord(pos))
        .sum()
}

fn gps_coord(pos: &Vec2) -> usize {
    pos.x as usize + (100 * pos.y as usize)
}

fn process_instructions(map: &mut Map, instructions: &[Instruction]) {
    for instruction in instructions {
        let robot_pos = map
            .items
            .iter()
            .find_map(|(pos, item)| {
                if *item == Item::Robot {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap()
            .clone();

        try_to_move_item(map, &robot_pos, instruction, false);

        // dbg_print_map(map);
    }
}

fn dbg_print_map(map: &Map) {
    let min_x = map.items.keys().map(|pos| pos.x).min().unwrap();
    let max_x = map.items.keys().map(|pos| pos.x).max().unwrap();
    let min_y = map.items.keys().map(|pos| pos.y).min().unwrap();
    let max_y = map.items.keys().map(|pos| pos.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Vec2 { x, y };
            let item = map.items.get(&pos);
            let c = match item {
                Some(Item::Wall) => '#',
                Some(Item::Box) => 'O',
                Some(Item::LeftBox) => '[',
                Some(Item::RightBox) => ']',
                Some(Item::Robot) => '@',
                None => '.',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn try_to_move_item(map: &mut Map, pos: &Vec2, instruction: &Instruction, dry_run: bool) -> bool {
    let new_pos = fun_name(pos, instruction);

    if let Some(item) = map.items.get(&new_pos) {
        if *item == Item::Wall {
            return false;
        }
        if (*item == Item::LeftBox || *item == Item::RightBox)
            && (*instruction == Instruction::Up || *instruction == Instruction::Down)
        {
            let linked_pos = match *item {
                Item::LeftBox => fun_name(&new_pos, &Instruction::Right),
                Item::RightBox => fun_name(&new_pos, &Instruction::Left),
                _ => panic!("Invalid instruction"),
            };
            let possible = try_to_move_item(map, &new_pos, instruction, true)
                && try_to_move_item(map, &linked_pos, instruction, true);
            if !possible {
                return false;
            }
            try_to_move_item(map, &new_pos, instruction, dry_run);
            try_to_move_item(map, &linked_pos, instruction, dry_run);
        } else if !try_to_move_item(map, &new_pos, instruction, dry_run) {
            return false;
        }
    }

    if !dry_run {
        let robot = map.items.remove(pos).unwrap();
        map.items.insert(new_pos, robot);
    }
    true
}

fn fun_name(pos: &Vec2, instruction: &Instruction) -> Vec2 {
    match instruction {
        Instruction::Left => Vec2 {
            x: pos.x - 1,
            y: pos.y,
        },
        Instruction::Right => Vec2 {
            x: pos.x + 1,
            y: pos.y,
        },
        Instruction::Up => Vec2 {
            x: pos.x,
            y: pos.y - 1,
        },
        Instruction::Down => Vec2 {
            x: pos.x,
            y: pos.y + 1,
        },
    }
}

fn scale_up_map(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '#' => output.push_str("##"),
            'O' => output.push_str("[]"),
            '.' => output.push_str(".."),
            '@' => output.push_str("@."),
            _ => output.push(c),
        }
    }
    output
}

fn parse(input: &str) -> (Map, Vec<Instruction>) {
    let mut split = input.split("\n\n");
    let map = parse_map(split.next().unwrap());
    let instructions = parse_instructions(split.next().unwrap());
    (map, instructions)
}

fn parse_map(input: &str) -> Map {
    let mut items = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = Vec2 {
                x: x as i16,
                y: y as i16,
            };
            let item = match c {
                '#' => Item::Wall,
                'O' => Item::Box,
                '[' => Item::LeftBox,
                ']' => Item::RightBox,
                '@' => Item::Robot,
                _ => return,
            };
            items.insert(pos, item);
        });
    });
    Map { items }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .filter(|&c| c == '^' || c == 'v' || c == '<' || c == '>')
        .map(|c| match c {
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part_one(input), 10092);
    }

    #[test]
    fn test_part_one2() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(part_one(input), 2028);
    }

    #[test]
    fn test_part_two() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part_two(input), 9021);
    }

    #[test]
    fn test_part_two2() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!(part_two(input), 618);
    }
}
