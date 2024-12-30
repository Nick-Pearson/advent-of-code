fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

struct Map<'a> {
    data: &'a str,
    line_len: usize,
}

impl<'a> Map<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            data,
            line_len: data.lines().next().unwrap().len() + 1,
        }
    }

    fn index_of(&self, c: char, start_idx: usize) -> Option<usize> {
        self.data[start_idx..].find(c).map(|idx| idx + start_idx)
    }

    fn coords(&self, idx: usize) -> (u8, u8) {
        ((idx % self.line_len) as u8, (idx / self.line_len) as u8)
    }

    fn idx(&self, coords: (u8, u8)) -> usize {
        (coords.1 as usize * self.line_len) + coords.0 as usize
    }

    fn get(&self, coords: (u8, u8)) -> Option<char> {
        let idx = self.idx(coords);
        self.data.chars().nth(idx)
    }
}

fn part_one(input: &str) -> i64 {
    let map = Map::new(input);
    let mut start_idx = 0;
    let mut total = 0;

    while let Some(idx) = map.index_of('0', start_idx) {
        let coords = map.coords(idx);

        let trailheads = find_unique_trail_heads(&map, coords);
        total += trailheads.len() as i64;

        start_idx = idx + 1;
    }
    total
}

fn part_two(input: &str) -> i64 {
    let map = Map::new(input);
    let mut start_idx = 0;
    let mut total = 0;

    while let Some(idx) = map.index_of('0', start_idx) {
        let coords = map.coords(idx);

        let trailheads = find_all_trail_heads(&map, coords);
        total += trailheads.len() as i64;

        start_idx = idx + 1;
    }
    total
}

fn find_unique_trail_heads(map: &Map, start: (u8, u8)) -> Vec<(u8, u8)> {
    let mut trail_heads = Vec::new();
    find_trail_heads(map, start, &mut |coords| {
        if trail_heads.contains(&coords) {
            return;
        }
        trail_heads.push(coords);
    });
    trail_heads
}

fn find_all_trail_heads(map: &Map, start: (u8, u8)) -> Vec<(u8, u8)> {
    let mut trail_heads = Vec::new();
    find_trail_heads(map, start, &mut |coords| {
        trail_heads.push(coords);
    });
    trail_heads
}

fn find_trail_heads<F>(map: &Map, start: (u8, u8), consumer: &mut F)
where
    F: FnMut((u8, u8)),
{
    let current = map.get(start).unwrap();
    if current == '9' {
        consumer(start);
        return;
    }

    try_to_explore_trail(map, (start.0 + 1, start.1), current, consumer);
    if start.0 > 0 {
        try_to_explore_trail(map, (start.0 - 1, start.1), current, consumer);
    }
    try_to_explore_trail(map, (start.0, start.1 + 1), current, consumer);
    if start.1 > 0 {
        try_to_explore_trail(map, (start.0, start.1 - 1), current, consumer);
    }
}

fn try_to_explore_trail<F>(map: &Map, possible: (u8, u8), current: char, consumer: &mut F)
where
    F: FnMut((u8, u8)),
{
    if let Some(next) = map.get(possible) {
        if next as u8 - 1 == current as u8 {
            find_trail_heads(map, possible, consumer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part_one(map), 36);
    }

    #[test]
    fn test_part_one2() {
        let map = "7770777
7771777
7772777
6543456
7777777
8777778
9777779";
        assert_eq!(part_one(map), 2);
    }

    #[test]
    fn test_part_one3() {
        let map = "0123
1234
8765
9876";
        assert_eq!(part_one(map), 1);
    }

    #[test]
    fn test_part_two() {
        let map = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part_two(map), 81);
    }
}
