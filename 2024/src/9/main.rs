use core::panic;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Clone, Debug)]
enum Node {
    File(i64, i64),
    Space(i64),
}

fn part_one(input: &str) -> i64 {
    let nodes = parse(input);
    score(&defragment(&nodes))
}

fn part_two(input: &str) -> i64 {
    let nodes = parse(input);
    score(&defragment2(&nodes))
}

fn parse(input: &str) -> Vec<Node> {
    let mut file = true;
    let mut id = 0;
    let mut result = Vec::new();

    for c in input.chars() {
        let size = c.to_digit(10).unwrap() as i64;
        if file {
            result.push(Node::File(size, id));
            id += 1;
        } else {
            result.push(Node::Space(size));
        }
        file = !file;
    }
    result
}

fn defragment(nodes: &Vec<Node>) -> Vec<Node> {
    let mut result = Vec::new();
    let mut remaining = nodes.clone();

    while !remaining.is_empty() {
        let node = remaining.remove(0);

        match node {
            Node::File(size, id) => {
                result.push(Node::File(size, id));
            }
            Node::Space(mut size) => {
                while size > 0 && !remaining.is_empty() {
                    let candidate = remaining.pop().expect("No more nodes");
                    if let Node::File(candidate_size, candidate_id) = candidate {
                        if candidate_size <= size {
                            result.push(Node::File(candidate_size, candidate_id));
                            size -= candidate_size;
                        } else {
                            result.push(Node::File(size, candidate_id));
                            remaining.push(Node::File(candidate_size - size, candidate_id));
                            size = 0;
                        }
                    }
                }
            }
        }
    }
    result
}

fn defragment2(nodes: &Vec<Node>) -> Vec<Node> {
    let mut result = nodes.clone();
    let mut index = 0;

    while index < result.len() {
        let i = result.len() - index - 1;
        let swap_idx = find_swap_idx(&result, i);

        if let Some(idx) = swap_idx {
            let file = result[i].clone();
            result.remove(i);
            let size = match file {
                Node::File(size, _) => size,
                _ => panic!("Expected file node"),
            };
            result.insert(i, Node::Space(size));
            result.insert(idx, file);

            let space = result[idx + 1].clone();
            result.remove(idx + 1);
            if let Node::Space(space_size) = space {
                if space_size > size {
                    result.insert(idx + 1, Node::Space(space_size - size));
                }
            } else {
                panic!("Expected space node");
            }
        }

        index += 1;
    }
    result
}

fn find_swap_idx(nodes: &Vec<Node>, i: usize) -> Option<usize> {
    let node = &nodes[i];

    if let Node::File(size, _) = node {
        let size = *size;

        return nodes
            .iter()
            .enumerate()
            .filter(|n| {
                if let Node::Space(candidate_size) = n.1 {
                    return *candidate_size >= size;
                }
                false
            })
            .find(|(n, _)| *n < i)
            .map(|a| a.0);
    }
    None
}

fn score(nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    let mut index = 0;

    for node in nodes {
        match node {
            Node::File(size, id) => {
                for i in 0..*size {
                    result += (i + index) * *id;
                }
                index += *size;
            }
            Node::Space(size) => {
                index += *size;
            }
        }
    }
    result
}

fn dbg(nodes: &Vec<Node>) {
    for node in nodes {
        match node {
            Node::File(size, id) => {
                for _ in 0..*size {
                    print!("{}", id);
                }
            }
            Node::Space(size) => {
                for _ in 0..*size {
                    print!(".");
                }
            }
        }
    }
    println!("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2333133121414131402"), 1928);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2333133121414131402"), 2858);
    }
}
