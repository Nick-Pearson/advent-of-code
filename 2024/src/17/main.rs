use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug, Clone)]
struct Computer {
    a: i32,
    b: i32,
    c: i32,
    pc: i32,
    output: Vec<i32>,
}

fn part_one(input: &str) -> String {
    let (mut computer, program) = parse(input);
    run_program(&mut computer, &program, usize::MAX);
    computer.output.iter().join(",")
}

fn part_two(input: &str) -> usize {
    let (_, program) = parse(input);

    let mut computer = &mut Computer {
        a: 0,
        b: 0,
        c: 0,
        pc: program.len() as i32 - 2,
        output: vec![],
    };
    while program[computer.pc as usize] != 5 {
        run_program(&mut computer, &program, 1);
    }

    
    let instruction = program[computer.pc as usize];
    let operand = program[computer.pc as usize + 1];
    dbg!(instruction, operand);

    0
}

fn run_program(computer: &mut Computer, program: &[i32], max_instructions: usize) {
    for _ in 0..max_instructions {
        if computer.pc >= program.len() as i32 {
            return;
        }
        let instruction = program[computer.pc as usize];
        let operand = program[computer.pc as usize + 1];
        match instruction {
            0 => {
                // adv
                let op = combo(&computer, operand);
                computer.a = computer.a / (1 << op);
            }
            1 => {
                // bxl
                computer.b = computer.b ^ operand as i32;
            }
            2 => {
                // bst
                computer.b = combo(&computer, operand) % 8;
            }
            3 => {
                // jnz
                if computer.a != 0 {
                    computer.pc = operand as i32 - 2;
                }
            }
            4 => {
                // bxc
                computer.b = computer.b ^ computer.c;
            }
            5 => {
                // out
                computer.output.push(combo(&computer, operand) % 8);
            }
            6 => {
                // bdv
                let op = combo(&computer, operand);
                computer.b = computer.a / (1 << op);
            }
            7 => {
                // cdv
                let op = combo(&computer, operand);
                computer.c = computer.a / (1 << op);
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }
        computer.pc += 2;
    }
}

fn combo(computer: &Computer, op: i32) -> i32 {
    match op {
        0 | 1 | 2 | 3 => op,
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => panic!("Invalid combo operand: {}", op),
    }
}

fn parse(input: &str) -> (Computer, Vec<i32>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[12..].parse().unwrap();
    let b = lines.next().unwrap()[12..].parse().unwrap();
    let c = lines.next().unwrap()[12..].parse().unwrap();

    let program = parse_program(&lines.skip(1).next().unwrap()[9..]);

    (
        Computer {
            a,
            b,
            c,
            pc: 0,
            output: vec![],
        },
        program,
    )
}

fn parse_program(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part_one(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part_two(input), 117440);
    }
}
