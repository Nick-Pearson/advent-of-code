use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter(|(answer, inputs)| is_valid(answer, inputs))
        .map(|(ans, _)| ans)
        .sum()
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let (answer, inputs) = line.split(": ").collect_tuple().unwrap();
    let answer = answer.parse().unwrap();
    let inputs = inputs.split(" ").map(|n| n.parse().unwrap()).collect();
    (answer, inputs)
}

fn is_valid(answer: &i64, inputs: &[i64]) -> bool {
    is_valid_recursive(answer, inputs[0], &inputs[1..])
}

fn is_valid_recursive(answer: &i64, current: i64, inputs: &[i64]) -> bool {
    if inputs.is_empty() {
        return current == *answer;
    }
    if current > *answer {
        return false;
    }
    is_valid_recursive(answer, current * inputs[0], &inputs[1..])
        || is_valid_recursive(answer, current + inputs[0], &inputs[1..])
}

fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter(|(answer, inputs)| is_valid2(answer, inputs))
        .map(|(ans, _)| ans)
        .sum()
}

fn is_valid2(answer: &i64, inputs: &[i64]) -> bool {
    is_valid_recursive2(answer, inputs[0], &inputs[1..])
}

fn is_valid_recursive2(answer: &i64, current: i64, inputs: &[i64]) -> bool {
    if inputs.is_empty() {
        return current == *answer;
    }
    if current > *answer {
        return false;
    }
    is_valid_recursive2(answer, current * inputs[0], &inputs[1..])
        || is_valid_recursive2(answer, current + inputs[0], &inputs[1..])
        || is_valid_recursive2(answer, concat(current, inputs[0]), &inputs[1..])
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid(&190, &[10, 19]));
        assert_eq!(true, is_valid(&3267, &[81, 40, 27]));
        assert_eq!(false, is_valid(&83, &[17, 5]));
        assert_eq!(false, is_valid(&156, &[15, 6]));
        assert_eq!(false, is_valid(&7290, &[6, 8, 6, 15]));
        assert_eq!(false, is_valid(&161011, &[16, 10, 13]));
        assert_eq!(false, is_valid(&192, &[17, 8, 14]));
        assert_eq!(false, is_valid(&21037, &[9, 7, 18, 13]));
        assert_eq!(true, is_valid(&292, &[11, 6, 16, 20]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        assert_eq!(result, 3749);
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        assert_eq!(result, 11387);
    }
}
