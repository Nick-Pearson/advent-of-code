
use num_integer::binomial;

fn main() {
    //target area: x=156..202, y=-110..-69
    println!("Part 1: {}", part_one(-110));
    println!("Part 2: {}", part_two(202, 156, -69, -110));
}

fn part_one(min_y: i64) -> i64
{
    return binomial(-min_y, 2)
}

fn part_two(max_x: i64, min_x: i64, max_y: i64, min_y: i64) -> usize
{
    let mut vals = Vec::new();
    let max = i64::max(max_x, -max_y);
    for step in 1..=max*2
    {
        let xs = get_possible_x_values(max_x, min_x, step);
        let ys = get_possible_y_values(max_y, min_y, step);

        // println!("{} => {:?}, {:?}", step, xs, ys);
        for x in xs
        {
            let mut a:Vec<(i64,i64)> = ys.iter()
                .map(|y| (x,*y))
                .collect();
            vals.append(&mut a);
        }
    }

    vals.sort();
    vals.dedup();
    // println!("final :\n {:?}", vals);
    return vals.len();
}

fn get_possible_x_values(max: i64, min: i64, step: i64) -> Vec<i64>
{
    let mut vals = Vec::new();
    for x in 1..=max
    {
        let mut v = 0;
        for s in 0..i64::min(step,x)
        {
            v = v + (x - s);
        }
        if v >= min && v <= max
        {
            vals.push(x);
        }
    }
    return vals;
}

fn get_possible_y_values(max: i64, min: i64, step: i64) -> Vec<i64>
{
    let mut vals = Vec::new();
    for y in min..=-min
    {
        let mut v = 0;
        for s in 0..step
        {
            v = v + (y - s);
        }
        if v >= min && v <= max
        {
            vals.push(y);
        }
    }
    return vals;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() 
    {
        assert_eq!(45, part_one(-10));
    }

    #[test]
    fn test_part_two() 
    {
        assert_eq!(112, part_two(30, 20, -5, -10));
    }
}