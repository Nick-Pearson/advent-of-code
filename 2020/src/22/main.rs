use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() 
{
    if let Ok(lines) = read_lines("src/22/input.txt") {
        let input:Vec<String> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let player_2_start = input.iter().position(|x| x == "Player 2:").unwrap();
        let player_1_deck = Deck::new(input.iter().skip(1).take(player_2_start - 2).map(|x| x.parse::<i64>().unwrap()).collect());
        println!("Player 1: {:?}", player_1_deck);
        let player_2_deck = Deck::new(input.iter().skip(player_2_start + 1).map(|x| x.parse::<i64>().unwrap()).collect());
        println!("Player 2: {:?}", player_2_deck);

        let decks = vec![player_1_deck.clone(), player_2_deck.clone()];
        let res = play_game(&decks);
        println!("Player {:?} wins with total score {:?}", res.0, res.1.score());
        let res2 = play_recursive_combat(&decks);
        println!("Player {:?} wins with total score {:?}", res2.0, res2.1.score());
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: VecDeque<i64>
}

impl Deck
{
    pub fn new(cards: Vec<i64>) -> Deck
    {
        return Deck
        {
            cards: VecDeque::from(cards)
        };
    }

    pub fn next(&mut self) -> i64
    {
        return self.cards.pop_front().unwrap();
    }

    pub fn has_space_to_recurce(&self) -> bool
    {
        let next = *self.cards.get(0).unwrap();
        return next < self.cards.len() as i64;
    }

    pub fn on_winner(&mut self, cards: &Vec<i64>)
    {
        for c in cards
        {
            self.cards.push_back(*c);
        }
    }

    pub fn score(&self) -> i64
    {
        return self.cards.iter()
            .rev()
            .enumerate()
            .map(|x| (x.0 as i64 + 1) * x.1)
            .fold(0, |a, b| a + b);
    }

    pub fn is_empty(&self) -> bool
    {
        return self.cards.is_empty();
    }

    pub fn len(&self) -> i64
    {
        return self.cards.len() as i64;
    }

    pub fn get_string(&self) -> String
    {
        return self.cards.iter()
            .map(|v| v.to_string())
            .fold(String::new(), |a,b| a + "," + &b);
    }

    pub fn get_slice(&self, size: usize) -> Deck
    {
        let mut q = VecDeque::new();
        for i in 0..size
        {
            q.push_back(*self.cards.get(i).unwrap());
        }
        return Deck
        {
            cards: q
        };
    }
}

pub fn play_game(players: &Vec<Deck>) -> (usize, Deck)
{
    let mut game_state:Vec<Deck> = players.clone();

    let mut round = 0;
    let max_rounds = 5000;
    while round < max_rounds
    {
        // println!("Player Decks: {:?}", game_state);
        let cards:Vec<i64> = game_state.iter_mut()
            .filter(|x| !x.is_empty())
            .map(|d| d.next())
            .collect();
        
        // println!("Players play: {:?}", cards);
        let winner = cards.iter().enumerate().max_by_key(|c| c.1).unwrap();
        // println!("Player {} wins with {}", winner.0+1, winner.1);

        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by(|a, b| b.partial_cmp(a).unwrap());
        game_state[winner.0].on_winner(&sorted_cards);
        
        let remaining_players = game_state.iter()
            .filter(|&x| !x.is_empty())
            .count();
        if remaining_players <=1 
        {
            break;
        }

        round = round + 1;
    }
    if round == max_rounds
    {
        panic!("Ran out of rounds, game state: {:?}", game_state);
    }
    return game_state.iter()
        .enumerate()
        .find(|x| !x.1.is_empty())
        .map(|x| (x.0 + 1, x.1.clone()))
        .unwrap();
}

pub fn play_recursive_combat(players: &Vec<Deck>) -> (usize, Deck)
{
    let mut game_state:Vec<Deck> = players.clone();
    let mut seen_states = HashSet::<String>::new();

    let mut round = 0;
    let max_rounds = 50000;
    while round < max_rounds
    {
        // println!("Player Decks: {:?}", game_state);
        let game_state_str = game_state.iter()
            .map(|d| d.get_string())
            .fold(String::new(), |a,b| a + ";" + &b);
        if !seen_states.insert(game_state_str)
        {
            // println!("Seen the state, Player 1 automatically wins");
            return (1, game_state[0].clone());
        }

        let should_recurse = game_state.iter()
            .find(|x| !x.has_space_to_recurce())
            .is_none();
        let cards:Vec<i64> = game_state.iter_mut()
            .filter(|x| !x.is_empty())
            .map(|d| d.next())
            .collect();
        // println!("Players play: {:?}", cards);

        let winner;
        if should_recurse
        {
            // println!("Starting sub-game...");
            let mut sub_game_deck = Vec::new();
            cards.iter()
                .enumerate()
                .map(|x| game_state[x.0].get_slice(*x.1 as usize))
                .for_each(|x| sub_game_deck.push(x));
            let res = play_recursive_combat(&sub_game_deck);
            // println!("Player {} wins sub game", res.0);
            winner = res.0 - 1;
        }
        else
        {
            let res = cards.iter().enumerate().max_by_key(|c| c.1).unwrap();
            // println!("Player {} wins with {}", res.0+1, res.1);
            winner = res.0;
        }
        
        let mut sorted_cards = Vec::new();
        sorted_cards.push(cards[winner]);
        if winner == 0
        {
            sorted_cards.push(cards[1]);
        }
        else
        {
            sorted_cards.push(cards[0]);
        }
        game_state[winner].on_winner(&sorted_cards);
        
        let remaining_players = game_state.iter()
            .filter(|&x| !x.is_empty())
            .count();
        if remaining_players <=1 
        {
            break;
        }

        round = round + 1;
    }
    if round == max_rounds
    {
        panic!("Ran out of rounds, game state: {:?}", game_state);
    }
    return game_state.iter()
        .enumerate()
        .find(|x| !x.1.is_empty())
        .map(|x| (x.0 + 1, x.1.clone()))
        .unwrap();
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
    fn test_play_game() 
    {
        let player_1 = Deck::new(vec![9, 2, 6, 3, 1]);
        let player_2 = Deck::new(vec![5, 8, 4, 7, 10]);

        let result = play_game(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1], res);
    }

    #[test]
    fn test_play_recursive_combat1() 
    {
        let player_1 = Deck::new(vec![9, 2, 6, 3, 1]);
        let player_2 = Deck::new(vec![5, 8, 4, 7, 10]);

        let result = play_recursive_combat(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3], res);
    }

    #[test]
    fn test_play_recursive_combat2() 
    {
        let player_1 = Deck::new(vec![9, 8, 5, 2]);
        let player_2 = Deck::new(vec![10, 1, 7]);

        let result = play_recursive_combat(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![5, 10, 2, 9, 8, 7, 1], res);
    }

    #[test]
    fn test_play_recursive_combat3() 
    {
        let player_1 = Deck::new(vec![8, 1]);
        let player_2 = Deck::new(vec![3, 4, 10, 9, 7, 5]);

        let result = play_recursive_combat(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![7, 5, 4, 1, 10, 8, 9, 3], res);
    }

    #[test]
    fn test_play_recursive_combat4() 
    {
        let player_1 = Deck::new(vec![8]);
        let player_2 = Deck::new(vec![10, 9, 7, 5]);

        let result = play_recursive_combat(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![9, 7, 5, 10, 8], res);
    }

    #[test]
    fn test_play_recursive_combat5() 
    {
        let player_1 = Deck::new(vec![8]);
        let player_2 = Deck::new(vec![10, 9, 7, 5]);

        let result = play_recursive_combat(&vec![player_1, player_2]);
        assert_eq!(2, result.0);
        let res:Vec<i64> = result.1.cards.iter().copied().collect();
        assert_eq!(vec![9, 7, 5, 10, 8], res);
    }

    #[test]
    fn test_score_deck() 
    {
        let deck = Deck::new(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);

        assert_eq!(306, deck.score());
    }

    #[test]
    fn test_score_deck2() 
    {
        let deck = Deck::new(vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);

        assert_eq!(291, deck.score());
    }
}