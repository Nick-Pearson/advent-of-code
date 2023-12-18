fn main() {
    let input = include_str!("input.txt");
    let part_one: u32 = input.lines().map(|l| parse_calibration(l)).sum();
    println!("Part 1: {}", part_one);
    let part_two: u32 = input.lines().map(|l| parse_calibration2(l)).sum();
    println!("Part 2: {}", part_two);
}

fn parse_calibration(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_numeric())
        .filter_map(|c| c.to_digit(10))
        .collect();
    (digits[0] * 10) + digits[digits.len() - 1]
}

fn parse_calibration2(line: &str) -> u32 {
    let modified_line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    parse_calibration(&modified_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration() {
        assert_eq!(12, parse_calibration("1abc2"));
        assert_eq!(38, parse_calibration("pqr3stu8vwx"));
        assert_eq!(15, parse_calibration("a1b2c3d4e5f"));
        assert_eq!(77, parse_calibration("treb7uchet"));
    }

    #[test]
    fn test_parse_calibration2() {
        assert_eq!(29, parse_calibration2("two1nine"));
        assert_eq!(83, parse_calibration2("eightwothree"));
        assert_eq!(13, parse_calibration2("abcone2threexyz"));
        assert_eq!(24, parse_calibration2("xtwone3four"));
        assert_eq!(42, parse_calibration2("4nineeightseven2"));
        assert_eq!(14, parse_calibration2("zoneight234"));
        assert_eq!(76, parse_calibration2("7pqrstsixteen"));
        assert_eq!(12, parse_calibration2("1abc2"));
        assert_eq!(38, parse_calibration2("pqr3stu8vwx"));
        assert_eq!(15, parse_calibration2("a1b2c3d4e5f"));
        assert_eq!(77, parse_calibration2("treb7uchet"));
        assert_eq!(79, parse_calibration2("sevenine"));
        assert_eq!(81, parse_calibration2("eight5fourone"));
    }
}
