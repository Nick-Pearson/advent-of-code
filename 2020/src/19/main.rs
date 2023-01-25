use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate regex;
use regex::Regex;

fn main() 
{
    if let Ok(lines) = read_lines("src/19/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        
        let mut it = input.iter();
        let mut rules = get_sorted_rules(&mut it);
        let tests:Vec<&String> = it.collect();

        let re1 = Regex::new(&get_rule(&rules)).unwrap();
        let valid_rules1 = tests.iter()
            .filter(|r| re1.is_match(r))
            .count();
        println!("Total Rules: {}", tests.len());
        println!("Valid Rules 1: {}", valid_rules1);

        rules[8] = String::from("*42");
        let mut rule11 = String::new();
        for _i in 1..2
        {
            let mut r = String::new();
            r.push_str(" 42 ");
            for _j in 1.._i
            {
                r.push_str("42 ");
            }
            r.push_str("31");
            for _j in 1.._i
            {
                r.push_str(" 31");
            }
            r.push_str(" |");
            rule11.push_str(&r);
        }
        rule11.remove(0);
        rule11.pop();
        rule11.pop();
        rules[11] = rule11;
        let re2 = Regex::new(&get_rule(&rules)).unwrap();
        let valid_rules2 = tests.iter()
            .filter(|r| re2.is_match(r))
            .count();
        println!("Valid Rules 2: {}", valid_rules2);
    }
}

pub fn get_sorted_rules(input: &mut dyn Iterator<Item = &String>) -> Vec<String>
{
    let mut rules = Vec::new();
    for val in input
    {
        if val == ""
        {
            break;
        }
        let split:Vec<&str> = val.split(": ").collect();
        rules.push((split[0].parse::<usize>().unwrap(), split[1].to_owned()));
    }
    rules.sort_by_key(|r| r.0);
    return rules.iter().map(|r| r.1.to_string()).collect();
}

pub fn get_rule(rules: &Vec<String>) -> String
{
    let mut regex = get_regex_for(0, rules);
    regex.push('$');
    regex.insert(0, '^');
    return regex;
}

pub fn get_regex_for(rule_id: usize, rules: &Vec<String>) -> String
{
    let rule = &rules[rule_id];
    
    if rule.find('|') != None
    {
        let subrules:Vec<String> = rule.split(" | ")
                .map(|s| p_rule(s, rules))
                .collect();
        let mut result = String::new();
        result.push('(');
        for s in subrules
        {
            result.push_str(&s);
            result.push('|');
        }
        result.pop();
        result.push(')');
        return result;
    }
    else 
    {
        return p_rule(&rule[..], rules);
    }
}

pub fn p_rule(rule: &str, rules: &Vec<String>) -> String
{
    if &rule[0..1] == "\""
    {
        return rule[1..2].to_string();
    }
    else if &rule[0..1] == "*"
    {
        let rid = rule[1..].parse::<usize>().unwrap();
        let mut r = get_regex_for(rid, rules);
        r.insert(0, '(');
        r.push(')');
        r.push('*');
        return r;
    }
    else
    {
        let subrules:Vec<String> = rule.split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .map(|rid| get_regex_for(rid, rules))
            .collect();
        let mut result = String::new();
        for s in subrules
        {
            result.push_str(&s);
        }
        return result;
    }
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
    fn test_get_rule_value() 
    {
        let input:Vec<String> = vec![
            String::from("\"a\""),
        ];
        assert_eq!("^a$", get_rule(&input));
    }

    #[test]
    fn test_get_rule_chain() 
    {
        let input:Vec<String> = vec![
            String::from("1 2"),
            String::from("\"a\""),
            String::from("\"b\"")
        ];
        assert_eq!("^ab$", get_rule(&input));
    }

    #[test]
    fn test_get_rule_or() 
    {
        let input:Vec<String> = vec![
            String::from("1 2 | 2 1"),
            String::from("\"a\""),
            String::from("\"b\"")
        ];
        assert_eq!("^(ab|ba)$", get_rule(&input));
    }

    #[test]
    fn test_valid_tests() 
    {
        let input:Vec<String> = vec![
            String::from("4 1 5"),
            String::from("2 3 | 3 2"),
            String::from("4 4 | 5 5"),
            String::from("4 5 | 5 4"),
            String::from("\"a\""),
            String::from("\"b\"")
        ];
        let rule = get_rule(&input);
        let re = Regex::new(&rule).unwrap();
        println!("{}", rule);

        assert_eq!(true, re.is_match("ababbb"));
        assert_eq!(true, re.is_match("abbbab"));
        assert_eq!(false, re.is_match("bababa"));
        assert_eq!(false, re.is_match("aaabbb"));
        assert_eq!(false, re.is_match("aaaabbb"));
    }
}