use std::{iter::Map, str::{Lines, Split}};

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT, [12, 13, 14]);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

fn split(input: &str) -> Map<Lines, fn(&str) -> Map<Split<'_, &str>, fn(&str) -> Map<Split<'_, &str>, fn(&str) -> (u32, &str)>>> {
    input.lines().map(|line| {
        line.split(": ").skip(1).next().unwrap().split("; ").map(|set| {
            set.split(", ").map(|part| {
                let mut parts = part.splitn(2, ' ');
                (
                    parts.next().unwrap().parse::<u32>().unwrap(),
                    parts.next().unwrap(),
                )
            })
        })
    })
}

fn part1(input: &str, available: [u32; 3]) -> u32 {
    let mut total = 0;
    let input = split(input);
    'uwu: for (idx, line) in input.enumerate() {
        let mut valid = true;
        for set in line {
            for (count, color) in set {
                match color {
                    "red" => valid = available[0] >= count,
                    "green" => valid = available[1] >= count,
                    "blue" => valid = available[2] >= count,
                    _ => panic!("Unknown color: {}", color),
                };
                if !valid {
                    continue 'uwu;
                }
            }
        }
        total += idx + 1;
    }
    total as u32
}

fn part2(input: &str) -> u32 {
    let mut total = 0;
    let input = split(input);
    for line in input {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in line {
            for part in set {
                let (count, color) = part;
                match color {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => panic!("Unknown color: {}", color),
                };
            }
        }
        total += max_red * max_green * max_blue;
    }
    total
}