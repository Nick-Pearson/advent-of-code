use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Rule {
    min: i32,
    max: i32
}

#[derive(Debug)]
pub struct Field {
    name: String,
    rules: Vec<Rule>
}


fn main() 
{
    if let Ok(lines) = read_lines("src/16/input.txt") {
        let input = &mut lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        
        let fields = get_fields(input);
        let my_ticket = parse_ticket(&input.skip(1).next().unwrap());
        let tickets:Vec<_> = input.skip(2).map(|s| parse_ticket(&s)).collect();

        let error_rate = calc_error_rate(&tickets, &fields);
        println!("Error Rate: {}", error_rate);

        let valid_tickets = tickets.iter()
            .filter(|t| find_invalid_value(t, &fields).is_none())
            .cloned()
            .collect();
        let field_order = find_field_order(&valid_tickets, &fields);
        println!("My Ticket:");
        for i in 0..field_order.len()
        {
            println!("{}: {}", field_order[i], my_ticket[i]);
        }
        let dep_result:i64 = field_order.iter()
            .enumerate()
            .filter(|f| f.1.len() > 8)
            .filter(|f| &f.1[..9] == "departure")
            .map(|f| my_ticket[f.0] as i64)
            .product();
        println!("Departure Product: {}", dep_result);
    }
}

fn get_fields(input: &mut dyn Iterator<Item = String>) -> Vec<Field>
{
    let mut fields = Vec::new();
    for line in input
    {
        if line == ""
        {
            break;
        }
        fields.push(parse_field(&line));
    }
    return fields;
}

fn parse_field(input: &String) -> Field
{
    let mut split = input.split(": ");
    return Field{
        name: split.next().unwrap().to_string(),
        rules: split.next().unwrap().split(" or ").map(|s| parse_rule(s)).collect()
    };
}

fn parse_rule(input: &str) -> Rule
{
    let mut split = input.split('-');
    return Rule{
        min: split.next().map_or(0, |s| s.parse::<i32>().unwrap()),
        max: split.next().map_or(0, |s| s.parse::<i32>().unwrap()),
    };
}

fn parse_ticket(input: &String) -> Vec<i32>
{
    return input.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
}

pub fn find_invalid_value(ticket: &Vec<i32>, fields: &Vec<Field>) -> Option<i32>
{
    return ticket.iter()
        .copied()
        .find(|v| !matches_any_field(*v, fields));
}

fn matches_any_field(input: i32, fields: &Vec<Field>) -> bool
{
    for field in fields
    {
        if matches_field(input, field)
        {
            return true;
        }
    }
    return false;
}

fn matches_field(input: i32, field: &Field) -> bool
{
    for rule in &field.rules
    {
        if input >= rule.min && input <= rule.max
        {
            return true;
        }
    }
    return false;
}

pub fn calc_error_rate(tickets: &Vec<Vec<i32>>, fields: &Vec<Field>) -> i32
{
    return tickets.iter()
        .filter_map(|t| find_invalid_value(t, fields))
        .sum();
}

pub fn find_field_order(tickets: &Vec<Vec<i32>>, fields: &Vec<Field>) -> Vec<String>
{
    return find_field_order_i(0, tickets, fields, &Vec::new()).unwrap();
}

pub fn find_field_order_i(start_idx : usize, tickets: &Vec<Vec<i32>>, fields: &Vec<Field>, ignore_list: &Vec<usize>) -> Option<Vec<String>>
{
    let possibles:Vec<(usize, &Field)> = fields.iter()
        .enumerate()
        .filter(|f| !ignore_list.contains(&f.0))
        .filter(|f| matches_all_at(start_idx, tickets, f.1))
        .collect();

    if possibles.len() == 0
    {
        return None;
    }
    else if start_idx + 1 == tickets[0].len()
    {
        if possibles.len() > 1
        {
            panic!("Ambiguous input");
        }
        return Some(vec![possibles[0].1.name.clone()]);
    }
    for possible in possibles
    {
        let mut l = ignore_list.to_vec();
        l.push(possible.0);
        let result = find_field_order_i(start_idx+1, tickets, fields, &l);
        if result.is_some()
        {
            let mut r = Vec::new();
            r.push(possible.1.name.clone());
            r.append(&mut result.unwrap());
            return Some(r);
        }
    }
    return None;
}

fn matches_all_at(i : usize, tickets: &Vec<Vec<i32>>, field: &Field) -> bool
{
    return tickets.iter()
        .all(|t| matches_field(t[i], field));
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
    fn test_find_invalid_value() 
    {
        let rules = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50"
        ].iter().map(|s| parse_field(&s.to_string())).collect();

        assert_eq!(None, find_invalid_value(&vec![7,3,47], &rules));
        assert_eq!(Some(4), find_invalid_value(&vec![40,4,50], &rules));
        assert_eq!(Some(55), find_invalid_value(&vec![55,2,20], &rules));
        assert_eq!(Some(12), find_invalid_value(&vec![38,6,12], &rules));
    }

    #[test]
    fn test_calc_error_rate() 
    {
        let rules = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50"
        ].iter().map(|s| parse_field(&s.to_string())).collect();
        let tickets = vec![
            vec![7,3,47],
            vec![40,4,50],
            vec![55,2,20],
            vec![38,6,12]
        ];

        assert_eq!(71, calc_error_rate(&tickets, &rules));
    }

    #[test]
    fn test_find_field_order() 
    {
        let rules = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19"
        ].iter().map(|s| parse_field(&s.to_string())).collect();
        let tickets = vec![
            vec![3,9,18],
            vec![15,1,5],
            vec![5,14,9]
        ];

        let expected = vec![
            "row",
            "class",
            "seat"
        ];
        assert_eq!(expected, find_field_order(&tickets, &rules));
    }
}