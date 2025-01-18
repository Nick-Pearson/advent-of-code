use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input, 100));
    println!("Part two: {}", part_two(input, 100));
}

#[derive(Debug)]
struct Map {
    start: (i16, i16),
    end: (i16, i16),
    spaces: Vec<(i16, i16)>,
}

fn parse(input: &str) -> Map {
    let mut spaces = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x as i16, y as i16),
                'E' => end = (x as i16, y as i16),
                '.' => spaces.push((x as i16, y as i16)),
                _ => {}
            }
        }
    }
    Map { start, end, spaces }
}

fn part_one(input: &str, threshold: usize) -> usize {
    let map = parse(input);
    let from_start = dijstra(&map, map.start, map.end).expect("No route from start");
    let from_end = dijstra(&map, map.end, map.start).expect("No route from end");
    let distance = from_start[&map.end].distance;
    discover_cheats(&from_start, &from_end, 2, distance - threshold + 1)
}

fn part_two(input: &str, threshold: usize) -> usize {
    let map = parse(input);
    let from_start = dijstra(&map, map.start, map.end).expect("No route from start");
    let from_end = dijstra(&map, map.end, map.start).expect("No route from end");
    let distance = from_start[&map.end].distance;
    discover_cheats(&from_start, &from_end, 20, distance - threshold + 1)
}

fn generate_offsets(cheat_length: i16) -> Vec<(i16, i16)> {
    let mut offsets = Vec::new();
    for x in 0..=cheat_length {
        for y in 0..=cheat_length {
            if x + y < 2 || x + y > cheat_length {
                continue;
            }

            offsets.push((x, y));
            offsets.push((-x, y));
            offsets.push((x, -y));
            offsets.push((-x, -y));
        }
    }
    offsets.sort();
    offsets.dedup();
    offsets
}

fn discover_cheats(
    from_start: &HashMap<(i16, i16), PathNode>,
    from_end: &HashMap<(i16, i16), PathNode>,
    cheat_length: i16,
    max_distance: usize,
) -> usize {
    let mut cheats = HashSet::new();
    let offsets: Vec<(i16, i16)> = generate_offsets(cheat_length);
    for (location, node) in from_start.iter() {
        if node.distance == usize::MAX {
            continue;
        }

        for offset in &offsets {
            let neighbor = (location.0 + offset.0, location.1 + offset.1);
            if let Some(other_node) = from_end.get(&neighbor) {
                let duration_of_cheat = offset.0.abs() + offset.1.abs();
                let cheat_distance =
                    node.distance + other_node.distance + duration_of_cheat as usize;
                if cheat_distance < max_distance {
                    cheats.insert((location, neighbor));
                }
            }
        }
    }
    cheats.len()
}

struct PathNode {
    distance: usize,
}

impl PathNode {
    fn new() -> Self {
        PathNode {
            distance: usize::MAX - 30000,
        }
    }
}

fn dijstra(map: &Map, start: (i16, i16), end: (i16, i16)) -> Option<HashMap<(i16, i16), PathNode>> {
    let mut unvisited = all_locations(map);
    let mut nodes = all_nodes(&unvisited, start, end);

    loop {
        let closest_unvisited = unvisited.iter().min_by_key(|&x| nodes[x].distance);
        if closest_unvisited.is_none() {
            break;
        }

        let current = *closest_unvisited.unwrap();
        let current_distance = nodes[&current].distance;
        if current == end {
            if current_distance == usize::MAX {
                return None;
            }
            return Some(nodes);
        }

        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let neighbor = (current.0 + offset.0, current.1 + offset.1);
            let distance = current_distance.checked_add(1).unwrap_or(usize::MAX);
            if let Some(node) = nodes.get_mut(&neighbor) {
                if distance < node.distance {
                    node.distance = distance;
                }
            }
        }

        unvisited.remove(&current);
    }

    None
}

fn all_nodes(
    unvisited: &HashSet<(i16, i16)>,
    start: (i16, i16),
    end: (i16, i16),
) -> HashMap<(i16, i16), PathNode> {
    let mut nodes: HashMap<(i16, i16), PathNode> =
        unvisited.iter().map(|&x| (x, PathNode::new())).collect();
    nodes.insert(
        start,
        PathNode {
            distance: 0,
            ..PathNode::new()
        },
    );
    nodes.insert(end, PathNode::new());
    nodes
}

fn all_locations(map: &Map) -> HashSet<(i16, i16)> {
    let mut unvisited: HashSet<(i16, i16)> = map.spaces.iter().copied().collect();
    unvisited.insert(map.start);
    unvisited.insert(map.end);
    unvisited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        assert_eq!(part_one(input, 64), 1);
        assert_eq!(part_one(input, 40), 2);
        assert_eq!(part_one(input, 38), 3);
        assert_eq!(part_one(input, 36), 4);
        assert_eq!(part_one(input, 20), 5);
        assert_eq!(part_one(input, 12), 8);
        assert_eq!(part_one(input, 10), 10);
        assert_eq!(part_one(input, 8), 14);
        assert_eq!(part_one(input, 6), 16);
        assert_eq!(part_one(input, 4), 30);
        assert_eq!(part_one(input, 2), 44);
    }

    #[test]
    fn test_part_two() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        assert_eq!(part_two(input, 76), 3);
        assert_eq!(part_two(input, 74), 7);
        assert_eq!(part_two(input, 72), 29);
        assert_eq!(part_two(input, 70), 41);
        assert_eq!(part_two(input, 68), 55);
        assert_eq!(part_two(input, 66), 67);
        assert_eq!(part_two(input, 64), 86);
        assert_eq!(part_two(input, 62), 106);
        assert_eq!(part_two(input, 60), 129);
        assert_eq!(part_two(input, 58), 154);
        assert_eq!(part_two(input, 56), 193);
        assert_eq!(part_two(input, 54), 222);
        assert_eq!(part_two(input, 52), 253);
        assert_eq!(part_two(input, 50), 285);
    }
}
