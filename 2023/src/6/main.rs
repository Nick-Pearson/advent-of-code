use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Race {
    duration: u64,
    record: u64,
}

fn main() {
    let input = include_str!("input.txt");
    let (_, races) = parse(input).unwrap();
    let part_one: usize = races.iter().map(|r| num_winning_combinations(r)).product();
    println!("Part one: {}", part_one);
    let (_, race2) = parse2(input).unwrap();
    let part_two = num_winning_combinations(&race2);
    println!("Part two: {}", part_two);
}

fn num_winning_combinations(race: &Race) -> usize {
    (1..race.duration)
        .map(|h| calculate_distance(h, race.duration))
        .filter(|d| *d > race.record)
        .count()
}

fn calculate_distance(hold_time: u64, total_time: u64) -> u64 {
    hold_time * (total_time - hold_time)
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (_, (durations, records)) = tuple((
        preceded(
            tag("Time:"),
            many1(delimited(multispace0, parse_int, multispace0)),
        ),
        preceded(
            tag("Distance:"),
            many1(delimited(multispace0, parse_int, multispace0)),
        ),
    ))(input)?;

    Ok((
        input,
        durations
            .iter()
            .zip_eq(records)
            .map(|pair| Race {
                duration: *pair.0,
                record: pair.1,
            })
            .collect(),
    ))
}

fn parse2(input: &str) -> IResult<&str, Race> {
    let (_, (duration, record)) = tuple((
        preceded(tag("Time:"), parse_spaced_int),
        preceded(tag("Distance:"), parse_spaced_int),
    ))(input)?;

    Ok((input, Race { duration, record }))
}

fn parse_spaced_int(l: &str) -> IResult<&str, u64> {
    map(
        fold_many1(
            delimited(multispace0, recognize(digit1), multispace0),
            || String::new(),
            |a, i| a + i,
        ),
        |o: String| o.parse::<u64>().unwrap(),
    )(l)
}

fn parse_int(l: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |o: &str| o.parse::<u64>().unwrap())(l)
}
