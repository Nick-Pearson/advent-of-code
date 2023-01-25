
fn main() 
{
    let input = vec![17,1,3,16,19,0];
    println!("2020th #: {}", get_nth_number(&input, 2020));
    println!("30000000th #: {}", get_nth_number(&input, 30000000));
}

pub fn get_nth_number(input: &Vec<usize>, n: usize) -> usize
{
    let mut seen = vec![0; 100_000_000].into_boxed_slice();

    let ilen = input.len();
    let mut prev = input[0];
    for i in 1..n
    {
        let next;
        if ilen > i
        {
            next = input[i];
        }
        else
        {
            next = get_next(&seen, prev, i);
        }
        seen[prev] = i;
        prev = next;
    }
    return prev;
}

pub fn get_next(seen: &[usize], prev: usize, turn: usize) -> usize
{
    let idx = seen[prev];
    if idx != 0
    {
        return turn - idx;
    }
    else
    {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_number() 
    {
        assert_eq!(0, get_nth_number(&vec![0,3,6], 1));
        assert_eq!(3, get_nth_number(&vec![0,3,6], 2));
        assert_eq!(6, get_nth_number(&vec![0,3,6], 3));
        assert_eq!(0, get_nth_number(&vec![0,3,6], 4));
        assert_eq!(3, get_nth_number(&vec![0,3,6], 5));
        assert_eq!(3, get_nth_number(&vec![0,3,6], 6));
        assert_eq!(1, get_nth_number(&vec![0,3,6], 7));
        assert_eq!(0, get_nth_number(&vec![0,3,6], 8));
        assert_eq!(4, get_nth_number(&vec![0,3,6], 9));
        assert_eq!(0, get_nth_number(&vec![0,3,6], 10));
        assert_eq!(436, get_nth_number(&vec![0,3,6], 2020));
        assert_eq!(1, get_nth_number(&vec![1,3,2], 2020));
        assert_eq!(10, get_nth_number(&vec![2,1,3], 2020));
        assert_eq!(27, get_nth_number(&vec![1,2,3], 2020));
        assert_eq!(78, get_nth_number(&vec![2,3,1], 2020));
        assert_eq!(438, get_nth_number(&vec![3,2,1], 2020));
        assert_eq!(1836, get_nth_number(&vec![3,1,2], 2020));
    }

    // #[test]
    // fn test_get_nth_number_big() 
    // {
    //     assert_eq!(175594, get_nth_number(&vec![0,3,6], 30000000));
    //     // assert_eq!(2578, get_nth_number(&vec![1,3,2], 30000000));
    //     // assert_eq!(3544142, get_nth_number(&vec![2,1,3], 30000000));
    //     // assert_eq!(261214, get_nth_number(&vec![1,2,3], 30000000));
    //     // assert_eq!(6895259, get_nth_number(&vec![2,3,1], 30000000));
    //     // assert_eq!(18, get_nth_number(&vec![3,2,1], 30000000));
    //     // assert_eq!(362, get_nth_number(&vec![3,1,2], 30000000));
    // }
}