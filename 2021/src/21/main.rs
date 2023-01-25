
fn main() {
    println!("Part 1: {}", part_one(8, 4));
    let res = part_two(8, 4, 21);
    println!("Part 2: {:?} => {:?}", res, usize::max(res.0, res.1));
}

fn part_one(p1_start: usize, p2_start: usize) -> usize
{
    let mut players = vec![
        (p1_start - 1, 0),
        (p2_start - 1, 0),
    ];
    let mut dice_rolls = 0;
    let mut other_score = 0;

    loop
    {
        for player in players.iter_mut()
        {
            let dice = dice_rolls % 100;
            let roll = (dice * 3) + 6;
            dice_rolls = dice_rolls + 3;
    
            player.0 = (player.0 + roll) % 10;
            player.1 = player.1 + player.0 + 1;
            if player.1 >= 1000
            {
                return other_score * dice_rolls;
            }
            other_score = player.1;
        }
    }
}


/*
outcomes (27)
3     1
4     3
5     6
6     7
7     6
8     3
9     1
*/
const OUTCOMES:&[(usize,usize)] = &[
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
];

fn player_two_roll(p1: usize, p2: usize, p1_score: usize, p2_score: usize, target: usize) -> (usize, usize)
{
    let mut universes = (0, 0);

    for o in OUTCOMES
    {
        let roll = o.0;
        
        let p2_new = (p2 + roll) % 10;
        let p2_score_new = p2_score + p2_new + 1;
        if p2_score_new >= target
        {
            universes.1 = universes.1 + o.1;
        }
        else
        {
            let outcome = player_one_roll(p1, p2_new, p1_score, p2_score_new, target);
            universes.0 = universes.0 + (outcome.0 * o.1);
            universes.1 = universes.1 + (outcome.1 * o.1);
        }
    }

    // println!("p2 {:?}", universes);
    return universes;
}

fn player_one_roll(p1: usize, p2: usize, p1_score: usize, p2_score: usize, target: usize) -> (usize, usize)
{
    let mut universes = (0, 0);

    for o in OUTCOMES
    {
        let roll = o.0;
        let weight = o.1;
        
        let p1_new = (p1 + roll) % 10;
        let p1_score_new = p1_score + p1_new + 1;
        if p1_score_new >= target
        {
            universes.0 = universes.0 + weight;
        }
        else
        {
            let outcome = player_two_roll(p1_new, p2, p1_score_new, p2_score, target);
            universes.0 = universes.0 + (outcome.0 * weight);
            universes.1 = universes.1 + (outcome.1 * weight);
        }
    }
    // println!("p1 {:?}", universes);
    return universes;
}

fn part_two(p1_start: usize, p2_start: usize, target: usize) -> (usize, usize)
{
    return player_one_roll(p1_start - 1, p2_start - 1, 0, 0, target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        assert_eq!(739785, part_one(4, 8));
    }

    #[test]
    fn test_part_two() 
    {
        assert_eq!((27, 0), part_two(1, 1, 1));
        assert_eq!((53, 26), part_two(1, 1, 5));
        assert_eq!((444356092776315, 341960390180808), part_two(4, 8, 21));
    }
}