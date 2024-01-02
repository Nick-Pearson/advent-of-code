use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn manhatten_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
struct Map {
    galaxies: Vec<Galaxy>,
}

impl Map {
    pub fn expand_universe(&mut self, expansion: usize) {
        let mut xs = HashSet::new();
        let mut ys = HashSet::new();
        let extra_rows = expansion - 1;

        for g in &self.galaxies {
            xs.insert(g.x);
            ys.insert(g.y);
        }

        for g in &mut self.galaxies {
            let populated_xs = xs.iter().filter(|x| **x < g.x).count();
            let num_x_expansions = g.x - populated_xs;
            
            let populated_ys = ys.iter().filter(|y| **y < g.y).count();
            let num_y_expansions = g.y - populated_ys;

            g.x = g.x + (num_x_expansions * extra_rows);
            g.y = g.y + (num_y_expansions * extra_rows);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = parse(input);
    let mut map2 = map.clone();
    map.expand_universe(2);
    map2.expand_universe(1000000);

    let part_one: usize = distance_between_pairs(&map);
    println!("Part one: {}", part_one);
    let part_two: usize = distance_between_pairs(&map2);
    println!("Part two: {}", part_two);
}

fn distance_between_pairs(map: &Map) -> usize {
    let mut result = 0;

    let num_galaxies = map.galaxies.len();
    for i in 0..num_galaxies {
        let g = &map.galaxies[i];
        for j in i + 1..num_galaxies {
            result = result + g.manhatten_distance(&map.galaxies[j]);
        }
    }

    result
}

fn parse(input: &str) -> Map {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_galaxies(line, y))
        .collect();
    Map { galaxies }
}

fn parse_galaxies(input: &str, y: usize) -> Vec<Galaxy> {
    input
        .chars()
        .enumerate()
        .filter(|c| c.1 == '#')
        .map(|c| Galaxy { x: c.0, y })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut map = parse(input);
        map.expand_universe(2);

        let part_one = distance_between_pairs(&map);
        assert_eq!(374, part_one);
    }

    #[test]
    fn test_part_two_ten() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut map = parse(input);
        map.expand_universe(10);

        let part_one = distance_between_pairs(&map);
        assert_eq!(1030, part_one);
    }

    #[test]
    fn test_part_two_hundred() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut map = parse(input);
        map.expand_universe(100);

        let part_one = distance_between_pairs(&map);
        assert_eq!(8410, part_one);
    }
}
