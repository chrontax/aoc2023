fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

fn symbols_map(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c != '.' && c.is_ascii_punctuation())
                .collect()
        })
        .collect()
}

struct Number {
    value: u32,
    x: usize,
    y: usize,
    length: usize,
}

impl Number {
    fn is_next_to_symbol(&self, symbols: &[Vec<bool>]) -> bool {
        for y in (self.y.saturating_sub(1)).clamp(0, symbols.len() - 1)
            ..=(self.y + 1).clamp(0, symbols.len() - 1)
        {
            for x in (self.x.saturating_sub(1)).clamp(0, symbols[y].len() - 1)
                ..=(self.x + self.length).clamp(0, symbols[y].len() - 1)
            {
                if symbols[y][x] {
                    return true;
                }
            }
        }
        false
    }
}

fn numbers(input: &str) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut cur_num = String::new();
            let mut start_x = 0;
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    if cur_num.is_empty() && c.is_ascii_digit() {
                        start_x = x;
                        cur_num.push(c);
                        if x == line.len() - 1 {
                            let num = cur_num.parse().unwrap();
                            cur_num.clear();
                            Some(Number {
                                value: num,
                                x: start_x,
                                y,
                                length: x - start_x,
                            })
                        } else {
                            None
                        }
                    } else if c.is_ascii_digit() {
                        cur_num.push(c);
                        if x == line.len() - 1 {
                            let num = cur_num.parse().unwrap();
                            cur_num.clear();
                            Some(Number {
                                value: num,
                                x: start_x,
                                y,
                                length: x - start_x,
                            })
                        } else {
                            None
                        }
                    } else if !cur_num.is_empty() {
                        let num = cur_num.parse().unwrap();
                        cur_num.clear();
                        Some(Number {
                            value: num,
                            x: start_x,
                            y,
                            length: x - start_x,
                        })
                    } else {
                        None
                    }
                })
                .filter_map(|x| x)
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let symbols = symbols_map(input);
    let nums = numbers(input);
    nums.iter()
        .filter(|num| num.is_next_to_symbol(&symbols))
        .map(|num| num.value)
        .sum()
}

fn gears(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '*' { Some((x, y)) } else { None })
        })
        .collect()
}

fn gear_ratio((x, y): (usize, usize), nums: &[Number]) -> Option<u32> {
    let mut gears = nums
        .iter()
        .filter(|num| {
            (num.x..num.x + num.length).any(|xx| (x.saturating_sub(1)..=x + 1).contains(&xx))
                && (y.saturating_sub(1)..=y + 1).contains(&num.y)
        })
        .map(|num| num.value);
    let first = gears.next()?;
    let second = gears.next()?;
    if gears.next().is_some() {
        return None;
    }
    Some(first * second)
}

fn part2(input: &str) -> u32 {
    let nums = numbers(input);
    gears(input)
        .iter()
        .filter_map(|gear| gear_ratio(*gear, &nums))
        .sum()
}
