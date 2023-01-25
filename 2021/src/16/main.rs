use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("src/16/input.txt") {
        let input:String = lines
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .find(|_x| true)
            .unwrap();

        println!("Part 1: {}", part_one(&input));
        println!("Part 2: {}", part_two(&input));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn expand(input: &String) -> Vec<u8>
{
    let mut result = Vec::new();
    for c in input.chars()
    {
        let mut app:Vec<u8> = match c {
            '0' => vec![0,0,0,0],
            '1' => vec![0,0,0,1],
            '2' => vec![0,0,1,0],
            '3' => vec![0,0,1,1],
            '4' => vec![0,1,0,0],
            '5' => vec![0,1,0,1],
            '6' => vec![0,1,1,0],
            '7' => vec![0,1,1,1],
            '8' => vec![1,0,0,0],
            '9' => vec![1,0,0,1],
            'A' => vec![1,0,1,0],
            'B' => vec![1,0,1,1],
            'C' => vec![1,1,0,0],
            'D' => vec![1,1,0,1],
            'E' => vec![1,1,1,0],
            'F' => vec![1,1,1,1],
            _ => panic!("bad input character {}", c)
        };
        result.append(&mut app);
    }
    return result;
}

fn read_3(stream: &mut std::slice::Iter<'_, u8>) -> u8
{
    let mut val:u8 = 0;
    for i in 0..3
    {
        val = val + (*stream.next().unwrap() << (2 - i));
    }
    return val;
}

fn read_5(stream: &mut std::slice::Iter<'_, u8>) -> u8
{
    let mut val:u8 = 0;
    for i in 0..5
    {
        val = val + (*stream.next().unwrap() << (4 - i));
    }
    return val;
}

fn read_11(stream: &mut std::slice::Iter<'_, u8>) -> u32
{
    let mut val:u32 = 0;
    for i in 0..11
    {
        val = val + ((*stream.next().unwrap() as u32) << (10 - i));
    }
    return val;
}

fn read_15(stream: &mut std::slice::Iter<'_, u8>) -> u32
{
    let mut val:u32 = 0;
    for i in 0..15
    {
        val = val + ((*stream.next().unwrap() as u32) << (14 - i));
    }
    return val;
}

#[derive(Debug)]
pub struct Instruction
{
    version: u8,
    id: u8,
    literal: usize,
    sub_packets: Vec<Instruction>
}

fn parse_instruction(stream: &mut std::slice::Iter<'_, u8>) -> (Instruction, usize)
{
    let ver = read_3(stream);
    let id = read_3(stream);
    let sub_packets;
    let mut literal = 0;
    let mut bits_read = 6;

    if id == 4
    {
        sub_packets = Vec::new();

        let mut bit = read_5(stream);
        literal = literal + (bit & 15) as usize;
        bits_read = bits_read + 5;

        while bit & 16 > 0
        {
            bit = read_5(stream);
            literal = (literal << 4) + (bit & 15) as usize;
            bits_read = bits_read + 5;
        }
    }
    else
    {
        let l = *stream.next().unwrap();
        bits_read = bits_read + 1;
        if l == 0
        {
            let mut num_sub_bits = read_15(stream) as usize;
            bits_read = bits_read + 15;
            
            let mut packets = Vec::new();
            while num_sub_bits > 0
            {
                let result = parse_instruction(stream);
                num_sub_bits = num_sub_bits - result.1;
                bits_read = bits_read + result.1;
                packets.push(result.0);
            }

            sub_packets = packets;
        }
        else
        {
            let num_sub_packets = read_11(stream);
            bits_read = bits_read + 11;
            sub_packets = (0..num_sub_packets)
                .map(|_x| parse_instruction(stream))
                .map(|x| {
                    bits_read = bits_read + x.1;
                    return x.0;
                })
                .collect();
        }
    }

    return (Instruction{
        version: ver,
        id: id,
        sub_packets: sub_packets,
        literal: literal
    }, bits_read);
}

fn sum_versions(instr: &Instruction) -> usize
{
    let mut sum = instr.version as usize;
    for i in instr.sub_packets.iter()
    {
        sum = sum + sum_versions(&i);
    }
    return sum;
}

fn part_one(input: &String) -> usize
{
    let bits = expand(input);
    let i:Instruction = parse_instruction(&mut bits.iter()).0;
    return sum_versions(&i);
}

fn bool_to_int(val: bool) -> usize
{
    if val
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

fn evaluate_instruction(instr: &Instruction) -> usize
{
    let sub_values:Vec<usize> = instr.sub_packets.iter()
        .map(|p| evaluate_instruction(p))
        .collect();
    return match instr.id
    {
        0 => sub_values.iter().fold(0, |a,b| a + b),
        1 => sub_values.iter().fold(1, |a,b| a * b),
        2 => sub_values.iter().fold(9999999, |a,b| usize::min(a, *b)),
        3 => sub_values.iter().fold(0, |a,b| usize::max(a, *b)),
        4 => instr.literal,
        5 => bool_to_int(sub_values[0] > sub_values[1]),
        6 => bool_to_int(sub_values[0] < sub_values[1]),
        7 => bool_to_int(sub_values[0] == sub_values[1]),
        _ => panic!("unsupported instruction id {}", instr.id)
    };
}

fn part_two(input: &String) -> usize
{
    let bits = expand(input);
    let i:Instruction = parse_instruction(&mut bits.iter()).0;
    return evaluate_instruction(&i);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one1() 
    {
        assert_eq!(6, part_one(&String::from("D2FE28")));
    }
    #[test]
    fn test_part_one2() 
    {
        assert_eq!(9, part_one(&String::from("38006F45291200")));
    }
    #[test]
    fn test_part_one3() 
    {
        assert_eq!(14, part_one(&String::from("EE00D40C823060")));
    }
    #[test]
    fn test_part_one4() 
    {
        assert_eq!(16, part_one(&String::from("8A004A801A8002F478")));
    }
    #[test]
    fn test_part_one5() 
    {
        assert_eq!(12, part_one(&String::from("620080001611562C8802118E34")));
    }
    #[test]
    fn test_part_one6() 
    {
        assert_eq!(23, part_one(&String::from("C0015000016115A2E0802F182340")));
    }
    #[test]
    fn test_part_one7() 
    {
        assert_eq!(31, part_one(&String::from("A0016C880162017C3686B18A3D4780")));
    }

    #[test]
    fn test_part_two1() 
    {
        assert_eq!(3, part_two(&String::from("C200B40A82")));
    }
    #[test]
    fn test_part_two2() 
    {
        assert_eq!(54, part_two(&String::from("04005AC33890")));
    }
    #[test]
    fn test_part_two3() 
    {
        assert_eq!(7, part_two(&String::from("880086C3E88112")));
    }
    #[test]
    fn test_part_two4() 
    {
        assert_eq!(9, part_two(&String::from("CE00C43D881120")));
    }
    #[test]
    fn test_part_two5() 
    {
        assert_eq!(1, part_two(&String::from("D8005AC2A8F0")));
    }
    #[test]
    fn test_part_two6() 
    {
        assert_eq!(0, part_two(&String::from("F600BC2D8F")));
    }
    #[test]
    fn test_part_two7() 
    {
        assert_eq!(0, part_two(&String::from("9C005AC2F8F0")));
    }
    #[test]
    fn test_part_two8() 
    {
        assert_eq!(1, part_two(&String::from("9C0141080250320F1802104A08")));
    }
}