use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
pub struct VMState {
    acc: i32,
    pc: i32,
    visited: HashSet<i32>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Instruction {
    op_code: String,
    arg: i32
}


impl Instruction {
    fn from(o: &str, a: i32) -> Instruction {
        return Instruction {
            op_code: String::from(o),
            arg: a
        };
    }
}

impl VMState {
    fn new() -> VMState {
        return VMState {
            acc: 0,
            pc: 0,
            visited: HashSet::new()
        };
    }

    fn process(&mut self, ins: &Instruction)
    {
        if ins.op_code == "jmp"
        {
            self.pc = self.pc + (ins.arg - 1);
        }
        else if ins.op_code == "acc"
        {
            self.acc = self.acc + ins.arg;
        }
        self.pc = self.pc + 1;
    }

    fn has_visited(&mut self) -> bool
    {
        return !self.visited.insert(self.pc);
    }
}

fn main() {
    if let Ok(lines) = read_lines("src/8/input.txt") {
        let it:Vec<Instruction> = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|r| parse_instruction(&r))
            .collect();
        let broken = execute_until_duplicate(&it);
        println!("Broken Result: {:?}", broken.acc);
        let fixed = fix_and_run_program(&it);
        println!("Fixed Result: {:?}", fixed.acc);
    }
}

pub fn execute_until_duplicate(program: &Vec<Instruction>) -> VMState
{
    let mut state = VMState::new();
    let len = program.len() as i32;
    while state.pc != len
    {
        if state.has_visited()
        {
            break;
        }

        state.process(&program[state.pc as usize]);
    }
    return state;
}

pub fn fix_and_run_program(program: &Vec<Instruction>) -> VMState
{
    let looping_result = execute_until_duplicate(program);
    
    let result = find_back(program, &looping_result.visited, program.len() as i32).unwrap();

    let mut program_copy = program.to_vec();
    program_copy[result as usize] = switch_instruction(&program[result as usize]);
    
    return execute_until_duplicate(&program_copy);
}

pub fn switch_instruction(ins: &Instruction) -> Instruction
{
    if ins.op_code == "nop"
    {
        return Instruction::from("jmp", ins.arg);
    }
    else
    {
        return Instruction::from("nop", ins.arg);
    }
}

pub fn find_back(program: &Vec<Instruction>, success: &HashSet<i32>, pc: i32) -> Option<i32>
{
    let switched_jumps = find_previous_jumps_if_switched(program, pc);
    let found = switched_jumps.iter()
        .find(|x| success.contains(x))
        .copied();
    if found.is_some()
    {
        return found;
    }
    let jumps = find_previous_jumps(program, pc);
    for jump in jumps
    {
        let result = find_back(program, success, jump);
        if result.is_some()
        {
            return result;
        }
    }
    
    return None;
}

// finds jmp instructions that point to current location
pub fn find_previous_jumps(program: &Vec<Instruction>, target: i32) -> Vec<i32>
{
    let mut results = Vec::new();

    if program.get((target - 1) as usize).map_or(false, |x| x.op_code == "nop" || x.op_code == "acc")
    {
        results.push(target - 1)
    }

    for i in 0..program.len()
    {
        let ins = &program[i];
        if ins.op_code == "jmp" && ins.arg + (i as i32) == target
        {
            results.push(i as i32);
        }
    }
    return results;
}

// finds jmp instructions that point to current location if changed
pub fn find_previous_jumps_if_switched(program: &Vec<Instruction>, target: i32) -> Vec<i32>
{
    let mut results = Vec::new();

    if program.get((target - 1) as usize).map_or(false, |x| x.op_code == "jmp")
    {
        results.push(target - 1)
    }

    for i in 0..program.len()
    {
        let ins = &program[i];
        if ins.op_code == "nop" && ins.arg + (i as i32) == target
        {
            results.push(i as i32);
        }
    }
    return results;
}

pub fn parse_instruction(ins: &String) -> Instruction
{
    let mut parts = ins.split(" ");
    let op_code = parts.next().unwrap().to_string();
    let arg = parts.next().unwrap();
    return Instruction{op_code: op_code, arg: arg.parse::<i32>().unwrap()};
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
    fn test_parse_instruction() 
    {
        assert_eq!(Instruction::from("nop", 0), parse_instruction(&String::from("nop +0")));
        assert_eq!(Instruction::from("acc", 1), parse_instruction(&String::from("acc +1")));
        assert_eq!(Instruction::from("jmp", -3), parse_instruction(&String::from("jmp -3")));
    }

    #[test]
    fn test_execute_until_duplicate_single() 
    {
        assert_eq!(10, execute_until_duplicate(&vec![Instruction::from("acc", 10)]).acc);
        assert_eq!(-54, execute_until_duplicate(&vec![Instruction::from("acc", -54)]).acc);
        assert_eq!(0, execute_until_duplicate(&vec![Instruction::from("jmp", 0)]).acc);
        assert_eq!(0, execute_until_duplicate(&vec![Instruction::from("nop", 0)]).acc);
    }

    #[test]
    fn test_execute_until_duplicate_loop() 
    {
        let input = vec![
            Instruction::from("nop", 0),
            Instruction::from("acc", 1),
            Instruction::from("jmp", 4),
            Instruction::from("acc", 3),
            Instruction::from("jmp", -3),
            Instruction::from("acc", -99),
            Instruction::from("acc", 1),
            Instruction::from("jmp", -4),
            Instruction::from("acc", 6)
        ];
        assert_eq!(5, execute_until_duplicate(&input).acc);
    }

    #[test]
    fn test_find_previous_jumps_nop() 
    {   
        let input1 = vec![
            Instruction::from("nop", 0),
        ];
        assert_eq!(vec![0], find_previous_jumps(&input1, 1));
        let input2 = vec![
            Instruction::from("acc", 0),
        ];
        assert_eq!(vec![0], find_previous_jumps(&input2, 1));
        let input3 = vec![
            Instruction::from("jmp", 0),
        ];
        assert_eq!(0, find_previous_jumps(&input3, 1).len());
    }

    #[test]
    fn test_find_previous_jumps_jmps() 
    {   
        let input1 = vec![
            Instruction::from("jmp", 1),
        ];
        assert_eq!(vec![0], find_previous_jumps(&input1, 1));
        let input2 = vec![
            Instruction::from("jmp", 3),
            Instruction::from("nop", 2),
            Instruction::from("jmp", 1),
        ];
        assert_eq!(vec![0, 2], find_previous_jumps(&input2, 3));
        let input2 = vec![
            Instruction::from("acc", 3),
            Instruction::from("jmp", -1),
            Instruction::from("jmp", 1),
        ];
        assert_eq!(vec![1], find_previous_jumps(&input2, 0));
    }

    #[test]
    fn test_find_previous_jumps_switched_nop() 
    {   
        let input1 = vec![
            Instruction::from("jmp", 5),
        ];
        assert_eq!(vec![0], find_previous_jumps_if_switched(&input1, 1));
        let input2 = vec![
            Instruction::from("nop", 0),
        ];
        assert_eq!(0, find_previous_jumps_if_switched(&input2, 1).len());
    }

    #[test]
    fn test_find_previous_jumps_switched_jmps() 
    {   
        let input1 = vec![
            Instruction::from("nop", 1),
        ];
        assert_eq!(vec![0], find_previous_jumps_if_switched(&input1, 1));
        let input2 = vec![
            Instruction::from("nop", 3),
            Instruction::from("jmp", 2),
            Instruction::from("nop", 1),
        ];
        assert_eq!(vec![0, 2], find_previous_jumps_if_switched(&input2, 3));
        let input2 = vec![
            Instruction::from("acc", 3),
            Instruction::from("nop", -1),
            Instruction::from("nop", 1),
        ];
        assert_eq!(vec![1], find_previous_jumps_if_switched(&input2, 0));
    }

    #[test]
    fn test_fix_and_run_program() 
    {
        let input = vec![
            Instruction::from("nop", 0),
            Instruction::from("acc", 1),
            Instruction::from("jmp", 4),
            Instruction::from("acc", 3),
            Instruction::from("jmp", -3),
            Instruction::from("acc", -99),
            Instruction::from("acc", 1),
            Instruction::from("jmp", -4),
            Instruction::from("acc", 6)
        ];
        assert_eq!(8, fix_and_run_program(&input).acc);
    }
}