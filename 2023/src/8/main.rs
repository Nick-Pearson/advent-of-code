use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    marker::PhantomData,
    ptr::NonNull,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, multispace1},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

type Link = NonNull<Node>;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: Link,
    right: Link,
}

impl Node {
    fn new(id: &str) -> Self {
        Node {
            id: id.into(),
            left: NonNull::dangling(),
            right: NonNull::dangling(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    begin: Link,
    all_begins: Vec<Link>,
    _phantom: PhantomData<Node>,
}

impl Map {
    pub fn new(begin: Link, all_begins: Vec<Link>) -> Self {
        Self {
            begin,
            all_begins,
            _phantom: PhantomData,
        }
    }

    fn recursive_add_to_map(val: Link, map: &mut HashMap<String, Box<Node>>) {
        match map.entry(unsafe { val.as_ref() }.id.clone()) {
            Occupied(_) => {}
            Vacant(e) => {
                let boxed = unsafe { Box::from_raw(val.as_ptr()) };
                let left = boxed.left.clone();
                let right = boxed.right.clone();
                e.insert(boxed);

                Self::recursive_add_to_map(left, map);
                Self::recursive_add_to_map(right, map);
            }
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        let mut to_drop = HashMap::new();
        Self::recursive_add_to_map(self.begin, &mut to_drop);
        drop(to_drop);
    }
}

struct Follow {
    current_node: Link,
}

impl Follow {
    pub fn new(begin: Link) -> Self {
        Self {
            current_node: begin,
        }
    }

    pub fn get_id(&self) -> &String {
        &unsafe { self.current_node.as_ref() }.id
    }

    pub fn move_left(&mut self) {
        self.current_node = unsafe { self.current_node.as_ref() }.left;
    }

    pub fn move_right(&mut self) {
        self.current_node = unsafe { self.current_node.as_ref() }.right;
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, (instructions, map)) = parse(input).unwrap();
    let part_one = follow_instructions(&instructions, &map);
    println!("Part one: {}", part_one);
    let part_two = follow_all_paths(&instructions, &map);
    println!("Part two: {}", part_two);
}

fn follow_instructions(instructions: &[Instruction], map: &Map) -> usize {
    follow_path_until(instructions, map.begin, |id| id == "ZZZ")
}

fn follow_path_until<P>(instructions: &[Instruction], begin: Link, predicate: P) -> usize
where
    P: Fn(&String) -> bool,
{
    let mut steps = 0;
    let mut current_node = Follow::new(begin);

    while !predicate(current_node.get_id()) {
        let instruction = &instructions[steps % instructions.len()];
        match instruction {
            Instruction::Left => current_node.move_left(),
            Instruction::Right => current_node.move_right(),
        };
        steps = steps + 1;
    }
    steps
}

fn follow_all_paths(instructions: &[Instruction], map: &Map) -> usize {
    let predicate = |id: &String| id.ends_with('Z');
    let steps: Vec<usize> = map
        .all_begins
        .iter()
        .map(|b| follow_path_until(instructions, *b, predicate))
        .collect();
    lcm(&steps)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse(input: &str) -> IResult<&str, (Vec<Instruction>, Map)> {
    tuple((terminated(parse_instructions, multispace1), parse_map))(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (rem, nodes) = parse_nodes(input)?;

    let mut node_map = HashMap::new();
    let mut left_right_map = HashMap::new();
    for n in nodes.into_iter() {
        left_right_map.insert(n.0.id.clone(), n.1);
        let key = n.0.id.clone();
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(n.0))) };
        node_map.insert(key, ptr);
    }

    for (id, (left, right)) in left_right_map.iter_mut() {
        let node = unsafe { node_map.get_mut(id).unwrap().as_mut() };

        let left_node = node_map.get(*left).unwrap();
        let right_node = node_map.get(*right).unwrap();

        node.left = left_node.clone();
        node.right = right_node.clone();
    }

    Ok((
        rem,
        Map::new(
            node_map.get("AAA").unwrap().clone(),
            node_map
                .iter()
                .filter(|e| e.0.ends_with('A'))
                .map(|e| e.1.clone())
                .collect(),
        ),
    ))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<(Node, (&str, &str))>> {
    many1(terminated(parse_node, multispace0))(input)
}

fn parse_node(input: &str) -> IResult<&str, (Node, (&str, &str))> {
    tuple((
        map(terminated(parse_node_id, tag(" = ")), |id| Node::new(id)),
        delimited(
            tag("("),
            separated_pair(parse_node_id, tag(", "), parse_node_id),
            tag(")"),
        ),
    ))(input)
}

fn parse_node_id(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(alt((char('L'), char('R'))), |c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Invalid character {}", c),
    }))(input)
}
