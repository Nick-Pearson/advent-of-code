use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() 
{
    if let Ok(lines) = read_lines("src/17/input.txt") {
        let input = &mut lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        
        let count6 = count_after_cycles(&input, 6);
        println!("{} cubes active after 6 cycles", count6);
    }
}

#[derive(Debug, Clone)]
pub struct Conway {
    // Z Y X
    grid: Vec<Vec<Vec<bool>>>
}

impl Conway
{
    pub fn new(init: Vec<Vec<bool>>) -> Conway
    {
        let mut vec = Vec::new();
        vec.push(init);
        return Conway{
            grid: vec
        };
    }

    pub fn count_active(&self) -> i64
    {
        return self.grid.iter()
            .map(|l| self.count_active_in_layer(l))
            .sum();
    }

    fn count_active_in_layer(&self, layer: &Vec<Vec<bool>>) -> i64
    {
        return layer.iter()
            .map(|row| self.count_active_in_row(row))
            .sum()
    }
    
    fn count_active_in_row(&self, row: &Vec<bool>) -> i64
    {
        return row.iter().filter(|v| **v).count() as i64;
    }

    pub fn expand_grid(&mut self)
    {
        let size_x = self.grid[0][0].len();
        for layer in &mut self.grid
        {
            layer.push(vec![false; size_x]);
            layer.insert(0, vec![false; size_x]);

            for row in layer
            {
                row.insert(0, false);
                row.push(false);
            }
        }

        let mut empty_layer = Vec::new();
        for _y in 0..self.grid[0].len()
        {
            empty_layer.push(vec![false; size_x+2]);
        }
        
        self.grid.push(empty_layer.clone());
        self.grid.insert(0, empty_layer);
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: bool)
    {
        self.grid[z][y][x] = value;
    }

    pub fn debug_print(&self)
    {
        for layer in 0..self.grid.len()
        {
            for s in self.get_layer_string(layer).iter()
            {
                println!("{}", s)
            }
            println!("");
        }
    }

    pub fn get_layer_string(&self, z: usize) -> Vec<String>
    {
        return self.grid[z].iter()
            .filter(|r| !r.iter().all(|r| !*r))
            .map(|r| r.iter().map(|t| if *t { return '#' } else { return '.' }).collect::<String>())
            .collect();
    }
}

pub fn count_after_cycles(input: &Vec<String>, cycles: usize) -> i64
{
    return cycle_n_times(parse_input(input), cycles).count_active();
}

pub fn cycle_n_times(input: Conway, cycles: usize) -> Conway
{
    let mut input_grid = &mut input.clone();
    let mut output_grid = &mut input.clone();
    println!("Printing Initial Grid");
    output_grid.debug_print();

    for _i in 0..cycles
    {
        input_grid.expand_grid();
        output_grid.expand_grid();

        let tmp = output_grid;
        output_grid = input_grid;
        input_grid = tmp;

        iterate_conway(input_grid, output_grid);
        
        println!("Printing Grid after {} cycles", _i+1);
        output_grid.debug_print();
    }
    return output_grid.clone();
}

pub fn iterate_conway(input: &Conway, output: &mut Conway)
{
    for z in 0..input.grid.len()
    {
        let mut prev_layer = None;
        if z > 0
        {
            prev_layer = input.grid.get(z - 1);
        }
        let layer = &input.grid[z];
        let next_layer = input.grid.get(z + 1);

        for y in 0..layer.len()
        {
            for x in 0..layer[y].len()
            {
                let active = layer[y][x];
                let count = count_at(x, y, prev_layer, layer, next_layer);

                if active
                {
                    // print!("_{}_", count);
                    output.set(x, y, z, count == 2 || count == 3);
                }
                else
                {
                    // print!(" {} ", count);
                    output.set(x, y, z, count == 3);
                }
            }
            // println!("");
        }
        // println!("\n");
    }
}

pub fn count_at(x: usize, y: usize, prev_layer: Option<&Vec<Vec<bool>>>, layer: &Vec<Vec<bool>>, next_layer: Option<&Vec<Vec<bool>>>) -> i32
{
    let mut count: i32 = 0;
    count = count + prev_layer.map_or(0, |l| count_layer(x, y, l));
    count = count + next_layer.map_or(0, |l| count_layer(x, y, l));
    count = count + count_layer(x, y, layer);
    if layer[y][x]
    {
        count = count - 1;
    }
    return count;
}

pub fn count_layer(x: usize, y: usize, layer: &Vec<Vec<bool>>) -> i32
{
    let mut prev = None;
    if y > 0 
    {
        prev = layer.get(y - 1)
    }
    return count_occupied(x, prev, &layer[y], layer.get(y + 1));
}

pub fn count_occupied(x: usize, line_above: Option<&Vec<bool>>, line: &Vec<bool>, line_below: Option<&Vec<bool>>) -> i32
{
    let mut count: i32 = 0;
    count = count + line_above.map_or(0, |l| count_line(x, l));
    count = count + line_below.map_or(0, |l| count_line(x, l));
    count = count + count_line(x, line);
    return count;
}

fn count_line(x: usize, line: &Vec<bool>) -> i32
{
    let mut count: i32 = 0;
    if x > 0 
    {
        count = count + line.get(x - 1).map_or(0, |c| if *c { 1 } else { 0 });
    }
    count = count + (if line[x] { 1 } else { 0 });
    count = count + line.get(x + 1).map_or(0, |c| if *c { 1 } else { 0 });
    return count;
}


fn parse_input(input: &Vec<String>) -> Conway
{
    let lines:Vec<_> = input.iter()
        .map(|s| to_lines(s))
        .collect();
    return Conway::new(lines);
}

fn to_lines(input: &String) -> Vec<bool>
{
    return input.chars().map(|c| c == '#').collect();
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
    fn test_to_lines() 
    {
        assert_eq!(vec![false, true, false], to_lines(&String::from(".#.")));
        assert_eq!(vec![false, false, true], to_lines(&String::from("..#")));
        assert_eq!(vec![true, true, true], to_lines(&String::from("###")));
    }

    #[test]
    fn test_count_at_8() 
    {
        let input = vec![
            "###",
            "###",
            "###"
        ].iter().map(|s| s.to_string()).collect();
        let layer = &parse_input(&input).grid[0];
        assert_eq!(8, count_at(1, 1, None, layer, None));
        assert_eq!(3, count_at(0, 0, None, layer, None));
        assert_eq!(3, count_at(2, 0, None, layer, None));
        assert_eq!(3, count_at(0, 2, None, layer, None));
        assert_eq!(3, count_at(2, 2, None, layer, None));
    }
    
    #[test]
    fn test_count_at_0() 
    {
        let input = vec![
            "...",
            ".#.",
            "..."
        ].iter().map(|s| s.to_string()).collect();
        let layer = &parse_input(&input).grid[0];
        assert_eq!(0, count_at(1, 1, None, layer, None));
        assert_eq!(1, count_at(0, 0, None, layer, None));
        assert_eq!(1, count_at(0, 2, None, layer, None));
        assert_eq!(1, count_at(2, 0, None, layer, None));
        assert_eq!(1, count_at(2, 2, None, layer, None));
    }
    
    #[test]
    fn test_count_at_ex() 
    {
        let input = vec![
            ".#.",
            "..#",
            "###"
        ].iter().map(|s| s.to_string()).collect();
        let layer = &parse_input(&input).grid[0];
        assert_eq!(1, count_at(0, 0, None, layer, None));
        assert_eq!(2, count_at(2, 0, None, layer, None));
        assert_eq!(5, count_at(1, 1, None, layer, None));
        assert_eq!(3, count_at(2, 1, None, layer, None));
        assert_eq!(3, count_at(1, 2, None, layer, None));
    }
    
    #[test]
    fn test_cycle_1_times() 
    {
        let input = vec![
            ".#.",
            "..#",
            "###"
        ].iter().map(|s| s.to_string()).collect();

        let result = cycle_n_times(parse_input(&input), 1);
        let expected0:Vec<String> = vec![
            ".#...",
            "...#.",
            "..#.."
        ].iter().map(|s| s.to_string()).collect();
        assert_eq!(expected0, result.get_layer_string(0));
        let expected1:Vec<String> = vec![
            ".#.#.",
            "..##.",
            "..#.."
        ].iter().map(|s| s.to_string()).collect();
        assert_eq!(expected1, result.get_layer_string(1));
        let expected2:Vec<String> = vec![
            ".#...",
            "...#.",
            "..#.."
        ].iter().map(|s| s.to_string()).collect();
        assert_eq!(expected2, result.get_layer_string(2));
    }

    #[test]
    fn test_count_after_cycles() 
    {
        let input = vec![
            ".#.",
            "..#",
            "###"
        ].iter().map(|s| s.to_string()).collect();

        assert_eq!(112, count_after_cycles(&input, 6));
    }
}