use std::{
    collections::{HashMap, HashSet},
    usize,
};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Map {
    start: (i16, i16),
    end: (i16, i16),
    spaces: Vec<(i16, i16)>,
}

fn part_one(input: &str) -> usize {
    let map = parse(input);
    shortest_route(&map).expect("no route found")
}

fn part_two(input: &str) -> usize {
    let map = parse(input);
    best_locations(&map)
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

struct PathNode {
    distance_north: usize,
    prev_north: Option<(i16, i16)>,
    distance_east: usize,
    prev_east: Option<(i16, i16)>,
    distance_south: usize,
    prev_south: Option<(i16, i16)>,
    distance_west: usize,
    prev_west: Option<(i16, i16)>,
}

impl PathNode {
    fn new() -> Self {
        PathNode {
            distance_north: usize::MAX - 3000,
            prev_north: None,
            distance_east: usize::MAX - 3000,
            prev_east: None,
            distance_south: usize::MAX - 3000,
            prev_south: None,
            distance_west: usize::MAX - 3000,
            prev_west: None,
        }
    }
}

fn best_locations(map: &Map) -> usize {
    let route = get_nodes_from(&dijstra(map).expect("no route found"), &map.end);
    // dbg_map(map, &route);
    route.len()
}

fn dbg_map(map: &Map, route: &HashSet<(i16, i16)>) {
    let max = map
        .spaces
        .iter()
        .fold((0, 0), |(ax, ay), (x, y)| (ax.max(*x), ay.max(*y)));
    for y in 0..=max.1 + 1 {
        for x in 0..=max.0 + 1 {
            if map.start == (x, y) {
                print!("S");
            } else if map.end == (x, y) {
                print!("E");
            } else if route.contains(&(x, y)) {
                print!("O");
            } else if map.spaces.contains(&(x, y)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!()
    }
}

fn get_nodes_from(
    nodes: &HashMap<(i16, i16), PathNode>,
    location: &(i16, i16),
) -> HashSet<(i16, i16)> {
    let mut visited = HashSet::new();
    let mut to_visit = vec![(*location, min_distance(&nodes[location]) + 1)];
    while let Some((current, prev_dist)) = to_visit.pop() {
        
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        let node = &nodes[&current];

        if node.distance_north + 1 == prev_dist || node.distance_north + 1001 == prev_dist {
            if let Some(prev) = node.prev_north {
                to_visit.push((prev, node.distance_north));
            }
        }
        if node.distance_south + 1 == prev_dist || node.distance_south + 1001 == prev_dist {
            if let Some(prev) = node.prev_south {
                to_visit.push((prev, node.distance_south));
            }
        }
        if node.distance_east + 1 == prev_dist || node.distance_east + 1001 == prev_dist {
            if let Some(prev) = node.prev_east {
                to_visit.push((prev, node.distance_east));
            }
        }
        if node.distance_west + 1 == prev_dist || node.distance_west + 1001 == prev_dist {
            if let Some(prev) = node.prev_west {
                to_visit.push((prev, node.distance_west));
            }
        }
    }
    visited
}

fn shortest_route(map: &Map) -> Option<usize> {
    dijstra(map).map(|x| min_distance(&x[&map.end]))
}

fn dijstra(map: &Map) -> Option<HashMap<(i16, i16), PathNode>> {
    let mut nodes = all_nodes(map, &all_locations(map));

    for _ in 0..100 {
        let mut unvisited = all_locations(map);
        let mut changed = false;
        loop {
            let closest_unvisited = unvisited.iter().min_by_key(|&x| min_distance(&nodes[x]));
            if closest_unvisited.is_none() {
                break;
            }

            let current = *closest_unvisited.unwrap();
            for dir in [Direction::North,
                Direction::South,
                Direction::East,
                Direction::West] {
                let next = match dir {
                    Direction::North => (current.0, current.1 - 1),
                    Direction::South => (current.0, current.1 + 1),
                    Direction::East => (current.0 + 1, current.1),
                    Direction::West => (current.0 - 1, current.1),
                };
                let lowest_cost = calculate_lowest_cost(&nodes[&current], dir);
                changed |= update_distance(&mut nodes, current, next, lowest_cost, dir);
            }

            unvisited.remove(&current);
        }

        if !changed {
            return Some(nodes);
        }
    }
    panic!("no answer found");
}

fn calculate_lowest_cost(node: &PathNode, dir: Direction) -> usize {
    match dir {
        Direction::North => node
            .distance_north.saturating_add(1)
            .min(node.distance_east.saturating_add(1001))
            .min(node.distance_west.saturating_add(1001))
            .min(node.distance_south.saturating_add(2001)),
        Direction::South => node
            .distance_north.saturating_add(2001)
            .min(node.distance_east.saturating_add(1001))
            .min(node.distance_west.saturating_add(1001))
            .min(node.distance_south.saturating_add(1)),
        Direction::East => node
            .distance_north.saturating_add(1001)
            .min(node.distance_east.saturating_add(1))
            .min(node.distance_west.saturating_add(2001))
            .min(node.distance_south.saturating_add(1001)),
        Direction::West => node
            .distance_north.saturating_add(1001)
            .min(node.distance_east.saturating_add(2001))
            .min(node.distance_west.saturating_add(1))
            .min(node.distance_south.saturating_add(1001)),
    }
}

fn min_distance(node: &PathNode) -> usize {
    node.distance_north
        .min(node.distance_south)
        .min(node.distance_east)
        .min(node.distance_west)
}

fn update_distance(
    nodes: &mut HashMap<(i16, i16), PathNode>,
    current: (i16, i16),
    location: (i16, i16),
    distance: usize,
    direction: Direction,
) -> bool {
    if nodes.contains_key(&location) {
        let node = nodes.get_mut(&location).unwrap();
        let current_distance = match direction {
            Direction::North => &mut node.distance_north,
            Direction::South => &mut node.distance_south,
            Direction::East => &mut node.distance_east,
            Direction::West => &mut node.distance_west,
        };
        if *current_distance > distance {
            *current_distance = distance;
            match direction {
                Direction::North => node.prev_north = Some(current),
                Direction::South => node.prev_south = Some(current),
                Direction::East => node.prev_east = Some(current),
                Direction::West => node.prev_west = Some(current),
            }
            return true;
        }
    }
    false
}

fn all_nodes(map: &Map, unvisited: &HashSet<(i16, i16)>) -> HashMap<(i16, i16), PathNode> {
    let mut nodes: HashMap<(i16, i16), PathNode> =
        unvisited.iter().map(|&x| (x, PathNode::new())).collect();
    nodes.insert(
        map.start,
        PathNode {
            distance_north: 1000,
            distance_east: 0,
            distance_south: 1000,
            distance_west: 2000,
            ..PathNode::new()
        },
    );
    nodes.insert(map.end, PathNode::new());
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part_one(input), 7036);
    }

    #[test]
    fn test_part_one2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part_one(input), 11048);
    }

    #[test]
    fn test_part_two() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part_two(input), 45);
    }

    #[test]
    fn test_part_two2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part_two(input), 64);
    }
}
