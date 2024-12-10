use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();
    input.lines().map(parse_line).for_each(|(a, b)| {
        list_one.push(a);
        list_two.push(b);
    });
    println!("Part 1: {}", sum_distances(&list_one, &list_two));
    println!("Part 2: {}", similarity_score(&list_one, &list_two));
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut parts = line.split("   ");
    let a = parse_location_id(parts.next().unwrap());
    let b = parse_location_id(parts.next().unwrap());
    (a, b)
}

fn parse_location_id(location: &str) -> i32 {
    location.parse().unwrap()
}

fn sum_distances(list_one: &Vec<i32>, list_two: &Vec<i32>) -> i32 {
    list_one
        .iter()
        .sorted()
        .zip(list_two.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn similarity_score(list_one: &Vec<i32>, list_two: &Vec<i32>) -> i32 {
    let sorted_one = list_one.iter().sorted().collect::<Vec<_>>();
    let sorted_two = list_two.iter().sorted().collect::<Vec<_>>();

    let mut acc = 0;
    for i in 0..sorted_one.len() {
        let one = sorted_one[i];
        let mut n = 0;
        for j in 0..sorted_two.len() {
            let two = sorted_two[j];
            if two == one {
                n += 1;
            } else if two > one {
                break;
            }
        }
        acc += n * one;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_distances() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(sum_distances(&a, &b), 11);
    }

    #[test]
    fn test_similarity_score() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&a, &b), 31);
    }
}
