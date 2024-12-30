use core::str;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

struct FlowerGroup {
    locations: HashSet<(usize, usize)>,
}

impl FlowerGroup {
    fn area(&self) -> usize {
        self.locations.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for loc in &self.locations {
            if loc.0 <= 0 || !self.has_flower((loc.0 - 1, loc.1)) {
                perimeter += 1;
            }
            if !self.has_flower((loc.0 + 1, loc.1)) {
                perimeter += 1;
            }
            if loc.1 <= 0 || !self.has_flower((loc.0, loc.1 - 1)) {
                perimeter += 1;
            }
            if !self.has_flower((loc.0, loc.1 + 1)) {
                perimeter += 1;
            }
        }
        perimeter
    }

    fn sides(&self) -> usize {
        let mut all_sides = HashSet::new();
        for loc in &self.locations {
            if loc.0 <= 0 || !self.has_flower((loc.0 - 1, loc.1)) {
                all_sides.insert((loc.0, loc.1, 2));
            }
            if !self.has_flower((loc.0 + 1, loc.1)) {
                all_sides.insert((loc.0, loc.1, 3));
            }
            if loc.1 <= 0 || !self.has_flower((loc.0, loc.1 - 1)) {
                all_sides.insert((loc.0, loc.1, 0));
            }
            if !self.has_flower((loc.0, loc.1 + 1)) {
                all_sides.insert((loc.0, loc.1, 1));
            }
        }

        let mut num_sides = 0;
        while !all_sides.is_empty() {
            let &first = all_sides.iter().next().unwrap();
            all_sides.remove(&first);

            Self::remove_adj_sides(first, &mut all_sides);

            num_sides += 1;
        }
        num_sides
    }

    fn remove_adj_sides(side: (usize, usize, i32), to_remove: &mut HashSet<(usize, usize, i32)>) {
        if side.2 == 0 || side.2 == 1 {
            if side.0 > 0 {
                let left = (side.0 - 1, side.1, side.2);
                if to_remove.remove(&left) {
                    Self::remove_adj_sides(left, to_remove);
                }
            }

            let right = (side.0 + 1, side.1, side.2);
            if to_remove.remove(&right) {
                Self::remove_adj_sides(right, to_remove);
            }
        } else if side.2 == 2 || side.2 == 3 {
            if side.1 > 0 {
                let up = (side.0, side.1 - 1, side.2);
                if to_remove.remove(&up) {
                    Self::remove_adj_sides(up, to_remove);
                }
            }

            let down = (side.0, side.1 + 1, side.2);
            if to_remove.remove(&down) {
                Self::remove_adj_sides(down, to_remove);
            }
        }
    }

    fn has_flower(&self, loc: (usize, usize)) -> bool {
        self.locations.contains(&loc)
    }
}

struct Map {
    map: HashMap<char, Vec<(usize, usize)>>,
}

fn part_one(input: &str) -> usize {
    let map = parse(input);
    let mut result = 0;
    for (_, flowers) in map.map {
        let groups = group(&flowers);
        for group in groups {
            let area = group.area();
            let perimeter = group.perimeter();
            result += area * perimeter;
        }
    }
    result
}

fn part_two(input: &str) -> usize {
    let map = parse(input);
    let mut result = 0;
    for (_, flowers) in map.map {
        let groups = group(&flowers);
        for group in groups {
            let area = group.area();
            let sides = group.sides();
            result += area * sides;
        }
    }
    result
}

fn parse(input: &str) -> Map {
    let mut map = HashMap::new();
    let line_len = input.lines().next().unwrap().len() + 1;

    for (i, c) in input.chars().enumerate() {
        if c.is_alphabetic() {
            let flowers: &mut Vec<(usize, usize)> = map.entry(c).or_default();
            flowers.push((i % line_len, i / line_len));
        }
    }
    Map { map }
}

fn group(flowers: &Vec<(usize, usize)>) -> Vec<FlowerGroup> {
    let mut remaining: HashSet<(usize, usize)> = HashSet::new();
    remaining.extend(flowers.iter());

    let mut result = Vec::new();

    while !remaining.is_empty() {
        let &first = remaining.iter().next().unwrap();
        remaining.remove(&first);

        let mut locations = HashSet::new();
        locations.insert(first);

        add_adjacent(first, &mut locations, &mut remaining);

        result.push(FlowerGroup { locations });
    }

    result
}

fn add_adjacent(
    loc: (usize, usize),
    locations: &mut HashSet<(usize, usize)>,
    remaining: &mut HashSet<(usize, usize)>,
) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in directions.iter() {
        let new_loc = (
            (loc.0 as isize + dir.0) as usize,
            (loc.1 as isize + dir.1) as usize,
        );
        if remaining.remove(&new_loc) {
            locations.insert(new_loc);
            add_adjacent(new_loc, locations, remaining);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part_one(map), 1930);
    }

    #[test]
    fn test_part_one2() {
        let map = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_one(map), 772);
    }

    #[test]
    fn test_part_two() {
        let map = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part_two(map), 1206);
    }

    #[test]
    fn test_part_two2() {
        let map = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part_two(map), 80);
    }

    #[test]
    fn test_part_two3() {
        let map = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_two(map), 436);
    }

    #[test]
    fn test_part_two4() {
        let map = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(part_two(map), 236);
    }

    #[test]
    fn test_part_two5() {
        let map = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(part_two(map), 368);
    }
}
