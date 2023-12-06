use std::str::Lines;

use rangemap::RangeMap;

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

fn ranges(mut lines: Lines) -> Vec<RangeMap<i64, i64>> {
    let mut ranges = Vec::with_capacity(7);
    lines.next();
    for _ in 0..7 {
        lines.next();
        let mut range = RangeMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut iter = line.split_whitespace();
            let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
            let src_start = iter.next().unwrap().parse::<i64>().unwrap();
            let len = iter.next().unwrap().parse::<i64>().unwrap();
            let offset = dest_start - src_start;
            range.insert(src_start..src_start + len, offset);
        }
        ranges.push(range);
    }
    ranges
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();
    *ranges(lines)
        .iter()
        .fold(seeds, |acc, range| {
            acc.iter()
                .map(|&seed| seed + range.get(&seed).unwrap_or(&0))
                .collect::<Vec<_>>()
        })
        .iter()
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold((Vec::new(), None), |(mut acc, prev), seed| {
            if let Some(prev) = prev {
                acc.push(prev..prev+seed);
                (acc, None)
            } else {
                (acc, Some(seed))
            }
        })
        .0
        .iter()
        .map(|range| range.clone())
        .collect::<Vec<_>>();
    let ranges = ranges(lines);
    let mut last = seeds;
    for map in ranges {
        let qwq = last.iter();
        let uwu = qwq.clone()
            .flat_map(|rangee| {
                map.overlapping(rangee).map(|(range, offset)| {
                    range.start.max(rangee.start)+offset..range.end.min(rangee.end)+offset
                })
            });
        let owo = qwq
            .flat_map(|range| {
                map.gaps(&range)
            });
        last = uwu.chain(owo)
            .collect::<Vec<_>>();
    }
    last.iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}
