use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let map = parse_map(input);
    let start_coord = find_start(input);
    println!("Part one: {}", part_one(&map, start_coord));
    println!("Part two: {}", part_two(&map, start_coord));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32);

impl Coord {
    fn new<T: Into<i32>>(x: T, y: T) -> Self {
        Self(y.into() << 16 | x.into())
    }

    fn x(&self) -> i32 {
        self.0 & 0xFFFF
    }

    fn y(&self) -> i32 {
        self.0 >> 16
    }
}

trait Map {
    fn in_bounds(&self, coord: Coord) -> bool;

    fn contains_obstacle(&self, coord: &Coord) -> bool;
}

struct ArryBackedMap {
    size: (usize, usize),
    obstacles: HashSet<Coord>,
}

impl Map for ArryBackedMap {
    fn in_bounds(&self, coord: Coord) -> bool {
        let x = coord.x();
        let y = coord.y();
        x >= 0 && x < self.size.0 as i32 && y >= 0 && y < self.size.1 as i32
    }

    fn contains_obstacle(&self, coord: &Coord) -> bool {
        self.obstacles.contains(coord)
    }
}

struct ExtendedMap<'a> {
    base: &'a dyn Map,
    obstacle: Coord,
}

impl<'a> Map for ExtendedMap<'a> {
    fn in_bounds(&self, coord: Coord) -> bool {
        self.base.in_bounds(coord)
    }

    fn contains_obstacle(&self, coord: &Coord) -> bool {
        coord == &self.obstacle || self.base.contains_obstacle(coord)
    }
}

fn parse_map(input: &str) -> ArryBackedMap {
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Coord::new(x as i32, y as i32))
        })
        .collect();
    ArryBackedMap {
        size: (input.lines().next().unwrap().len(), input.lines().count()),
        obstacles,
    }
}

fn find_start(input: &str) -> Coord {
    input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find(|(_, c)| *c == '^')
                .map(|(x, _)| Coord::new(x as i32, y as i32))
        })
        .unwrap()
}

fn part_one(map: &dyn Map, start: Coord) -> usize {
    follow_route(map, start).into_iter().unique().count()
}

fn follow_route(map: &dyn Map, start: Coord) -> Vec<Coord> {
    let mut current = start;
    let mut route = vec![current];
    let mut direction = (0, -1);

    while map.in_bounds(current) {
        let next = Coord::new(current.x() + direction.0, current.y() + direction.1);
        if map.contains_obstacle(&next) {
            direction = rot_90(direction);
        } else {
            current = next;
            if map.in_bounds(next) {
                route.push(next)
            }
        }
    }

    route
}

fn part_two(map: &dyn Map, start: Coord) -> usize {
    let candidate_locations: Vec<Coord> = follow_route(map, start).into_iter().unique().collect();
    let mut count = 0;

    for loc in candidate_locations {
        let ext = ExtendedMap {
            base: map,
            obstacle: loc,
        };
        if is_loop(&ext, start) {
            count += 1;
        }
    }

    count
}

fn is_loop(map: &dyn Map, start: Coord) -> bool {
    let mut current = start;
    let mut route = vec![(current, 0, -1)];
    let mut direction = (0, -1);

    while map.in_bounds(current) {
        let next = Coord::new(current.x() + direction.0, current.y() + direction.1);
        if map.contains_obstacle(&next) {
            direction = rot_90(direction);
        } else {
            current = next;
            let key = (next, direction.0, direction.1);
            if route.contains(&key) {
                return true;
            }
            if map.in_bounds(next) {
                route.push(key)
            }
        }
    }

    false
}

fn rot_90(current: (i32, i32)) -> (i32, i32) {
    let x = -current.1;
    let y = current.0;
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot_90() {
        assert_eq!(rot_90((0, 1)), (-1, 0));
        assert_eq!(rot_90((-1, 0)), (0, -1));
        assert_eq!(rot_90((0, -1)), (1, 0));
        assert_eq!(rot_90((1, 0)), (0, 1));
    }

    #[test]
    fn test_part_one() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let map = parse_map(input);
        let start_coord = find_start(input);
        assert_eq!(part_one(&map, start_coord), 41);
    }

    #[test]
    fn test_part_two() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let map = parse_map(input);
        let start_coord = find_start(input);
        assert_eq!(part_two(&map, start_coord), 6);
    }
}
