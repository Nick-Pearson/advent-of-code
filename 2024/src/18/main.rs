use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input, 1024));
    let two = part_two(input);
    println!("Part two: {},{}", two.0, two.1);
}

fn part_one(input: &str, num_bytes: usize) -> i64 {
    let map = parse(input);
    dijstra(&map, num_bytes).unwrap()
}

fn part_two(input: &str) -> (i64, i64) {
    let map = parse(input);
    binary_search(&map, 0, map.corruptions.len() - 1)
}

fn binary_search(map: &Map, low: usize, high: usize) -> (i64, i64) {
    if low == high {
        return map.corruptions[low - 1];
    }

    let mid = (low + high) / 2;
    let result = dijstra(map, mid);
    if result.is_none() {
        binary_search(map, low, mid)
    } else {
        binary_search(map, mid + 1, high)
    }
}

struct Map {
    max: (i64, i64),
    corruptions: Vec<(i64, i64)>,
}

fn parse(input: &str) -> Map {
    let corruptions: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.split(',').collect_tuple().unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let max = corruptions
        .iter()
        .fold((0, 0), |acc, (x, y)| (acc.0.max(*x), acc.1.max(*y)));
    Map { max, corruptions }
}

struct PathNode {
    distance: i64,
}

impl PathNode {
    fn new() -> PathNode {
        PathNode { distance: i64::MAX }
    }
}

fn dijstra(map: &Map, num_bytes: usize) -> Option<i64> {
    let mut unvisited = all_locations(&map.max, &map.corruptions[..num_bytes]);
    let mut nodes = all_nodes(&unvisited);
    nodes.get_mut(&(0, 0)).unwrap().distance = 0;

    loop {
        let closest_unvisited = unvisited.iter().min_by_key(|&x| &nodes[x].distance);
        closest_unvisited?;

        let current = *closest_unvisited.unwrap();
        let current_distance = nodes[&current].distance;
        if current == map.max {
            if current_distance == i64::MAX {
                return None;
            }
            return Some(current_distance);
        }

        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let neighbor = (current.0 + offset.0, current.1 + offset.1);
            let distance = current_distance.checked_add(1).unwrap_or(i64::MAX);
            if let Some(node) = nodes.get_mut(&neighbor) {
                if distance < node.distance {
                    node.distance = distance;
                }
            }
        }

        unvisited.remove(&current);
    }
}

fn all_locations(max: &(i64, i64), corrupted: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut locations = HashSet::new();
    for x in 0..=max.0 {
        for y in 0..=max.1 {
            if !corrupted.contains(&(x, y)) {
                locations.insert((x, y));
            }
        }
    }
    locations
}

fn all_nodes(locations: &HashSet<(i64, i64)>) -> HashMap<(i64, i64), PathNode> {
    locations
        .iter()
        .map(|&loc| (loc, PathNode::new()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part_one(input, 12), 22);
    }

    #[test]
    fn test_part_two() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part_two(input), (6, 1));
    }
}
