use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use joinery::Joinable;

fn main() 
{
    if let Ok(lines) = read_lines("src/21/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let foods = parse_foods(&input);
        let safe_ingredients = get_safe_ingredients(&foods);
        let safe_ingred_count:usize = safe_ingredients.iter()
            .map(|i| foods.iter().filter(|f| f.ingredients.contains(i)).count())
            .sum();
        println!("Safe ingredient count: {}", safe_ingred_count);
        let dangerous_ingred_list = get_dangerous_ingredient_list(&foods);
        println!("Dangerous ingredients: {}", dangerous_ingred_list.join_with(","));
    }
}


#[derive(Debug, Clone)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: Vec<String>
}

fn get_mappings(input: &Vec<Food>) -> HashMap<String, String>
{
    let mut possibles:HashMap<String, HashSet<String>> = HashMap::new();

    for food in input
    {
        for allergen in &food.allergens
        {
            let result = possibles.get(allergen)
                .map_or_else(|| food.ingredients.clone(), |s| s.intersection(&food.ingredients).cloned().collect());
            possibles.insert(allergen.to_string(), result);
        }
    }

    // ingredient to allergen
    let mut mappings:HashMap<String, String> = HashMap::new();
    let mut changed = true;
    while changed
    {
        changed = false;
        for (k, v) in possibles.iter()
        {
            if v.len() == 1
            {
                mappings.insert(v.iter().next().unwrap().to_string(), k.to_string());
                changed = true;
            }
        }

        for (k,v) in mappings.iter()
        {
            possibles.remove(v);
            for (_, v) in possibles.iter_mut()
            {
                v.remove(k);
            }
        }
    }
    return mappings;
}

pub fn get_safe_ingredients(input: &Vec<Food>) -> Vec<String>
{
    let mappings = get_mappings(input);

    let mut result:Vec<_> = input.iter()
        .flat_map(|f| &f.ingredients)
        .filter(|i| mappings.get(*i).is_none())
        .collect::<HashSet<_>>()
        .into_iter()
        .cloned()
        .collect();
    result.sort();
    return result;
}

pub fn get_dangerous_ingredient_list(input: &Vec<Food>) -> Vec<String>
{
    let mappings = get_mappings(input);

    let mut result:Vec<(&String, &String)> = mappings.iter().collect();
    result.sort_by_key(|i| i.1);
    return result.iter().map(|i| i.0).cloned().collect();
}

pub fn parse_foods(input: &Vec<String>) -> Vec<Food>
{
    return input.iter()
        .map(|line| {
            let mut parts = line.split(" (contains ");
            let ingredients = parts.next().unwrap();
            let allergens = parts.next().unwrap();
            let food = Food{
                ingredients: ingredients.split(' ').map(|s| s.to_string()).collect(),
                allergens: allergens[..allergens.len()-1].split(", ").map(|s| s.to_string()).collect()
            };
            return food;
        })
        .collect();
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
    fn test_get_safe_ingredients() 
    {
        let input = vec![
            String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
            String::from("sqjhc fvjkl (contains soy)"),
            String::from("sqjhc mxmxvkd sbzzf (contains fish)")
        ];

        let expected = vec![
            String::from("kfcds"),
            String::from("nhms"),
            String::from("sbzzf"),
            String::from("trh")
        ];
        assert_eq!(expected, get_safe_ingredients(&parse_foods(&input)));
    }

    #[test]
    fn test_get_dangerous_ingredient_list() 
    {
        let input = vec![
            String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
            String::from("sqjhc fvjkl (contains soy)"),
            String::from("sqjhc mxmxvkd sbzzf (contains fish)")
        ];

        let expected = vec![
            String::from("mxmxvkd"),
            String::from("sqjhc"),
            String::from("fvjkl"),
        ];
        assert_eq!(expected, get_dangerous_ingredient_list(&parse_foods(&input)));
    }
}