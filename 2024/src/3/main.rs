use regex::Regex;

#[derive(Debug, PartialEq)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);

    let part_one = parsed
        .iter()
        .map(|op| if let Op::Mul(a, b) = op { a * b } else { 0 })
        .sum::<i64>();
    println!("Part one {}", part_one);

    let mut enabled = true;
    let mut part_two = 0;
    for op in parsed {
        match op {
            Op::Mul(a, b) => {
                if enabled {
                    part_two += a * b;
                }
            }
            Op::Do => {
                enabled = true;
            }
            Op::Dont => {
                enabled = false;
            }
        }
    }
    println!("Part two {}", part_two);
}

fn parse(input: &str) -> Vec<Op> {
    let mut result = Vec::new();
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    for cap in re.find_iter(input) {
        if cap.as_str().starts_with("mul(") {
            let mut parts = cap.as_str().split(',');
            let a = parts
                .next()
                .unwrap()
                .split("mul(")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let b = parts
                .next()
                .unwrap()
                .split(')')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            result.push(Op::Mul(a, b));
        } else if cap.as_str().starts_with("do()") {
            result.push(Op::Do);
        } else if cap.as_str().starts_with("don't()") {
            result.push(Op::Dont);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![Op::Mul(2, 4), Op::Mul(5, 5), Op::Mul(11, 8), Op::Mul(8, 5)];
        assert_eq!(
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            expected
        );
    }
    #[test]
    fn test_parse2() {
        let expected = vec![
            Op::Mul(2, 4),
            Op::Dont,
            Op::Mul(5, 5),
            Op::Mul(11, 8),
            Op::Do,
            Op::Mul(8, 5),
        ];
        assert_eq!(
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            expected
        );
    }
}
