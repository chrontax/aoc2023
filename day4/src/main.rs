use nom::{
    character::complete::{u32, space0},
    multi::many1,
    IResult, sequence::delimited,
};
use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    many1(
        delimited(space0, u32, space0)
    )(input)
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut thingy = line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split(" | ")
                .map(|s| -> HashSet<u32> { HashSet::from_iter(parse_numbers(s).unwrap().1) });
            thingy
                .next()
                .unwrap()
                .intersection(&thingy.next().unwrap())
                .fold(0, |acc, _| {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = input
        .lines()
        .map(|line| {
            let mut thingy = line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split(" | ")
                .map(|s| -> HashSet<u32> { HashSet::from_iter(parse_numbers(s).unwrap().1) });
            (thingy.next().unwrap(), thingy.next().unwrap())
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut cache = vec![0; cards.len()];
    for i in 0..cards.len() {
        count += part2_helper(&cards, i, &mut cache) + 1;
    }
    count
}

fn part2_helper(cards: &Vec<(HashSet<u32>, HashSet<u32>)>, id: usize, cache: &mut Vec<u32>) -> u32 {
    if cache[id] != 0 {
        return cache[id];
    }
    let mut count = cards[id].0.intersection(&cards[id].1).count() as u32;
    for i in 1..=count {
        count += part2_helper(cards, id + i as usize, cache);
    }
    cache[id] = count;
    count
}
