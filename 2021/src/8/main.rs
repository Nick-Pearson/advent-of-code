use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

fn main() {
    if let Ok(lines) = read_lines("src/8/input.txt") {
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

fn part_one(input: &Vec<String>) -> usize
{
    return input.iter()
        .map(|v| v.split(" | ").nth(1))
        .map(|v| v.unwrap().split(" "))
        .flatten()
        .map(|v| v.len())
        .filter(|v| *v==2 || *v==4 || *v==3 || *v==7)
        .count();
}

fn overlap_count(a: &str, b: &String) -> i32
{
    let mut c:Vec<char> = a.chars().collect();
    c.retain(|v| b.contains(*v));
    return c.len() as i32;
}

fn translate_five(item: &String, one: &str, four: &str) -> i32
{
    if overlap_count(one, item) == 2
    {
        return 3;
    }
    else if overlap_count(four, item) == 2
    {
        return 2;
    }
    else
    {
        return 5;
    }
}

fn translate_six(item: &String, one: &str, four: &str) -> i32
{
    if overlap_count(one, item) == 1
    {
        return 6;
    }
    else if overlap_count(four, item) == 3
    {
        return 0;
    }
    else
    {
        return 9;
    }
}

fn decode_line(input: &String) -> i32
{
    let replaced = input.replace(" | ", " ");
    let items:Vec<&str> = replaced.split(" ").collect();
    let one = items.iter().find(|v| v.len() == 2).cloned().unwrap();
    let four = items.iter().find(|v| v.len() == 4).cloned().unwrap();

    let translated:Vec<i32> = items.iter()
        .map(|i| {
            let mut chars:Vec<char> = i.chars().collect();
            chars.sort();
            let item = String::from_iter(chars);
            let len = item.len();

            return match len
            {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => translate_five(&item, one, four),
                6 => translate_six(&item, one, four),
                7 => 8,
                _ => panic!("bad len {}", len)
            };
        })
        .collect();

    let end = translated.len();
    return translated[end-1] +
        (translated[end-2]*10) +
        (translated[end-3]*100) +
        (translated[end-4]*1000);
}

fn part_two(input: &Vec<String>) -> i32
{
    return input.iter()
        .map(|v| decode_line(v))
        .fold(0, |a,b| a+b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        let input = vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
        ];
        assert_eq!(26, part_one(&input));
    }

    #[test]
    fn test_part_two() 
    {
        let input = vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")
        ];
        assert_eq!(61229, part_two(&input));
    }

    #[test]
    fn test_decode_line() 
    {
        let input = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        assert_eq!(5353, decode_line(&input));
    }
}