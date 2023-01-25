use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("src/12/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        println!("Part 1: {}", part_one(&input));
        println!("Part 2: {}", part_two(&input));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_mappings(input: &Vec<String>) -> HashMap<String, Vec<String>>
{
    let mut mappings = HashMap::new();
    for line in input
    {
        let mut parts = line.split("-");
        let key = String::from(parts.next().unwrap());
        let val = String::from(parts.next().unwrap());

        if key != "end" && val != "start"
        {
            mappings.entry(key.clone()).or_insert(Vec::new()).push(val.clone());
        }
        if val != "end" && key != "start"
        {
            mappings.entry(val).or_insert(Vec::new()).push(key);
        }
    }
    return mappings;
}

fn is_small_cave(cave: &String) -> bool
{
    return cave.chars().any(|c| matches!(c, 'a'..='z'));
}

fn has_dupe_small_caves(route: &Vec<&String>) -> bool
{
    for i in 0..route.len()
    {
        let cave = route[i];
        if is_small_cave(&cave)
        {
            for j in i+1..route.len()
            {
                if *route[j] == *cave
                {
                    return true;
                }
            }
        }
    }
    return false;
}

fn get_paths(mappings: &HashMap<String, Vec<String>>, part_two: bool) -> Vec<Vec<&String>>
{
    let mut unfinished_routes:Vec<Vec<&String>> = Vec::new();
    for init in mappings.get("start").unwrap()
    {
        unfinished_routes.push(vec![init]);
    }

    let mut routes:Vec<Vec<&String>> = Vec::new();
    while !unfinished_routes.is_empty()
    {
        let mut new_unfinished_routes = Vec::new();
        for r in unfinished_routes
        {
            let current = r[r.len() - 1];
            let possibles = mappings.get(current).unwrap();

            for poss in possibles
            {
                if poss == "end"
                {
                    routes.push(r.clone());
                }
                else if is_small_cave(poss)
                {
                    if !r.iter().any(|x| *x == poss) || (part_two && !has_dupe_small_caves(&r))
                    {
                        let mut new = r.clone();
                        new.push(poss);
                        new_unfinished_routes.push(new);
                    }
                }
                else
                {
                    let mut new = r.clone();
                    new.push(poss);
                    new_unfinished_routes.push(new);
                }
            }
        }
        unfinished_routes = new_unfinished_routes;
    }
    return routes;
}

fn part_one(input: &Vec<String>) -> usize
{
    return get_paths(&get_mappings(input), false).len()
}

fn part_two(input: &Vec<String>) -> usize
{
    return get_paths(&get_mappings(input), true).len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one1() 
    {
        let input = vec![
            String::from("start-A"),
            String::from("start-b"),
            String::from("A-c"),
            String::from("A-b"),
            String::from("b-d"),
            String::from("A-end"),
            String::from("b-end")
        ];
        assert_eq!(10, part_one(&input));
    }

    #[test]
    fn test_part_one2() 
    {
        let input = vec![
            String::from("dc-end"),
            String::from("HN-start"),
            String::from("start-kj"),
            String::from("dc-start"),
            String::from("dc-HN"),
            String::from("LN-dc"),
            String::from("HN-end"),
            String::from("kj-sa"),
            String::from("kj-HN"),
            String::from("kj-dc")
        ];
        assert_eq!(19, part_one(&input));
    }

    #[test]
    fn test_part_one3() 
    {
        let input = vec![
            String::from("fs-end"),
            String::from("he-DX"),
            String::from("fs-he"),
            String::from("start-DX"),
            String::from("pj-DX"),
            String::from("end-zg"),
            String::from("zg-sl"),
            String::from("zg-pj"),
            String::from("pj-he"),
            String::from("RW-he"),
            String::from("fs-DX"),
            String::from("pj-RW"),
            String::from("zg-RW"),
            String::from("start-pj"),
            String::from("he-WI"),
            String::from("zg-he"),
            String::from("pj-fs"),
            String::from("start-RW")
        ];
        assert_eq!(226, part_one(&input));
    }

    #[test]
    fn test_part_two1() 
    {
        let input = vec![
            String::from("start-A"),
            String::from("start-b"),
            String::from("A-c"),
            String::from("A-b"),
            String::from("b-d"),
            String::from("A-end"),
            String::from("b-end")
        ];
        assert_eq!(36, part_two(&input));
    }

    #[test]
    fn test_part_two2() 
    {
        let input = vec![
            String::from("dc-end"),
            String::from("HN-start"),
            String::from("start-kj"),
            String::from("dc-start"),
            String::from("dc-HN"),
            String::from("LN-dc"),
            String::from("HN-end"),
            String::from("kj-sa"),
            String::from("kj-HN"),
            String::from("kj-dc")
        ];
        assert_eq!(103, part_two(&input));
    }

    #[test]
    fn test_part_two3() 
    {
        let input = vec![
            String::from("fs-end"),
            String::from("he-DX"),
            String::from("fs-he"),
            String::from("start-DX"),
            String::from("pj-DX"),
            String::from("end-zg"),
            String::from("zg-sl"),
            String::from("zg-pj"),
            String::from("pj-he"),
            String::from("RW-he"),
            String::from("fs-DX"),
            String::from("pj-RW"),
            String::from("zg-RW"),
            String::from("start-pj"),
            String::from("he-WI"),
            String::from("zg-he"),
            String::from("pj-fs"),
            String::from("start-RW")
        ];
        assert_eq!(3509, part_two(&input));
    }
}