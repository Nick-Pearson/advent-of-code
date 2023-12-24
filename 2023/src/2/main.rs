use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, recognize},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    pub fn has_enough(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn power(&self) -> usize {
        self.red as usize * self.green as usize * self.blue as usize
    }

    pub fn max(&self, other: &Self) -> Self {
        Cubes {
            red: i32::max(self.red, other.red),
            green: i32::max(self.green, other.green),
            blue: i32::max(self.blue, other.blue),
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    game_id: i32,
    rounds: Vec<Cubes>,
}

fn main() {
    let input = include_str!("input.txt");
    let games: Vec<Game> = input.lines().map(|l| parse_game(l).unwrap().1).collect();

    let part_one: i32 = games
        .iter()
        .filter(|g| {
            is_game_possible(
                g,
                Cubes {
                    red: 12,
                    green: 13,
                    blue: 14,
                },
            )
        })
        .map(|g| g.game_id)
        .sum();
    println!("Part one: {}", part_one);
    let part_two: usize = games.iter().map(|g| game_power(g)).sum();
    println!("Part two: {}", part_two);
}

fn game_power(game: &Game) -> usize {
    game.rounds
        .iter()
        .cloned()
        .reduce(|a, e| a.max(&e))
        .map_or(0, |c| c.power())
}

fn is_game_possible(game: &Game, total_cubes: Cubes) -> bool {
    game.rounds
        .iter()
        .find(|r| !total_cubes.has_enough(r))
        .is_none()
}

fn parse_cubes(i: &str) -> IResult<&str, Cubes> {
    let parts = i.split(", ");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in parts {
        let (_, (amount, _, colour)) = tuple((
            map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap()),
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ))(part)?;

        match colour {
            "red" => red = red + amount,
            "green" => green = green + amount,
            "blue" => blue = blue + amount,
            _ => panic!("invalid branch"),
        }
    }
    Ok((i, Cubes { red, green, blue }))
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (games, game_id) = terminated(
        preceded(
            tag("Game "),
            map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap()),
        ),
        tag(": "),
    )(i)?;

    let mut rounds = Vec::new();
    for r in games.split("; ") {
        rounds.push(parse_cubes(r)?.1);
    }
    Ok((i, Game { game_id, rounds }))
}
