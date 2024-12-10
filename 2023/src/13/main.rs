use nom::{
    branch::alt,
    character::complete::{char, line_ending, newline},
    combinator::{map, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum ItemKind {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    rows: Vec<Vec<ItemKind>>,
}

fn main() {
    let input = include_str!("input.txt");
    let (_, patterns) = parse(input).unwrap();
}

fn find_horizontal_symetry(pattern: &Pattern) -> Option<usize> {
    let width = pattern.rows[0].len();
    None
}

fn find_vertical_symetry(pattern: &Pattern) -> Option<usize> {
    let height = pattern.rows.len();
    for i in 1..height-1 {
        
    }
    None
}
fn parse(input: &str) -> IResult<&str, Vec<Pattern>> {
    many1(terminated(parse_pattern, opt(line_ending)))(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(many1(terminated(parse_items, opt(line_ending))), |rows| {
        Pattern { rows }
    })(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<ItemKind>> {
    many1(map(alt((char('.'), char('#'))), |c| match c {
        '.' => ItemKind::Ash,
        '#' => ItemKind::Rock,
        _ => panic!("unexpected char {}", c),
    }))(input)
}
