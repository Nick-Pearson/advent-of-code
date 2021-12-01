use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() 
{
    if let Ok(lines) = read_lines("src/20/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let tiles:Vec<ImageTile> = parse_images(&input);
        let corners = find_corners(&tiles);
        println!("Corners: {:?}", corners);
        println!("Result: {}", corners.iter().map(|v| *v as i64).product::<i64>());
        let image = construct_image(&tiles);
        println!("Image:");
        for line in image
        {
            println!("{}", line);
        }
    }
}


#[derive(Debug, Clone)]
pub struct ImageTile {
    id: i32,
    data: Vec<String>,
    edges: Vec<i32>,
    adj_tids: Vec<i32>
}

impl ImageTile
{
    fn new(id: i32, data: &Vec<String>) -> ImageTile
    {
        let edges = find_edges(data);
        return ImageTile{
            id: id,
            data: data.to_vec(),
            edges: edges,
            adj_tids: vec![0; 4],
        };
    }

    pub fn debug_print(&self)
    {
        println!("Tile {}:", self.id);
        println!("Edges={:?}", self.edges);
        println!("Adj={:?}", self.adj_tids);
        for line in &self.data
        {
            println!("{}", line)
        }
    }

    pub fn turn(&mut self, turns: usize)
    {
        println!("{}: turning {}", self.id, turns);
        match turns
        {
            0 => return,
            1 => {
                let mut d = Vec::new();
                for _i in 0..self.data.len()
                {
                    d.push(String::new());
                }
                for line in self.data.iter().rev()
                {
                    let mut i = 0;
                    for c in line.chars()
                    {
                        d[i].push(c);
                        i = i + 1;
                    }
                }
                self.data = d;
            },
            2 => self.data = self.data.iter().rev().map(|s| s.chars().rev().collect::<String>()).collect(),
            3 => {
                self.turn(1);
                self.turn(2);
                return;
            }
            _ => panic!("unsupported turn {}", turns)
        }

        for _i in 0..turns
        {
            let tmp = self.edges.pop();
            self.edges.insert(0, tmp.unwrap());
            let tmp = self.adj_tids.pop();
            self.adj_tids.insert(0, tmp.unwrap());
        }
    }

    pub fn flip_y(&mut self)
    {
        println!("{}: flipping around Y", self.id);
        let tmp = self.edges[1];
        self.edges[1] = self.edges[3];
        self.edges[3] = tmp;
        let tmp = self.adj_tids[1];
        self.adj_tids[1] = self.adj_tids[3];
        self.adj_tids[3] = tmp;

        self.edges[0] = compliment_edge(self.edges[0]);
        self.edges[2] = compliment_edge(self.edges[2]);
        self.data = self.data.iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();
    }

    pub fn flip_x(&mut self)
    {
        println!("{}: flipping around X", self.id);

        let tmp = self.edges[0];
        self.edges[0] = self.edges[2];
        self.edges[2] = tmp;
        let tmp = self.adj_tids[0];
        self.adj_tids[0] = self.adj_tids[2];
        self.adj_tids[2] = tmp;

        self.edges[1] = compliment_edge(self.edges[1]);
        self.edges[3] = compliment_edge(self.edges[3]);
        self.data = self.data.iter()
            .rev()
            .map(|s| s.to_string())
            .collect();
    }
}

pub fn find_edges(data: &Vec<String>) -> Vec<i32>
{
    let mut result = Vec::new();
    let mut l = 0;
    let mut r = 0;
    let len = data.len();
    for i in 0..len
    {
        let chr = data[i].chars().skip(len - 1).next().unwrap();
        if chr == '#'
        {
            r = r + (1 << i);
        }
        let chl = data[len - 1 - i].chars().next().unwrap();
        if chl == '#'
        {
            l = l + (1 << i);
        }
    }
    result.push(data[0].chars().enumerate().filter(|c| c.1 == '#').map(|c| 1 << c.0).sum());
    result.push(r);
    result.push(data[len - 1].chars().enumerate().filter(|c| c.1 == '#').map(|c| 1 << (len - 1 - c.0)).sum());
    result.push(l);
    return result;
}

pub fn compliment_edge(input: i32) -> i32
{
    let mut r = 0;
    for i in 0..10
    {
        if input & (1 << i) > 0
        {
            r = r + (1 << (9-i));
        }
    }
    return r;
}

pub fn find_corners(tiles: &Vec<ImageTile>) -> Vec<i32>
{
    return tiles.iter()
        .filter(|t| t.adj_tids.iter().filter(|v| **v != 0).count() == 2)
        .map(|t| t.id)
        .collect();
}

pub fn construct_image(tiles: &Vec<ImageTile>) -> Vec<String>
{
    let corners = find_corners(&tiles);
    println!("corners: {:?}", corners);
    
    // place the first corner in top left
    let mut top_left_corner = tiles.iter().find(|t| t.id == corners[0]).unwrap().clone();
    top_left_corner.turn(find_top_left_turns(&top_left_corner.adj_tids));

    // first vertical
    let mut image = Vec::new();
    let mut result = construct_vertical(&top_left_corner, tiles, &vec![0; 100]);
    image.append(&mut result.0);

    // horizontals
    let mut last_tids = result.1;
    let mut edge_to_match = top_left_corner.edges[1];
    let mut opt_top_horizontal_tile = tiles.iter().find(|t| t.id == top_left_corner.adj_tids[1]);
    let mut i = 0;
    while opt_top_horizontal_tile.is_some() && i < 20
    {
        println!("================================================================");
        println!("Last TIds: {:?}", last_tids);
        let mut top_horizontal_tile = opt_top_horizontal_tile.unwrap().clone();
        transform_to_match(edge_to_match, 3, &mut top_horizontal_tile);
        if top_horizontal_tile.adj_tids[0] != 0
        {
            top_horizontal_tile.flip_x();
        }

        let result = construct_vertical(&top_horizontal_tile, tiles, &last_tids);
        for i in 0..result.0.len()
        {
            image[i].push_str(&result.0[i]);
        }

        last_tids = result.1;
        opt_top_horizontal_tile = tiles.iter().find(|t| t.id == top_horizontal_tile.adj_tids[1]);
        edge_to_match = top_horizontal_tile.edges[1];
        i = i + 1;
    }
    if i == 20
    {
        panic!("ran out of iterations");
    }

    return image;
}

fn construct_vertical(initial_tile: &ImageTile, tiles: &Vec<ImageTile>, lhs_tiles: &Vec<i32>) -> (Vec<String>, Vec<i32>)
{
    let mut image = Vec::new();
    let mut tids = Vec::new();
    initial_tile.debug_print();
    for i in 1..9
    {
        image.push(initial_tile.data[i][1..9].to_string());
    }
    tids.push(initial_tile.id);

    let mut edge_to_match = initial_tile.edges[2];
    let mut opt_next_tile = tiles.iter().find(|t| t.id == initial_tile.adj_tids[2]);
    let mut i = 0;
    while opt_next_tile.is_some() && i < 20
    {
        let mut next_tile = opt_next_tile.unwrap().clone();
        next_tile.debug_print();
        transform_to_match(edge_to_match, 0, &mut next_tile);
        if next_tile.adj_tids[3] != lhs_tiles[i+1]
        {
            next_tile.flip_y();
        }
        next_tile.debug_print();

        for i in 1..9
        {
            image.push(next_tile.data[i][1..9].to_string());
        }

        tids.push(next_tile.id);
        opt_next_tile = tiles.iter().find(|t| t.id == next_tile.adj_tids[2]);
        edge_to_match = next_tile.edges[2];
        i = i + 1;
    }
    if i == 20
    {
        panic!("ran out of iterations");
    }
    return (image, tids);
}

// transforms a tile to match an edge at a direction
pub fn transform_to_match(edge: i32, direction: usize, tile: &mut ImageTile)
{
    println!("{}: Edges = {:?} finding {} or {} @ {}", tile.id, tile.edges, edge, compliment_edge(edge), direction);
    let idx = tile.edges.iter()
        .enumerate()
        .find(|e| *e.1 == edge)
        .map(|e| e.0);
    if idx.is_some()
    {
        let turns = get_turns(idx.unwrap(), direction);
        tile.turn(turns);
    }
    else
    {
        let comp_edge = compliment_edge(edge);
        let comp_idx = tile.edges.iter()
            .enumerate()
            .find(|e| *e.1 == comp_edge)
            .map(|e| e.0)
            .unwrap();
        let turns = get_turns(comp_idx, direction);
        tile.turn(turns);
    }
    if direction % 2 == 0
    {
        tile.flip_y();
    }
    else
    {
        tile.flip_x();
    }
}

fn get_turns(idx: usize, direction: usize) -> usize
{
    let result:i32 = direction as i32 - idx as i32;
    if result < 0
    {
        return (result + 4) as usize;
    }
    return result as usize;
}

pub fn find_top_left_turns(adj_edges: &Vec<i32>) -> usize
{
    let mut first = true;
    for i in 0..8
    {
        if adj_edges[i%4] == 0
        {
            if !first
            {
                return 4 - i%4;
            }
            first = false;
        }
    }
    panic!("ran out of iterations for {:?}", adj_edges);
}

fn find_matching_tile(edge: &i32, ignore_tile: i32, tiles: &Vec<ImageTile>) -> Option<i32>
{
    let comp_edge = compliment_edge(*edge);
    return tiles.iter()
        .filter(|t| t.id != ignore_tile)
        .find(|t| t.edges.contains(edge) || t.edges.contains(&comp_edge))
        .map(|t| t.id);
}

fn parse_images(input: &Vec<String>) -> Vec<ImageTile>
{
    let mut result = Vec::new();

    let mut tile_id = 0;
    let mut data = Vec::new();
    for line in input
    {
        if line == ""
        {
            result.push(ImageTile::new(tile_id, &data));
            data = Vec::new();
            tile_id = 0;
        }
        else if &line[0..5] == "Tile "
        {
            tile_id = line[5..9].parse::<i32>().unwrap();
        }
        else
        {
            data.push(line.clone());
        }
    }
    let copy_result = result.clone();
    for tile in result.iter_mut()
    {
        tile.adj_tids = tile.edges.iter()
            .map(|e| find_matching_tile(e, tile.id, &copy_result).unwrap_or(0))
            .collect();
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliment_edge() 
    {
        assert_eq!(512, compliment_edge(1));
        assert_eq!(768, compliment_edge(3));
        assert_eq!(0, compliment_edge(0));
        assert_eq!(1023, compliment_edge(1023));
    }

    #[test]
    fn test_find_edges() 
    {
        let input = vec![
            String::from("..##.#..#."),
            String::from("##..#....."),
            String::from("#...##..#."),
            String::from("####.#...#"),
            String::from("##.##.###."),
            String::from("##...#.###"),
            String::from(".#.#.#..##"),
            String::from("..#....#.."),
            String::from("###...#.#."),
            String::from("..###..###")
        ];

        let result = find_edges(&input);
        let expected = vec![300, 616, 231, 498];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_edges2() 
    {
        let input = vec![
            String::from("###.##.#.."),
            String::from(".#..#.##.."),
            String::from(".#.##.#..#"),
            String::from("#.#.#.##.#"),
            String::from("....#...##"),
            String::from("...##..##."),
            String::from("...#.#####"),
            String::from(".#.####.#."),
            String::from("..#..###.#"),
            String::from("..##.#..#."),
        ];

        let result = find_edges(&input);
        let expected = vec![183, 348, 210, 576];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_edges3() 
    {
        let input = vec![
            String::from("#....#####"),
            String::from("#........."),
            String::from("#........."),
            String::from("#........."),
            String::from("#........."),
            String::from(".........#"),
            String::from(".........#"),
            String::from(".........#"),
            String::from(".........#"),
            String::from("#####....#"),
        ];

        let result = find_edges(&input);
        let expected = vec![993, 993, 993, 993];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_edges4() 
    {
        let input = vec![
            String::from("#.#.#####."),
            String::from(".#..######"),
            String::from("..#......."),
            String::from("######...."),
            String::from("####.#..#."),
            String::from(".#...#.##."),
            String::from("#.#####.##"),
            String::from("..#.###..."),
            String::from("..#......."),
            String::from("..#.###..."),
        ];

        let result = find_edges(&input);
        let expected = vec![501, 66, 184, 616];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_corners() 
    {
        let input = vec![
            String::from("Tile 2311:"),
            String::from("..##.#..#."),
            String::from("##..#....."),
            String::from("#...##..#."),
            String::from("####.#...#"),
            String::from("##.##.###."),
            String::from("##...#.###"),
            String::from(".#.#.#..##"),
            String::from("..#....#.."),
            String::from("###...#.#."),
            String::from("..###..###"),
            String::from(""),
            String::from("Tile 1951:"),
            String::from("#.##...##."),
            String::from("#.####...#"),
            String::from(".....#..##"),
            String::from("#...######"),
            String::from(".##.#....#"),
            String::from(".###.#####"),
            String::from("###.##.##."),
            String::from(".###....#."),
            String::from("..#.#..#.#"),
            String::from("#...##.#.."),
            String::from(""),
            String::from("Tile 1171:"),
            String::from("####...##."),
            String::from("#..##.#..#"),
            String::from("##.#..#.#."),
            String::from(".###.####."),
            String::from("..###.####"),
            String::from(".##....##."),
            String::from(".#...####."),
            String::from("#.##.####."),
            String::from("####..#..."),
            String::from(".....##..."),
            String::from(""),
            String::from("Tile 1427:"),
            String::from("###.##.#.."),
            String::from(".#..#.##.."),
            String::from(".#.##.#..#"),
            String::from("#.#.#.##.#"),
            String::from("....#...##"),
            String::from("...##..##."),
            String::from("...#.#####"),
            String::from(".#.####.#."),
            String::from("..#..###.#"),
            String::from("..##.#..#."),
            String::from(""),
            String::from("Tile 1489:"),
            String::from("##.#.#...."),
            String::from("..##...#.."),
            String::from(".##..##..."),
            String::from("..#...#..."),
            String::from("#####...#."),
            String::from("#..#.#.#.#"),
            String::from("...#.#.#.."),
            String::from("##.#...##."),
            String::from("..##.##.##"),
            String::from("###.##.#.."),
            String::from(""),
            String::from("Tile 2473:"),
            String::from("#....####."),
            String::from("#..#.##..."),
            String::from("#.##..#..."),
            String::from("######.#.#"),
            String::from(".#...#.#.#"),
            String::from(".#########"),
            String::from(".###.#..#."),
            String::from("########.#"),
            String::from("##...##.#."),
            String::from("..###.#.#."),
            String::from(""),
            String::from("Tile 2971:"),
            String::from("..#.#....#"),
            String::from("#...###..."),
            String::from("#.#.###..."),
            String::from("##.##..#.."),
            String::from(".#####..##"),
            String::from(".#..####.#"),
            String::from("#..#.#..#."),
            String::from("..####.###"),
            String::from("..#.#.###."),
            String::from("...#.#.#.#"),
            String::from(""),
            String::from("Tile 2729:"),
            String::from("...#.#.#.#"),
            String::from("####.#...."),
            String::from("..#.#....."),
            String::from("....#..#.#"),
            String::from(".##..##.#."),
            String::from(".#.####..."),
            String::from("####.#.#.."),
            String::from("##.####..."),
            String::from("##..#.##.."),
            String::from("#.##...##."),
            String::from(""),
            String::from("Tile 3079:"),
            String::from("#.#.#####."),
            String::from(".#..######"),
            String::from("..#......."),
            String::from("######...."),
            String::from("####.#..#."),
            String::from(".#...#.##."),
            String::from("#.#####.##"),
            String::from("..#.###..."),
            String::from("..#......."),
            String::from("..#.###..."),
            String::from("")
        ];

        let result = find_corners(&parse_images(&input));
        let expected = vec![1951, 1171, 2971, 3079];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_construct_image()
    {
        let input = vec![
            String::from("Tile 2311:"),
            String::from("..##.#..#."),
            String::from("##..#....."),
            String::from("#...##..#."),
            String::from("####.#...#"),
            String::from("##.##.###."),
            String::from("##...#.###"),
            String::from(".#.#.#..##"),
            String::from("..#....#.."),
            String::from("###...#.#."),
            String::from("..###..###"),
            String::from(""),
            String::from("Tile 1951:"),
            String::from("#.##...##."),
            String::from("#.####...#"),
            String::from(".....#..##"),
            String::from("#...######"),
            String::from(".##.#....#"),
            String::from(".###.#####"),
            String::from("###.##.##."),
            String::from(".###....#."),
            String::from("..#.#..#.#"),
            String::from("#...##.#.."),
            String::from(""),
            String::from("Tile 1171:"),
            String::from("####...##."),
            String::from("#..##.#..#"),
            String::from("##.#..#.#."),
            String::from(".###.####."),
            String::from("..###.####"),
            String::from(".##....##."),
            String::from(".#...####."),
            String::from("#.##.####."),
            String::from("####..#..."),
            String::from(".....##..."),
            String::from(""),
            String::from("Tile 1427:"),
            String::from("###.##.#.."),
            String::from(".#..#.##.."),
            String::from(".#.##.#..#"),
            String::from("#.#.#.##.#"),
            String::from("....#...##"),
            String::from("...##..##."),
            String::from("...#.#####"),
            String::from(".#.####.#."),
            String::from("..#..###.#"),
            String::from("..##.#..#."),
            String::from(""),
            String::from("Tile 1489:"),
            String::from("##.#.#...."),
            String::from("..##...#.."),
            String::from(".##..##..."),
            String::from("..#...#..."),
            String::from("#####...#."),
            String::from("#..#.#.#.#"),
            String::from("...#.#.#.."),
            String::from("##.#...##."),
            String::from("..##.##.##"),
            String::from("###.##.#.."),
            String::from(""),
            String::from("Tile 2473:"),
            String::from("#....####."),
            String::from("#..#.##..."),
            String::from("#.##..#..."),
            String::from("######.#.#"),
            String::from(".#...#.#.#"),
            String::from(".#########"),
            String::from(".###.#..#."),
            String::from("########.#"),
            String::from("##...##.#."),
            String::from("..###.#.#."),
            String::from(""),
            String::from("Tile 2971:"),
            String::from("..#.#....#"),
            String::from("#...###..."),
            String::from("#.#.###..."),
            String::from("##.##..#.."),
            String::from(".#####..##"),
            String::from(".#..####.#"),
            String::from("#..#.#..#."),
            String::from("..####.###"),
            String::from("..#.#.###."),
            String::from("...#.#.#.#"),
            String::from(""),
            String::from("Tile 2729:"),
            String::from("...#.#.#.#"),
            String::from("####.#...."),
            String::from("..#.#....."),
            String::from("....#..#.#"),
            String::from(".##..##.#."),
            String::from(".#.####..."),
            String::from("####.#.#.."),
            String::from("##.####..."),
            String::from("##..#.##.."),
            String::from("#.##...##."),
            String::from(""),
            String::from("Tile 3079:"),
            String::from("#.#.#####."),
            String::from(".#..######"),
            String::from("..#......."),
            String::from("######...."),
            String::from("####.#..#."),
            String::from(".#...#.##."),
            String::from("#.#####.##"),
            String::from("..#.###..."),
            String::from("..#......."),
            String::from("..#.###..."),
            String::from("")
        ];

        let mut result = ImageTile::new(1, &construct_image(&parse_images(&input)));
        result.turn(1);
        result.flip_y();

        let expected = vec![
            String::from(".#.#..#.##...#.##..#####"),
            String::from("###....#.#....#..#......"),
            String::from("##.##.###.#.#..######..."),
            String::from("###.#####...#.#####.#..#"),
            String::from("##.#....#.##.####...#.##"),
            String::from("...########.#....#####.#"),
            String::from("....#..#...##..#.#.###.."),
            String::from(".####...#..#.....#......"),
            String::from("#..#.##..#..###.#.##...."),
            String::from("#.####..#.####.#.#.###.."),
            String::from("###.#.#...#.######.#..##"),
            String::from("#.####....##..########.#"),
            String::from("##..##.#...#...#.#.#.#.."),
            String::from("...#..#..#.#.##..###.###"),
            String::from(".#.#....#.##.#...###.##."),
            String::from("###.#...#..#.##.######.."),
            String::from(".#.#.###.##.##.#..#.##.."),
            String::from(".####.###.#...###.#..#.#"),
            String::from("..#.#..#..#.#.#.####.###"),
            String::from("#..####...#.#.#.###.###."),
            String::from("#####..#####...###....##"),
            String::from("#.##..#..#...#..####...#"),
            String::from(".#.###..##..##..####.##."),
            String::from("...###...##...#...#..###")
        ];
        for i in 0..result.data.len()
        {
            assert_eq!(expected[i], result.data[i], "Failed at row index {}", i);
        }
        assert_eq!(expected, result.data);
    }
}