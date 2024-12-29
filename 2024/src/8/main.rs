use std::{collections::HashMap, convert::TryInto, ops::Sub};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i16, i16);

impl Coord {
    fn new<T>(x: T, y: T) -> Self
    where
        i16: From<T>,
    {
        Self(i16::from(x), i16::from(y))
    }

    fn try_new<T: TryInto<i16>>(x: T, y: T) -> Result<Self, T::Error> {
        Ok(Self(x.try_into()?, y.try_into()?))
    }

    fn x(&self) -> i16 {
        self.0
    }

    fn y(&self) -> i16 {
        self.1
    }

    fn mag_manhatten(&self) -> i32 {
        self.x().abs() as i32 + self.y().abs() as i32
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Coord) -> Coord {
        Coord::new(self.x() - other.x(), self.y() - other.y())
    }
}

struct Map {
    size: (usize, usize),
    antenna: HashMap<char, Vec<Coord>>,
}

fn part_one(input: &str) -> usize {
    let map = parse_map(input);
    let mut result = 0;

    for x in 0..map.size.0 {
        for y in 0..map.size.1 {
            if is_valid_antinode(&map, Coord::try_new(x, y).unwrap(), true) {
                result += 1;
            }
        }
    }
    result
}

fn part_two(input: &str) -> usize {
    let map = parse_map(input);
    let mut result = 0;

    for x in 0..map.size.0 {
        for y in 0..map.size.1 {
            if is_valid_antinode(&map, Coord::try_new(x, y).unwrap(), false) {
                result += 1;
            }
        }
    }
    result
}

fn is_valid_antinode(map: &Map, coord: Coord, check_distances: bool) -> bool {
    for a in map.antenna.values() {
        if valid_antinode(a, coord, check_distances) {
            return true;
        }
    }
    false
}

fn valid_antinode(antenna: &[Coord], coord: Coord, check_distances: bool) -> bool {
    for i in 0..antenna.len() {
        let a_to_point = antenna[i] - coord;
        for j in i + 1..antenna.len() {
            let b_to_point = antenna[j] - coord;
            if check_distance_between_antennas(a_to_point, b_to_point, check_distances) {
                return true;
            }
        }
    }
    false
}

fn check_distance_between_antennas(vec_a: Coord, vec_b: Coord, check_distances: bool) -> bool {
    if !are_parallel(vec_a, vec_b) {
        return false;
    }
    let dist_a = vec_a.mag_manhatten();
    let dist_b = vec_b.mag_manhatten();
    !check_distances || dist_a == dist_b * 2 || dist_b == dist_a * 2
}

fn are_parallel(vec_a: Coord, vec_b: Coord) -> bool {
    vec_a.x() * vec_b.y() == vec_a.y() * vec_b.x()
}

fn parse_map(input: &str) -> Map {
    let mut antenna = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let entry = antenna.entry(c).or_insert(vec![]);
                entry.push(Coord::try_new(x, y).unwrap());
            }
        }
    }
    let size = (
        input.lines().next().map_or(0, |line| line.len()),
        input.lines().count(),
    );
    Map { size, antenna }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part_one(input), 14);
    }

    #[test]
    fn test_part_one_2() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_part_one_3() {
        let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";
        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn test_part_two() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part_two(input), 34);
    }
}
