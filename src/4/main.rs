#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("src/4/input.txt") {
        let required = make_hash_set(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

        let mut passports = Vec::new();
        let mut cur_pass = HashMap::new();
        for line in lines {
            if let Ok(ip) = line {
                if "" == ip
                {
                    passports.push(cur_pass);
                    cur_pass = HashMap::new();
                }
                else
                {
                    cur_pass.extend(parse_data(&ip));
                }
            }
        }
        passports.push(cur_pass);

        let count1 = passports.iter()
            .filter(|x| is_valid1(&mut x.keys(), &required))
            .count();
        println!("valid 1 = {}", count1);
        let count2 = passports.iter()
            .filter(|x| is_valid2(x, &required))
            .count();
        println!("valid 2 = {}", count2);
    }
}

pub fn parse_data(input: &str) -> HashMap<String, String>
{
    let parts = input.split(' ');
    let mut result = HashMap::new();

    for part in parts
    {
        let mut s = part.split(':');
        result.insert(String::from(s.next().unwrap()), String::from(s.next().unwrap()));
    }
    return result;
}

pub fn is_valid1(input: &mut dyn Iterator<Item = &String>, test: &HashSet<String>) -> bool
{
    return input
        .filter(|x| test.contains(*x))
        .count() >= test.len()
}

pub fn is_valid2(input: &HashMap<String, String>, test: &HashSet<String>) -> bool
{
    if !is_valid1(&mut input.keys(), test)
    {
        return false;
    }
    else
    {
        return valid_year(input.get("byr").unwrap(), 1920, 2002) &&
            valid_year(input.get("iyr").unwrap(), 2010, 2020) &&
            valid_year(input.get("eyr").unwrap(), 2020, 2030) &&
            valid_height(input.get("hgt").unwrap()) &&
            valid_colour(input.get("hcl").unwrap()) &&
            valid_eye_col(input.get("ecl").unwrap()) &&
            valid_pid(input.get("pid").unwrap());
    }
}

pub fn make_hash_set(input: Vec<&str>) -> HashSet<String>
{
    return input.iter()
        .map(|x| String::from(*x))
        .collect();

}

pub fn valid_year(input: &String, min: u32, max: u32) -> bool
{
    return input.parse::<u32>()
        .map_or(false, |x| x >= min && x <= max);
}

pub fn valid_colour(input: &String) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new("#[0-9|a-f]{6}").unwrap();
    }
    return RE.is_match(input);
}

pub fn valid_pid(input: &String) -> bool
{
    if input.len() != 9
    {
        return false
    }
    lazy_static! {
        static ref RE: Regex = Regex::new("[0-9]{9}$").unwrap();
    }
    return RE.is_match(input);
}

pub fn valid_eye_col(input: &String) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
    }
    return RE.is_match(input);
}

pub fn valid_height(input: &String) -> bool
{
    lazy_static! {
        static ref CM: Regex = Regex::new("[1-9][0-9][0-9]cm").unwrap();
        static ref IN: Regex = Regex::new("[1-9][0-9]in").unwrap();
    }
    if CM.is_match(input)
    {
        let val = input[0..3].parse::<u32>().unwrap();
        return val >= 150 && val <= 193;
    }
    else if IN.is_match(input)
    {
        let val = input[0..2].parse::<u32>().unwrap();
        return val >= 59 && val <= 76;
    }
    else
    {
        return false;
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
    fn test_parse_data() 
    {
        let mut map = HashMap::new();
        map.insert(String::from("hgt"), String::from("1"));
        map.insert(String::from("wgt"), String::from("2"));
        map.insert(String::from("test"), String::from("3"));
        assert_eq!(map, parse_data("hgt:1 wgt:2 test:3"));
    }
    
    #[test]
    fn test_is_valid1() 
    {
        assert_eq!(false, is_valid1(&mut parse_data("hgt:1 wgt:2 test:3").keys(), &make_hash_set(vec!["hgt", "wgt", "test", "other"])));
        assert_eq!(false, is_valid1(&mut parse_data("hgt:1 wgt:2 test:3").keys(), &make_hash_set(vec!["other"])));
        assert_eq!(true, is_valid1(&mut parse_data("hgt:1 wgt:2 test:3").keys(), &make_hash_set(vec!["hgt", "wgt", "test"])));
        assert_eq!(true, is_valid1(&mut parse_data("hgt:1 wgt:2 test:3").keys(), &make_hash_set(vec!["hgt", "wgt"])));
        assert_eq!(true, is_valid1(&mut parse_data("hgt:1 wgt:2 test:3 test:3").keys(), &make_hash_set(vec!["hgt", "wgt", "test"])));
    }

    #[test]
    fn test_valid_year() 
    {
        assert_eq!(false, valid_year(&String::from("PP"), 2020, 2030));
        assert_eq!(false, valid_year(&String::from("2019"), 2020, 2030));
        assert_eq!(false, valid_year(&String::from("0"), 2020, 2030));
        assert_eq!(false, valid_year(&String::from("2031"), 2020, 2030));
        assert_eq!(true, valid_year(&String::from("2020"), 2020, 2030));
        assert_eq!(true, valid_year(&String::from("2030"), 2020, 2030));
        assert_eq!(true, valid_year(&String::from("2025"), 2020, 2030));
    }

    #[test]
    fn test_valid_colour() 
    {
        assert_eq!(false, valid_colour(&String::from("PP")));
        assert_eq!(false, valid_colour(&String::from("#11223")));
        assert_eq!(false, valid_colour(&String::from("112233")));
        assert_eq!(false, valid_colour(&String::from("#ffggaa")));
        assert_eq!(false, valid_colour(&String::from("#55AAFF")));
        assert_eq!(true, valid_colour(&String::from("#002244")));
        assert_eq!(true, valid_colour(&String::from("#ffff22")));
        assert_eq!(true, valid_colour(&String::from("#abcd98")));
    }

    #[test]
    fn test_valid_pid() 
    {
        assert_eq!(false, valid_pid(&String::from("ABCDEFGH")));
        assert_eq!(false, valid_pid(&String::from("0")));
        assert_eq!(false, valid_pid(&String::from("-1234797")));
        assert_eq!(false, valid_pid(&String::from("12345678")));
        assert_eq!(false, valid_pid(&String::from("0123456789")));
        assert_eq!(true, valid_pid(&String::from("123456789")));
        assert_eq!(true, valid_pid(&String::from("000000001")));
        assert_eq!(true, valid_pid(&String::from("999888777")));
    }

    #[test]
    fn test_valid_eye_col() 
    {
        assert_eq!(false, valid_eye_col(&String::from("red")));
        assert_eq!(false, valid_eye_col(&String::from("0")));
        assert_eq!(false, valid_eye_col(&String::from("")));
        assert_eq!(true, valid_eye_col(&String::from("amb")));
        assert_eq!(true, valid_eye_col(&String::from("blu")));
        assert_eq!(true, valid_eye_col(&String::from("brn")));
        assert_eq!(true, valid_eye_col(&String::from("gry")));
        assert_eq!(true, valid_eye_col(&String::from("grn")));
        assert_eq!(true, valid_eye_col(&String::from("hzl")));
        assert_eq!(true, valid_eye_col(&String::from("oth")));
    }

    #[test]
    fn test_valid_height() 
    {
        assert_eq!(false, valid_height(&String::from("160")));
        assert_eq!(false, valid_height(&String::from("65")));
        assert_eq!(false, valid_height(&String::from("165cd")));
        assert_eq!(false, valid_height(&String::from("PP")));
        assert_eq!(false, valid_height(&String::from("170CM")));
        assert_eq!(false, valid_height(&String::from("194cm")));
        assert_eq!(false, valid_height(&String::from("149cm")));
        assert_eq!(false, valid_height(&String::from("58in")));
        assert_eq!(false, valid_height(&String::from("77in")));
        assert_eq!(true, valid_height(&String::from("150cm")));
        assert_eq!(true, valid_height(&String::from("193cm")));
        assert_eq!(true, valid_height(&String::from("165cm")));
        assert_eq!(true, valid_height(&String::from("59in")));
        assert_eq!(true, valid_height(&String::from("76in")));
        assert_eq!(true, valid_height(&String::from("65in")));
    }
}