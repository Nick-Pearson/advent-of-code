use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    rc::Rc,
};

use nom::{
    character::complete::{multispace0, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn inverse(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeKind {
    Vertical,
    Horizontal,
    BendNe,
    BendNw,
    BendSw,
    BendSe,
    Ground,
    Start,
}

impl PipeKind {
    pub fn directions(&self) -> Vec<Direction> {
        match self {
            PipeKind::Vertical => vec![Direction::North, Direction::South],
            PipeKind::Horizontal => vec![Direction::East, Direction::West],
            PipeKind::BendNe => vec![Direction::North, Direction::East],
            PipeKind::BendNw => vec![Direction::North, Direction::West],
            PipeKind::BendSw => vec![Direction::South, Direction::West],
            PipeKind::BendSe => vec![Direction::South, Direction::East],
            PipeKind::Ground => vec![],
            PipeKind::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    pub fn explore(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.y = self.y - 1,
            Direction::East => self.x = self.x + 1,
            Direction::South => self.y = self.y + 1,
            Direction::West => self.x = self.x - 1,
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    start_idx: usize,
    grid: Vec<PipeKind>,
}

impl Map {
    pub fn new(width: usize, grid: Vec<PipeKind>) -> Self {
        let start_idx = grid
            .iter()
            .enumerate()
            .find(|t| *t.1 == PipeKind::Start)
            .map(|e| e.0)
            .expect("failed to find start point");

        Map {
            width,
            start_idx,
            grid,
        }
    }

    pub fn start_coords(&self) -> Coords {
        Coords {
            x: self.start_idx % self.width,
            y: self.start_idx / self.width,
        }
    }

    pub fn pipe_at(&self, coords: &Coords) -> PipeKind {
        if coords.x >= self.width {
            return PipeKind::Ground;
        }

        let idx = coords.x + (coords.y * self.width);
        self.grid.get(idx).map_or(PipeKind::Ground, |x| *x)
    }
}

#[derive(Debug, Clone)]
struct Path {
    map: Rc<Map>,
    coords: Coords,
    explored: Vec<Coords>,
}

impl Path {
    pub fn new(map: Rc<Map>, coords: Coords) -> Self {
        Self {
            map,
            coords,
            explored: vec![coords],
        }
    }

    pub fn current(&self) -> PipeKind {
        self.map.pipe_at(&self.coords)
    }

    pub fn explore(&mut self, direction: &Direction) {
        self.coords.explore(direction);
        self.explored.push(self.coords);
    }

    pub fn can_explore(&self, direction: &Direction) -> bool {
        let mut new_coords = self.coords.clone();
        new_coords.explore(direction);

        if self.explored.contains(&new_coords) {
            return false;
        }

        let new_tile = self.map.pipe_at(&new_coords);
        new_tile
            .directions()
            .contains(&Direction::inverse(&direction))
    }

    pub fn enclosed_tiles(&self) -> usize {
        let (top_left, bottom_right) = self.bounding_box();
        let mut enclosed = 0;
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                let c = Coords { x, y };
                if self.is_enclosed(&c, &(top_left, bottom_right)) {
                    enclosed = enclosed + 1;
                }
            }
        }
        enclosed
    }

    fn is_enclosed(&self, coords: &Coords, bounding_box: &(Coords, Coords)) -> bool {
        if self.explored.contains(coords) {
            return false;
        }

        let mut to_x = Vec::new();
        for x in bounding_box.0.x..coords.x {
            let test = Coords { x, y: coords.y };
            if self.explored.contains(&test) {
                to_x.push(self.map.pipe_at(&test));
            } else {
                to_x.push(PipeKind::Ground)
            }
        }

        let mut crosses = 0;
        let mut blocked_north = false;
        let mut blocked_south = false;
        for p in to_x {
            match p {
                PipeKind::Vertical => {
                    crosses = crosses + 1;
                }
                PipeKind::Horizontal => {}
                // todo: automate working out that Start is a NEBend tile
                PipeKind::Start | PipeKind::BendNe | PipeKind::BendNw => {
                    if blocked_south {
                        crosses = crosses + 1;
                        blocked_south = false
                    } else {
                        blocked_north = !blocked_north;
                    }
                }
                PipeKind::BendSw | PipeKind::BendSe => {
                    if blocked_north {
                        crosses = crosses + 1;
                        blocked_north = false
                    } else {
                        blocked_south = !blocked_south;
                    }
                }
                PipeKind::Ground => {}
            }
        }
        crosses % 2 == 1
    }

    fn bounding_box(&self) -> (Coords, Coords) {
        let mut top_left = Coords {
            x: usize::MAX,
            y: usize::MAX,
        };
        let mut bottom_right = Coords { x: 0, y: 0 };

        for p in &self.explored {
            top_left.x = top_left.x.min(p.x);
            top_left.y = top_left.y.min(p.y);

            bottom_right.x = bottom_right.x.max(p.x);
            bottom_right.y = bottom_right.y.max(p.y);
        }

        (top_left, bottom_right)
    }
}

struct Search {
    paths: Vec<Path>,
}

impl Search {
    pub fn new(map: Rc<Map>, start: Coords) -> Self {
        let paths = vec![Path::new(map, start)];
        Self { paths }
    }

    pub fn find_furthest_point(&self) -> (Coords, usize) {
        let mut point_distance = HashMap::new();
        for path in &self.paths {
            path.explored.iter().enumerate().for_each(|(dist, point)| {
                match point_distance.entry(*point) {
                    Occupied(mut e) => {
                        if *e.get() > dist {
                            *e.get_mut() = dist;
                        }
                    }
                    Vacant(e) => {
                        e.insert(dist);
                    }
                }
            });
        }

        let result = point_distance.iter().max_by_key(|p| p.1).unwrap();
        (*result.0, *result.1)
    }

    pub fn longest_path(&self) -> &Path {
        self.paths.iter().max_by_key(|p| p.explored.len()).unwrap()
    }

    pub fn explore(&mut self) -> usize {
        let mut explored = 0;
        let mut new_paths = Vec::new();

        for path in &mut self.paths {
            let directions: Vec<Direction> = path
                .current()
                .directions()
                .into_iter()
                .filter(|d| path.can_explore(&d))
                .collect();

            for d in directions.iter().skip(1) {
                let mut cloned = path.clone();
                cloned.explore(d);
                new_paths.push(cloned);
            }

            directions.first().map(|d| path.explore(d));
            explored = explored + directions.len();
        }
        new_paths.into_iter().for_each(|p| self.paths.push(p));
        explored
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, map) = parse_map(input).unwrap();
    let start = map.start_coords();
    let map = Rc::new(map);

    let mut search = Search::new(map, start);
    while search.explore() != 0 {}
    let (coords, distance) = search.find_furthest_point();
    println!("Part one: {} ({}, {})", distance, coords.x, coords.y);
    let longest = search.longest_path();
    let part_two = longest.enclosed_tiles();
    println!(
        "Part two: {} (in path {} long)",
        part_two,
        longest.explored.len()
    );
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let width = input.lines().take(1).next().map_or(0, |s| s.len());
    let build_map = move |grid| Map::new(width, grid);

    map(
        many1(map(
            terminated(
                one_of(['.', '|', '-', 'L', 'J', '7', 'F', 'S'].as_ref()),
                multispace0,
            ),
            parse_pipe_kind,
        )),
        build_map,
    )(input)
}

fn parse_pipe_kind(c: char) -> PipeKind {
    match c {
        '.' => PipeKind::Ground,
        '|' => PipeKind::Vertical,
        '-' => PipeKind::Horizontal,
        'L' => PipeKind::BendNe,
        'J' => PipeKind::BendNw,
        '7' => PipeKind::BendSw,
        'F' => PipeKind::BendSe,
        'S' => PipeKind::Start,
        _ => panic!("failed to parse {}", c),
    }
}
