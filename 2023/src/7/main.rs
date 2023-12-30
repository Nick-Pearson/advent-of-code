use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    character::complete::{anychar, digit1, multispace0, multispace1},
    combinator::{map, recognize},
    multi::{count, many1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum HandKind {
    FiveOfAKind(u8),
    FourOfAKind(u8),
    FullHouse(u8, u8),
    ThreeOfAKind(u8),
    TwoPair(u8, u8),
    OnePair(u8),
    HighCard(u8),
}

#[derive(Debug, Clone)]
struct Hand {
    kind: HandKind,
    cards: [u8; 5],
    bid: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let (_, mut hands) = parse(input).unwrap();
    hands.sort_by(compare_hands);

    let part_one: usize = hands
        .iter()
        .enumerate()
        .map(|t| (t.0 + 1) * t.1.bid as usize)
        .sum();
    println!("Part one: {}", part_one);

    let mut hands2 = convert_jokers_in_hands(&hands);
    hands2.sort_by(compare_hands);
    let part_two: usize = hands2
        .iter()
        .enumerate()
        .map(|t| (t.0 + 1) * t.1.bid as usize)
        .sum();
    println!("Part two: {}", part_two);
}

fn convert_jokers_in_hands(hands: &[Hand]) -> Vec<Hand> {
    hands.iter().map(convert_jokers).collect()
}

fn convert_jokers(hand: &Hand) -> Hand {
    let mut cards = hand.cards.clone();
    for i in 0..5 {
        if cards[i] == 11 {
            cards[i] = 0;
        }
    }
    Hand {
        kind: classify_hand(&cards),
        cards,
        bid: hand.bid,
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    compare_kinds(&a.kind, &b.kind).then_with(|| compare_cards(&a.cards, &b.cards))
}

fn compare_kinds(akind: &HandKind, bkind: &HandKind) -> Ordering {
    let f = |k: &HandKind| match k {
        HandKind::FiveOfAKind(_) => 7,
        HandKind::FourOfAKind(_) => 6,
        HandKind::FullHouse(_, _) => 5,
        HandKind::ThreeOfAKind(_) => 4,
        HandKind::TwoPair(_, _) => 3,
        HandKind::OnePair(_) => 2,
        HandKind::HighCard(_) => 1,
    };
    f(akind).partial_cmp(&f(bkind)).unwrap()
}

fn compare_cards(ahand: &[u8; 5], bhand: &[u8; 5]) -> Ordering {
    for i in 0..5 {
        let a = ahand[i];
        let b = bhand[i];
        let ord = a.partial_cmp(&b).unwrap();
        match ord {
            Ordering::Equal => {}
            _ => return ord,
        }
    }
    Ordering::Equal
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    let (_, hand_input) = many1(tuple((
        count(anychar, 5),
        delimited(multispace1, parse_int, multispace0),
    )))(input)?;

    Ok((
        input,
        hand_input
            .into_iter()
            .map(|t| (convert_to_card_ids(t.0), t.1))
            .map(|t| Hand {
                kind: classify_hand(&t.0),
                cards: t.0,
                bid: t.1,
            })
            .collect(),
    ))
}

fn classify_hand(card_ids: &[u8; 5]) -> HandKind {
    let freqs = count_frequencies(card_ids);
    let mut kind = classify_from_freqs(&freqs);

    if let Some(permuation) = joker_permutation(&freqs) {
        let joker_kind = classify_from_freqs(&permuation);
        if compare_kinds(&joker_kind, &kind).is_gt() {
            kind = joker_kind;
        }
    }

    kind
}

fn joker_permutation(freqs: &[(u8, usize)]) -> Option<Vec<(u8, usize)>> {
    let maybe_jokers = freqs.iter().find(|f| f.0 == 0);
    match maybe_jokers {
        Some(jokers) => {
            let mut copy: Vec<(u8, usize)> = freqs.iter().filter(|p| p.0 != 0).cloned().collect();
            if copy.len() == 0 {
                return None;
            }

            copy[0].1 = copy[0].1 + jokers.1;
            Some(copy)
        }
        None => None,
    }
}

fn classify_from_freqs(freqs: &Vec<(u8, usize)>) -> HandKind {
    let highest_freq = freqs[0].1;
    match highest_freq {
        1 => HandKind::HighCard(freqs.iter().map(|f| f.0).max().unwrap()),
        2 => {
            let second_highest = freqs[1].1;
            if second_highest == 2 {
                HandKind::TwoPair(freqs[0].0, freqs[1].0)
            } else {
                HandKind::OnePair(freqs[0].0)
            }
        }
        3 => {
            let second_highest = freqs[1].1;
            if second_highest == 2 {
                HandKind::FullHouse(freqs[0].0, freqs[1].0)
            } else {
                HandKind::ThreeOfAKind(freqs[0].0)
            }
        }
        4 => HandKind::FourOfAKind(freqs[0].0),
        5 => HandKind::FiveOfAKind(freqs[0].0),
        _ => panic!("failed to identify hand type from: {:?}", freqs),
    }
}

fn count_frequencies(card_ids: &[u8; 5]) -> Vec<(u8, usize)> {
    let mut freq = [0; 16];

    for card in card_ids {
        let idx = *card as usize;
        freq[idx] = freq[idx] + 1;
    }

    freq.iter()
        .enumerate()
        .filter(|f| *f.1 > 0)
        .map(|f| (f.0 as u8, *f.1))
        .sorted_by_key(|f| 100 - f.1)
        .collect()
}

fn convert_to_card_ids(cards: Vec<char>) -> [u8; 5] {
    let mut ids = [0; 5];
    for i in 0..5 {
        ids[i] = card_id(cards[i])
    }
    ids
}

fn card_id(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => card as u8 - '0' as u8,
        _ => panic!("unexpected card {}", card),
    }
}

fn parse_int(l: &str) -> IResult<&str, i32> {
    map(recognize(digit1), |o: &str| o.parse::<i32>().unwrap())(l)
}
