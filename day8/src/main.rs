use std::str::Lines;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use num::Integer;

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    name: &'a str,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

fn steps(line: &str) -> Vec<Step> {
    line.chars()
        .map(|c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("Invalid character: {}", c),
        })
        .collect::<Vec<_>>()
}

fn node_parser(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    tuple((
        delimited(space0, alphanumeric1, tag(" = ")),
        delimited(
            tag("("),
            map(
                tuple((alphanumeric1, tag(", "), alphanumeric1)),
                |(x, _, y)| (x, y),
            ),
            tag(")"),
        ),
    ))(input)
}

fn nodes(lines: Lines) -> Vec<Node> {
    let vec = lines
        .map(|line| node_parser(line).unwrap().1)
        .collect::<Vec<_>>();
    let vec = vec
        .iter()
        .map(|(name, (x, y))| {
            Node {
                name,
                left: vec.iter().position(|(n, _)| n == x).unwrap(),
                right: vec.iter().position(|(n, _)| n == y).unwrap(),
            }
        })
        .collect::<Vec<_>>();
    vec
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let steps = steps(lines.next().unwrap());
    let mut steps = steps.iter().cycle();
    let mut steps_taken = 0;
    lines.next();
    let nodes = nodes(lines);
    let node = nodes.iter().find(|n| n.name == "AAA");
    let mut node = node.unwrap().clone();
    while node.name != "ZZZ" {
        let step = steps.next().unwrap();
        steps_taken += 1;
        node = match step {
            Step::Left => nodes[node.left],
            Step::Right => nodes[node.right],
        }
    }
    steps_taken
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let steps = steps(lines.next().unwrap());
    lines.next();
    let nodes = nodes(lines);
    let start_nodes = nodes
        .iter()
        .filter(|n| n.name.ends_with('A'))
        .collect::<Vec<_>>();
    let mut uwu = Vec::new();
    for node in start_nodes {
        let mut steps = steps.iter().copied().cycle();
        let mut node = node;
        let mut steps_taken = 0;
        while !node.name.ends_with('Z') {
            let step = steps.next().unwrap();
            steps_taken += 1;
            node = match step {
                Step::Left => &nodes[node.left],
                Step::Right => &nodes[node.right],
            }
        }
        uwu.push(steps_taken);
    }
    uwu.iter().fold(1, |acc, x| acc.lcm(x))
}
