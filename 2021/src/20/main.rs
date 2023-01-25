use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("src/20/input.txt") {
        let mut it = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        let algo = process_algo(&it.next().unwrap());
        it.next();
        let image = process_image(&it.collect());

        println!("Part 1: {}", part_one(&algo, &image));
        // println!("Part 2: {}", part_two(&input));
    }
}

fn process_algo(input: &String) -> Vec<bool>
{
    return input.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("unexpected aglo character {}", c)
        })
        .collect();
}

struct Image
{
    pixels: HashMap<(usize, usize), bool>
}

fn process_image(input: &Vec<String>) -> Image
{
    let mut map:HashMap<(usize, usize), bool> = HashMap::new();

    for y in 0..input.len()
    {
        let mut x = 0;
        for c in input[y].chars()
        {
            if c == '#'
            {
                map.insert((x, y), true);
            }
            x = x + 1;
        }
    }
    return Image{ pixels: map };
}

fn part_one(algo: &Vec<bool>, input: &Image) -> usize
{
    return 0;
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
    fn test_part_one() 
    {
        let algo_str = String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
        let img_str = vec![
            String::from("#..#."),
            String::from("#...."),
            String::from("##..#"),
            String::from("..#.."),
            String::from("..###")
        ];
        assert_eq!(35, part_one(&process_algo(&algo_str), &process_image(&img_str)));
    }
}
