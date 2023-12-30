use nom::{
    character::complete::{digit1, i64, multispace0, space0},
    combinator::{map, recognize},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Clone)]
struct History {
    values: Vec<i64>,
}

fn main() {
    let input = include_str!("input.txt");
    let (_, histories) = parse(input).unwrap();
    let part_one: i64 = histories.iter().map(predict_forward).sum();
    println!("Part one: {}", part_one);
    let part_two: i64 = histories.iter().map(predict_back).sum();
    println!("Part two: {}", part_two);
}

fn predict_forward(history: &History) -> i64 {
    predict(&history.values)
}

fn predict_back(history: &History) -> i64 {
    let mut reversed = history.values.clone();
    reversed.reverse();
    predict(&reversed)
}

fn predict(values: &Vec<i64>) -> i64 {
    let mut sum = values.last().copied().unwrap();
    let mut previous_row = values.clone();

    while !previous_row.iter().all(|d| *d == 0) {
        let differences = calculate_differences(&previous_row);
        sum = sum + differences.last().unwrap();
        previous_row = differences;
    }

    sum
}

fn calculate_differences(row: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(row.len() - 1);
    for i in 0..row.len() - 1 {
        result.push(row[i + 1] - row[i]);
    }
    result
}

fn parse(input: &str) -> IResult<&str, Vec<History>> {
    many1(terminated(parse_history, multispace0))(input)
}

fn parse_history(input: &str) -> IResult<&str, History> {
    map(many1(preceded(space0, i64)), |values| History { values })(input)
}
