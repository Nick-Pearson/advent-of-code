use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input, (101, 103)));
    println!("Part two: {}", part_two(input, (101, 103)));
}

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

fn part_one(input: &str, max_size: (usize, usize)) -> usize {
    let quadrants = generate_quadrants(
        parse(input)
            .iter()
            .map(|robot| solve_robot_position(robot, max_size, 100)),
        max_size,
    );
    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn generate_quadrants(
    iter: impl Iterator<Item = (usize, usize)>,
    max_size: (usize, usize),
) -> (usize, usize, usize, usize) {
    let half = (max_size.0 / 2, max_size.1 / 2);

    iter.filter(|pos| pos.0 != half.0 && pos.1 != half.1)
        .map(|(x, y)| {
            if x > half.0 {
                if y > half.1 {
                    (1, 0, 0, 0)
                } else {
                    (0, 1, 0, 0)
                }
            } else if y > half.1 {
                (0, 0, 1, 0)
            } else {
                (0, 0, 0, 1)
            }
        })
        .fold((0, 0, 0, 0), |acc, (x, y, z, w)| {
            (acc.0 + x, acc.1 + y, acc.2 + z, acc.3 + w)
        })
}

fn part_two(input: &str, max_size: (usize, usize)) -> usize {
    let robots = parse(input);
    for i in 0..100000 {
        let positions: HashSet<(usize, usize)> = robots
            .iter()
            .map(|robot| solve_robot_position(robot, max_size, i))
            .collect();

        if possible_solve(&positions, max_size) {
            // println!("Iteration: {}", i);
            // debug_print_tree(max_size, positions);
            return i as usize;
        }
    }
    panic!("No solution found");
}

fn debug_print_tree(max_size: (usize, usize), positions: HashSet<(usize, usize)>) {
    for y in 0..max_size.1 {
        for x in 0..max_size.0 {
            if positions.contains(&(x, y)) {
                print!("1");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn possible_solve(positions: &HashSet<(usize, usize)>, max_size: (usize, usize)) -> bool {
    let mut consecutive = 0;
    for y in 0..max_size.1 {
        for x in 0..max_size.0 {
            if positions.contains(&(x, y)) {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= 10 {
                return true;
            }
        }
        consecutive = 0;
    }
    false
}

fn solve_robot_position(
    robot: &Robot,
    max_size: (usize, usize),
    iterations: isize,
) -> (usize, usize) {
    (
        wrap_position(
            robot.position.0 as isize + (robot.velocity.0 * iterations),
            max_size.0,
        ),
        wrap_position(
            robot.position.1 as isize + (robot.velocity.1 * iterations),
            max_size.1,
        ),
    )
}

fn wrap_position(pos: isize, max: usize) -> usize {
    let pos = pos % max as isize;
    if pos < 0 {
        (max as isize + pos) as usize
    } else {
        pos as usize
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(parse_robot).collect()
}

fn parse_robot(line: &str) -> Robot {
    let mut parts = line.split_whitespace();
    let position = parts.next().unwrap()[2..]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let velocity = parts.next().unwrap()[2..]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Robot { position, velocity }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_robot_position() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
        };
        assert_eq!(solve_robot_position(&robot, (11, 7), 1), (4, 1));
        assert_eq!(solve_robot_position(&robot, (11, 7), 5), (1, 3));
    }

    #[test]
    fn test_part_one() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part_one(input, (11, 7)), 12);
    }

    #[test]
    fn test_part_two() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part_two(input, (11, 7)), 12);
    }
}
