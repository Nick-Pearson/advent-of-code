use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct VMState {
    mask_bits: u64,
    mask_data: u64,
    memory: Vec<(u64, u64)>
}

impl VMState {
    fn new() -> VMState {
        return VMState {
            mask_bits: u64::MAX,
            mask_data: 0,
            memory: Vec::new()
        };
    }

    #[allow(dead_code)]
    fn get_mem(&self, addr: u64) -> u64
    {
        return self.memory.iter()
            .find(|x| x.0 == addr)
            .map(|x| x.1)
            .unwrap_or(0);
    }

    fn sum_mem(&self) -> u64
    {
        return self.memory.iter()
            .map(|x| x.1)
            .sum();
    }

    fn set_mem(&mut self, addr: u64, val: u64)
    {
        let mem = self.memory.iter_mut()
            .find(|x| x.0 == addr);
        if mem.is_some()
        {
            mem.unwrap().1 = val;
        }
        else
        {
            self.memory.push((addr, val));
        }
    }

    fn mask_value(&self, val: u64) -> u64
    {
        return (val & self.mask_bits) | self.mask_data;
    }

    fn mask_addr(&self, addr: u64) -> Vec<u64>
    {
        let base_addr = (addr | self.mask_data) & !self.mask_bits;
        let mut results = Vec::new();
        results.push(base_addr);

        for i in 0..36
        {
            let m = 1 << i;
            if self.mask_bits & m > 0
            {
                let len = results.len();
                for j in 0..len
                {
                    results.push(results[j] | m);
                }
            }
        }

        return results;
    }

    fn set_mask(&mut self, mask: &str)
    {
        self.mask_bits = mask.chars()
            .enumerate()
            .filter(|x| x.1 == 'X')
            .map(|x| 1 << (35 - x.0))
            .sum();
        self.mask_data = mask.chars()
            .enumerate()
            .filter(|x| x.1 == '1')
            .map(|x| 1 << (35 - x.0))
            .sum();
    }

    #[allow(dead_code)]
    fn debug_print(&self)
    {
        println!("=== VM STATE ===");
        println!("Mask bits: {}", format!("{:036b}", self.mask_bits));
        println!("Mask data: {}", format!("{:036b}", self.mask_data));
        println!("Memory [");
        for m in self.memory.iter()
        {
            println!(" {}: {}", m.0, m.1);
        }
        println!("]");
    }
}

fn main() 
{
    if let Ok(lines) = read_lines("src/14/input.txt") {
        let program = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let state1 = run_program(&program, 1);
        println!("Memory Sum 1: {}", state1.sum_mem());
        let state2 = run_program(&program, 2);
        println!("Memory Sum 2: {}", state2.sum_mem());
    }
}

pub fn run_program(input: &Vec<String>, version: i32) -> VMState
{
    let mut state = VMState::new();
    for cmd in input
    {
        if &cmd[0..4] == "mask"
        {
            let val = &cmd[7..];
            state.set_mask(val);
        }
        else if &cmd[0..3] == "mem"
        {
            let addr = cmd.split(']').next().unwrap()[4..].parse::<u64>().unwrap();
            let val = cmd.split('=').skip(1).next().unwrap()[1..].parse::<u64>().unwrap();
            if version == 1
            {
                state.set_mem(addr, state.mask_value(val));
            }
            else
            {
                let addrs = state.mask_addr(addr);
                for a in addrs
                {
                    state.set_mem(a, val);
                }
            }
        }
        else
        {
            panic!("Unrecognised command {:?}", cmd);
        }
    }
    return state;
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
    fn test_get_set_mem() 
    {
        let mut state = VMState::new();
        assert_eq!(0, state.get_mem(100));
        state.set_mem(100, 512);
        assert_eq!(512, state.get_mem(100));
        state.set_mem(100, 1024);
        assert_eq!(1024, state.get_mem(100));
    }

    #[test]
    fn test_run_program() 
    {
        let input = vec![
            String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            String::from("mem[8] = 11"),
            String::from("mem[7] = 101"),
            String::from("mem[8] = 0")
        ];
        let state = run_program(&input, 1);
        assert_eq!(101, state.get_mem(7));
        assert_eq!(64, state.get_mem(8));
        assert_eq!(165, state.sum_mem());
    }

    #[test]
    fn test_run_program_v2() 
    {
        let input = vec![
            String::from("mask = 000000000000000000000000000000X1001X"),
            String::from("mem[42] = 100"),
            String::from("mask = 00000000000000000000000000000000X0XX"),
            String::from("mem[26] = 1")
        ];
        let state = run_program(&input, 2);
        assert_eq!(100, state.get_mem(58));
        assert_eq!(100, state.get_mem(59));
        assert_eq!(1, state.get_mem(16));
        assert_eq!(1, state.get_mem(17));
        assert_eq!(1, state.get_mem(18));
        assert_eq!(1, state.get_mem(19));
        assert_eq!(1, state.get_mem(24));
        assert_eq!(1, state.get_mem(25));
        assert_eq!(1, state.get_mem(26));
        assert_eq!(1, state.get_mem(27));
    }
}