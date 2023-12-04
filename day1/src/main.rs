use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

fn main() {
    let input = include_str!("../input.txt");
    let now = std::time::Instant::now();
    let part1 = calibration_value(input);
    println!("Part 1: {} [{:?}]", part1, now.elapsed());
    let now = std::time::Instant::now();
    let part2 = part2(input);
    println!("Part 2: {} [{:?}]", part2, now.elapsed());
}

fn calibration_value(s: &str) -> u32 {
    let mut values = Vec::new();
    for line in s.lines().map(|l| l.chars().collect::<Vec<char>>()) {
        let mut i: usize = 0;
        let mut value = 0;
        while !line[i].is_numeric() {
            i += 1;
        }
        value += line[i].to_digit(10).unwrap() * 10;
        i = line.len() - 1;
        while !line[i].is_numeric() {
            i -= 1;
        }
        value += line[i].to_digit(10).unwrap();
        values.push(value);
    }
    values.iter().sum()
}

const NUMS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn num(s: &str) -> IResult<&str, u32, ()> {
    map(
        alt((
            tag("zero"),
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        )),
        |s| NUMS.iter().find(|(n, _)| *n == s).unwrap().1,
    )(s)
}

fn part2(s: &str) -> u32 {
    let mut values = Vec::new();
    for line in s.lines().map(|l| l.chars().collect::<Vec<char>>()) {
        let mut i: usize = 0;
        let mut value = 0;
        while !line[i].is_numeric() && num(&line[i..].iter().collect::<String>()).is_err() {
            i += 1;
        }
        if let Ok((_, n)) = num(&line[i..].iter().collect::<String>()) {
            value += n * 10;
        } else {
            value += line[i].to_digit(10).unwrap() * 10;
        }
        i = line.len() - 1;
        while !line[i].is_numeric() && num(&line[i..].iter().collect::<String>()).is_err() {
            i -= 1;
        }
        if let Ok((_, n)) = num(&line[i..].iter().collect::<String>()) {
            value += n;
        } else {
            value += line[i].to_digit(10).unwrap();
        }
        values.push(value);
    }
    values.iter().sum()
}
