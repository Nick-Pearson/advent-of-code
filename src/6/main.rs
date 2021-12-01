use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    if let Ok(lines) = read_lines("src/6/input.txt") {

        let mut it = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        let forms = get_forms(&mut it);
        
        let total1:usize = forms.iter()
            .map(|x| get_any_yeses(&x))
            .map(|x| x.len())
            .sum();
        println!("Total (any): {}", total1);
        let total2:usize = forms.iter()
            .map(|x| get_all_yeses(&x))
            .map(|x| x.len())
            .sum();
        println!("Total (all): {}", total2);
    }
}

pub fn get_forms(items: &mut dyn Iterator<Item = String>) -> Vec<Vec<String>>
{
    let mut forms = Vec::new();
    let mut cur_form = Vec::new();

    for it in items
    {
        if "" == it
        {
            forms.push(cur_form);
            cur_form = Vec::new();
        }
        else
        {
            cur_form.push(it);
        }
    }
    forms.push(cur_form);
    return forms;
}

pub fn get_any_yeses(input: &Vec<String>) -> Vec<char>
{
    let answers:HashSet<char> = input.iter()
        .flat_map(|x| x.chars())
        .collect();

    let mut result:Vec<char> = answers.into_iter().collect();
    result.sort();
    return result;
}


pub fn get_all_yeses(input: &Vec<String>) -> Vec<char>
{
    let mut it = input.iter();
    let init = HashSet::<char>::from_iter(it.next().unwrap().chars());
    let answers:HashSet<char> = it
        .map(|x| HashSet::<char>::from_iter(x.chars()))
        .fold(init, |a,b| a.intersection(&b).cloned().collect());

    let mut result:Vec<char> = answers.into_iter().collect();
    result.sort();
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
    fn test_get_any_yeses() 
    {
        assert_eq!(vec!['a', 'b', 'c'], get_any_yeses(&vec![String::from("abc")]));
        assert_eq!(vec!['a', 'b', 'c'], get_any_yeses(&vec![String::from("a"), String::from("b"), String::from("c")]));
        assert_eq!(vec!['a', 'b', 'c'], get_any_yeses(&vec![String::from("ab"), String::from("ac")]));
        assert_eq!(vec!['a'], get_any_yeses(&vec![String::from("a"), String::from("a"), String::from("a"), String::from("a")]));
        assert_eq!(vec!['b'], get_any_yeses(&vec![String::from("b")]));
    }

    #[test]
    fn test_get_all_yeses() 
    {
        assert_eq!(vec!['a', 'b', 'c'], get_all_yeses(&vec![String::from("abc")]));
        assert_eq!(0, get_all_yeses(&vec![String::from("a"), String::from("b"), String::from("c")]).len());
        assert_eq!(vec!['a'], get_all_yeses(&vec![String::from("ab"), String::from("ac")]));
        assert_eq!(vec!['a'], get_all_yeses(&vec![String::from("a"), String::from("a"), String::from("a"), String::from("a")]));
        assert_eq!(vec!['b'], get_all_yeses(&vec![String::from("b")]));
    }
}