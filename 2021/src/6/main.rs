use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/6/input.txt") {
        let init:Vec<i32> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| r.split(',').map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .flatten()
            .collect();

        println!("80 Days: {}", simulate_fish(&init, 80));
        println!("256 Days: {}", simulate_fish(&init, 256));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn simulate_fish(initial: &Vec<i32>, days: usize) -> usize
{
    let mut state:Vec<usize> = vec![0; 9];
    for i in initial
    {
        let idx = *i as usize;
        state[idx] = state[idx] + 1;
    }

    for _day in 0..days
    {
        let fish_born = state[0];

        for i in 0..8
        {
            state[i] = state[i+1];
        }

        state[6] = state[6] + fish_born;
        state[8] = fish_born;
    }

    return state.iter().fold(0, |a,b| a + b);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_fish() 
    {
        let input = vec![3,4,3,1,2];
        assert_eq!(26, simulate_fish(&input, 18));
        assert_eq!(5934, simulate_fish(&input, 80));
        assert_eq!(26984457539, simulate_fish(&input, 256));
    }

}