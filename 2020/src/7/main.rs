use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("src/7/input.txt") {
        let it:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let containers = get_all_possible_containers(&mut it.iter(), &String::from("shiny gold"));
        println!("Total Containers: {}", containers.len());
        let total_bags = get_total_bags(&mut it.iter(), &String::from("shiny gold"));
        println!("Total Bags Inside: {}", total_bags);
    }
}

pub fn get_containing_bags(input: &String) -> Vec<String>
{
    return get_containing_bags_and_counts(input).iter()
        .map(|x| x.1.clone())
        .collect();
}

pub fn get_containing_bags_and_counts(input: &String) -> Vec<(u32, String)>
{
    let rhs = input.split(" contain ").skip(1).next().unwrap().to_string();
    if rhs == "no other bags."
    {
        return Vec::new();
    }
    let bags = rhs.split(',');
    return bags
        .map(|x| x.trim())
        .map(|x| x.trim_end_matches('.'))
        .map(|x| x.trim_end_matches(" bags"))
        .map(|x| x.trim_end_matches(" bag"))
        .map(|x| (x[0..1].parse::<u32>().unwrap(), String::from(&x[2..])))
        .collect();
}

pub fn get_outer_bag(input: &String) -> String
{
    return input.split(" bags contain ").next().unwrap().to_string();
}

pub fn get_total_bags(input: &mut dyn Iterator<Item = &String>, target: &String) -> u32
{
    let mut map = HashMap::new();
    for item in input
    {
        let outer = get_outer_bag(item);
        let bags = get_containing_bags_and_counts(item);
        map.insert(outer, bags);
    }
    return get_total_bags_recursive(&map, target) - 1;
}

pub fn get_total_bags_recursive(map: &HashMap<String, Vec<(u32, String)>>, target: &String) -> u32
{
    let results = map.get(target).map_or_else(|| [].iter(), |x| x.iter());
    return results
        .map(|x| get_total_bags_recursive(map, &x.1) * x.0)
        .sum::<u32>() + 1;
}

pub fn get_all_possible_containers(input: &mut dyn Iterator<Item = &String>, target: &String) -> Vec<String>
{
    let map = build_container_map(input);
    return get_all_containers(&map, target);
}

pub fn get_all_containers(map: &HashMap<String, Vec<String>>, target: &String) -> Vec<String>
{
    let mut set = HashSet::new();
    get_all_containers_recursive(map, &mut set, target);
    let mut r:Vec<String> = set.into_iter().collect();
    r.sort();
    return r;
}

pub fn get_all_containers_recursive(map: &HashMap<String, Vec<String>>, set: &mut HashSet<String>, target: &String)
{
    let results = map.get(target).map_or_else(|| Vec::new(), |x| x.to_vec());
    set.insert(target.to_string());
    for res in results
    {
        if !set.contains(&res)
        {
            get_all_containers_recursive(map, set, &res);
        }
        set.insert(res);
    }
    set.remove(target);
}

pub fn build_container_map(input: &mut dyn Iterator<Item = &String>) -> HashMap<String, Vec<String>>
{
    let mut map = HashMap::new();
    for item in input
    {
        let outer = get_outer_bag(item);
        let bags = get_containing_bags(item);
        
        for key in bags
        {
            let values: &mut Vec<String> = map.entry(key).or_insert_with(|| Vec::new());
            values.push(outer.clone());
        }
    }
    return map;
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
    fn test_get_containing_bags() 
    {
        assert_eq!(0, get_containing_bags(&String::from("faded blue bags contain no other bags.")).len());
        assert_eq!(vec!["faded blue", "dotted black"], get_containing_bags(&String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.")));
        assert_eq!(vec!["dark olive", "vibrant plum"], get_containing_bags(&String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.")));
    }

    #[test]
    fn test_get_containing_bags_and_counts() 
    {
        assert_eq!(0, get_containing_bags_and_counts(&String::from("faded blue bags contain no other bags.")).len());
        assert_eq!(vec![(5, String::from("faded blue")), (6, String::from("dotted black"))], get_containing_bags_and_counts(&String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.")));
        assert_eq!(vec![(1, String::from("dark olive")), (2, String::from("vibrant plum"))], get_containing_bags_and_counts(&String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.")));
    }

    #[test]
    fn test_get_outer_bag() 
    {
        assert_eq!("faded blue", get_outer_bag(&String::from("faded blue bags contain no other bags.")));
        assert_eq!("vibrant plum", get_outer_bag(&String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.")));
    }

    #[test]
    fn test_get_all_containers_one_level() 
    {
        let mut map = HashMap::new();
        map.insert(String::from("light green"), vec![String::from("faded blue"), String::from("dark blue")]);
        assert_eq!(vec!["dark blue", "faded blue"], get_all_containers(&map, &String::from("light green")));
    }

    #[test]
    fn test_get_all_containers_two_level() 
    {
        let mut map = HashMap::new();
        map.insert(String::from("light green"), vec![String::from("faded blue"), String::from("dark blue")]);
        map.insert(String::from("yellow"), vec![String::from("light green"), String::from("purple")]);
        assert_eq!(vec!["dark blue", "faded blue", "light green", "purple"], get_all_containers(&map, &String::from("yellow")));
    }

    #[test]
    fn test_get_all_containers_looping() 
    {
        let mut map = HashMap::new();
        map.insert(String::from("light green"), vec![String::from("yellow")]);
        map.insert(String::from("yellow"), vec![String::from("light green")]);
        assert_eq!(vec!["light green"], get_all_containers(&map, &String::from("yellow")));
    }

    #[test]
    fn test_get_all_containers_three_looping() 
    {
        let mut map = HashMap::new();
        map.insert(String::from("light green"), vec![String::from("yellow")]);
        map.insert(String::from("dark green"), vec![String::from("light green")]);
        map.insert(String::from("yellow"), vec![String::from("dark green")]);
        assert_eq!(vec!["dark green", "light green"], get_all_containers(&map, &String::from("yellow")));
    }

    #[test]
    fn test_get_all_possible_containers() 
    {
        let input = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags.")
        ];
        assert_eq!(vec!["bright white", "dark orange", "light red", "muted yellow"], get_all_possible_containers(&mut input.iter(), &String::from("shiny gold")));
    }

    #[test]
    fn test_get_total_bags_one() 
    {
        let input = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags.")
        ];
        assert_eq!(32, get_total_bags(&mut input.iter(), &String::from("shiny gold")));
    }

    #[test]
    fn test_get_total_bags_two() 
    {
        let input = vec![
            String::from("shiny gold bags contain 2 dark red bags."),
            String::from("dark red bags contain 2 dark orange bags."),
            String::from("dark orange bags contain 2 dark yellow bags."),
            String::from("dark yellow bags contain 2 dark green bags."),
            String::from("dark green bags contain 2 dark blue bags."),
            String::from("dark blue bags contain 2 dark violet bags."),
            String::from("dark violet bags contain no other bags."),
        ];
        assert_eq!(126, get_total_bags(&mut input.iter(), &String::from("shiny gold")));
    }
}