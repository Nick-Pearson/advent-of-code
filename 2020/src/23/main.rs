fn main() 
{
    let result = play_game(&vec![5,8,6,4,3,9,1,7,2], 100);
    println!("Part 1: {:?}", result);
    let result2 = part_two(&vec![5,8,6,4,3,9,1,7,2]);
    println!("Part 2: {:?} * {:?} = {:?}", result2[0], result2[1], result2[0]*result2[1]);
}

fn get_cup_str(cups: &Vec<usize>, mut offset: usize) -> String
{
    let num_cups = cups.len();

    offset = offset % num_cups;

    let mut s = String::new();
    for _x in 0..num_cups
    {
        let idx = (_x + (num_cups - offset)) % num_cups;
        if 0 == idx
        {
            s = s + &format!("({})", cups[idx]) + " ";
        }
        else
        {
            s = s + &cups[idx].to_string() + " ";
        }
    }
    return s;
}

fn rotate(cups: &mut Vec<usize>, offset: usize)
{
    let num_cups = cups.len();
    for _x in 0..offset
    {
        let tmp = cups[0];
        for _z in 1..num_cups
        {
            cups[_z - 1] = cups[_z];
        }
        cups[num_cups - 1] = tmp;
    }
}

fn play_game(cups: &Vec<usize>, limit: usize) -> Vec<usize>
{
    let num_cups = cups.len();
    let mut result = cups.clone();

    let mut offset = 0;
    while offset < limit
    {
        println!("-- move {} --", offset + 1);
        // println!("cups: {}", get_cup_str(&result, offset));
        let cup = result[0];

        let tmp1 = result.remove(1);
        let tmp2 = result.remove(1);
        let tmp3 = result.remove(1);
        // println!("pick up: {}, {}, {}", tmp1, tmp2, tmp3);
        
        //seach forward
        let mut target_idx = num_cups;
        for y in 0..num_cups
        {
            let target;
            if cup < (2 + y)
            {
                target = num_cups + cup - y - 1;
            }
            else
            {
                target = cup - y - 1;
            }

            let found = result.iter()
                .enumerate()
                .find(|x| *x.1 == target);
            if found.is_some()
            {
                target_idx = found.unwrap().0;
                break
            }
        }
        
        // println!("destination: {}", result[target_idx]);

        let idx = target_idx + 1 % num_cups;
        result.insert(idx, tmp3);
        result.insert(idx, tmp2);
        result.insert(idx, tmp1);
        
        // rotate the cups
        rotate(&mut result, 1);

        offset = offset + 1;
        // println!("");
    }
    rotate(&mut result, num_cups - (offset % num_cups));
    // println!("\n\n-- final --\ncups: {}", get_cup_str(&result, 0));
    return result;
}

fn part_two(cups: &Vec<usize>) -> Vec<usize>
{
    let mut input = vec![0; 1_000_000];
    let num_cups = cups.len();
    for i in 0..1_000_000
    {
        let multi = i / num_cups;
        input[i] = cups[i % num_cups] + (multi * num_cups);
    }

    let result = play_game(&input, 10_000_000);
    return vec![1, 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_move() 
    {
        let result = play_game(&vec![3,8,9,1,2,5,4,6,7], 1);
        assert_eq!(vec![3,2,8,9,1,5,4,6,7], result);
    }

    #[test]
    fn test_one_move_with_underflow() 
    {
        let result = play_game(&vec![2,8,9,1,5,4,6,7,3], 1);
        assert_eq!(vec![2,5,4,6,7,8,9,1,3], result);
    }

    #[test]
    fn test_play_game() 
    {
        let result = play_game(&vec![3,8,9,1,2,5,4,6,7], 10);
        assert_eq!(vec![5,8,3,7,4,1,9,2,6], result);
    }

    // #[test]
    // fn test_part_two() 
    // {
    //     let result = part_two(&vec![3,8,9,1,2,5,4,6,7]);
    //     assert_eq!(vec![934001,159792], result);
    // }
}