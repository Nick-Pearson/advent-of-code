use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let part_one = occurrances(input);
    println!("Part one {}", part_one);
    let part_two = x_mas(input);
    println!("Part two {}", part_two);
}

fn occurrances(input: &str) -> usize {
    let mut idx = input.find('X').unwrap_or(input.len());
    let mut count = 0;
    while idx < input.len() {
        count += occurrances_from(input, idx);
        let base = idx + 1;
        idx = input[base..].find('X').unwrap_or(input.len()) + base;
    }
    count
}

fn occurrances_from(input: &str, start: usize) -> usize {
    let line_len = input.find('\n').unwrap() as i64 + 1;
    is_occurance(input, start, 1)
        + is_occurance(input, start, -1)
        + is_occurance(input, start, line_len)
        + is_occurance(input, start, -line_len)
        + is_occurance(input, start, line_len + 1)
        + is_occurance(input, start, line_len - 1)
        + is_occurance(input, start, -line_len + 1)
        + is_occurance(input, start, -line_len - 1)
}

fn is_occurance(input: &str, start: usize, step: i64) -> usize {
    let mut idx = start as i64;
    let tgt = "XMAS";
    for i in 0..tgt.len() {
        if idx < 0
            || idx >= input.len() as i64
            || input[idx as usize..idx as usize + 1] != tgt[i..i + 1]
        {
            return 0;
        }
        idx += step;
    }
    1
}

fn x_mas(input: &str) -> usize {
    let mut idx = input.find('A').unwrap_or(input.len());
    let mut count = 0;
    while idx < input.len() {
        count += x_mas_from(input, idx);
        let base = idx + 1;
        idx = input[base..].find('A').unwrap_or(input.len()) + base;
    }
    count
}

fn x_mas_from(input: &str, start: usize) -> usize {
    let line_len = input.find('\n').unwrap() as i64 + 1;
    let upper_left = start as i64 - line_len - 1;
    let upper_right = start as i64 - line_len + 1;
    let lower_left = start as i64 + line_len - 1;
    let lower_right = start as i64 + line_len + 1;
    if is_mas(input, upper_left, lower_right) && is_mas(input, upper_right, lower_left) {
        1
    } else {
        0
    }
}

fn is_mas(input: &str, start: i64, end: i64) -> bool {
    if let Some(start_idx) = valid_idx(input, start) {
        if let Some(end_idx) = valid_idx(input, end) {
            let s = &input[start_idx..start_idx + 1];
            let e = &input[end_idx..end_idx + 1];
            s == "M" && e == "S" || s == "S" && e == "M"
        } else {
            false
        }
    } else {
        false
    }
}

fn valid_idx(input: &str, idx: i64) -> Option<usize> {
    if idx < 0 || idx >= input.len() as i64 {
        None
    } else {
        Some(idx as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occurrances() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, occurrances(input));
    }

    #[test]
    fn test_x_mas() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, x_mas(input));
    }
}
