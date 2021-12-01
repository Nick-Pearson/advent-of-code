extern crate num;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

fn main() 
{
    if let Ok(lines) = read_lines("src/13/input.txt") {
        let mut it = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap());
        let timestamp = it.next().unwrap().parse::<i64>().unwrap();
        let buses_with_zeros = get_buses(&it.next().unwrap());
        let buses = remove_indexes(&buses_with_zeros);

        let t = first_time(timestamp, &buses);
        let diff = t.1 - timestamp;
        println!("Found: {:?} in {:?} ({:?})", t.0, diff, t.1);
        println!("Result: {:?}", diff*t.0);
        let best = find_perfect_time(&buses_with_zeros);
        println!("Result 2: {:?}", best);
    }
}

fn first_time(start: i64, buses: &Vec<i64>) -> (i64, i64)
{
    let mut time = start;
    loop
    {
        let bus = buses.iter()
            .find(|x| (time % *x) == 0);
        if bus.is_some()
        {
            return (*bus.unwrap(), time);
        }
        time = time + 1;
    }
}

fn find_perfect_time(buses: &Vec<(i64, i64)>) -> i64
{
    let now = Instant::now();
    let mut time = 0;
    let mut step = 1;
    for bus in buses
    {
        while (time + bus.0) % bus.1 != 0
        {
            time = time + step;
        }
        step = step * bus.1;
    }
    let elapsed = now.elapsed();
    println!("Found answer in {}.{}s", elapsed.as_secs(), elapsed.subsec_millis());
    return time;
}

fn get_buses(input: &String) -> Vec<(i64, i64)>
{
    return input.split(',')
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.0 as i64, x.1.parse::<i64>().unwrap()))
        .collect();
}

fn remove_indexes(input: &Vec<(i64, i64)>) -> Vec<i64>
{
    return input.iter()
        .map(|x| x.1)
        .collect();
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
    fn test_find_perfect_time() 
    {
        assert_eq!(1068781, find_perfect_time(&get_buses(&String::from("7,13,x,x,59,x,31,19"))));
        assert_eq!(3417, find_perfect_time(&get_buses(&String::from("17,x,13,19"))));
        assert_eq!(754018, find_perfect_time(&get_buses(&String::from("67,7,59,61"))));
        assert_eq!(779210, find_perfect_time(&get_buses(&String::from("67,x,7,59,61"))));
        assert_eq!(1261476, find_perfect_time(&get_buses(&String::from("67,7,x,59,61"))));
        assert_eq!(1202161486, find_perfect_time(&get_buses(&String::from("1789,37,47,1889"))));
    }
}