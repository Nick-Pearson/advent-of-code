use std::process::id;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, multispace0},
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use nom::sequence::separated_pair;

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    winners: Vec<i32>,
    card: Vec<i32>,
}


fn main() {
    let input = include_str!("input.txt");
    let games: Vec<Game> = input.lines().map(|l| parse_game(l).unwrap().1).collect();

    let part_one:i32 = games.iter()
        .map(|g| calculate_score(g))
        .sum();
    println!("Part one: {}", part_one);
    let part_two:i32 = play_game(&games).iter()
        .map(|g| g.1)
        .sum();
    println!("Part two: {}", part_two);
}

fn play_game(games: &Vec<Game>) -> Vec<(Game, i32)> {
    let mut game_instances:Vec<(Game, i32)> = games.iter().cloned()
        .map(|g| (g, 1))
        .collect();

    for i in 0..game_instances.len() {
        let gi = &game_instances[i];
        let matches = num_matches(&gi.0);
        let current_copies = gi.1;

        for j in 0..matches {
            let idx = i + j + 1;
            game_instances[idx].1 = game_instances[idx].1 + current_copies;
        }
    }
    game_instances
}

fn num_matches(game: &Game) -> usize {
    game.card.iter()
        .filter(|c| game.winners.contains(c))
        .count()
}

fn calculate_score(game: &Game) -> i32 {
    match num_matches(game) {
        0 => 0,
        1 => 1,
        x => 2 << x - 2
    }
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (games, id) = terminated(
        preceded(
            tuple((tag("Card"), multispace1)),
            map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap()),
        ),
        tuple((tag(":"), multispace1)),
    )(i)?;
    let (_, (winners, card)) = separated_pair(
        many1(terminated(
            map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap()),
            multispace1,
        )),
        tag("|"),
        many1(delimited(
            multispace0,
            map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap()),
            multispace0,
        )),
    )(games)?;
    
    Ok((i, Game {
        id,
        winners,
        card,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let games: Vec<Game> = input.lines().map(|l| parse_game(l).unwrap().1).collect();

        let part_one:i32 = games.iter()
            .map(|g| calculate_score(g))
            .sum();
        assert_eq!(13, part_one)
    }
    #[test]
    fn test_part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let games: Vec<Game> = input.lines().map(|l| parse_game(l).unwrap().1).collect();

        let part_two:i32 = play_game(&games).iter()
            .map(|g| g.1)
            .sum();
        assert_eq!(30, part_two)
    }
}
