fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

fn time_n_distance(input: &str) -> Vec<(usize, usize)> {
    let mut tmp = input.lines().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
    });
    tmp.next().unwrap().zip(tmp.next().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    time_n_distance(input)
        .iter()
        .map(|&(time, distance)| {
            let mut check = time / 2;
            while check * (time - check) > distance {
                check -= 1;
            }
            (check..time - check).len() as usize - 1
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (time, distance) =
        time_n_distance(&input.chars().filter(|&c| c != ' ').collect::<String>())[0];
    let mut check = time / 2;
    while check * (time - check) > distance {
        check /= 2;
    }
    while check * (time - check) < distance {
        check += 1;
    }
    check -= 1;
    (check..time - check).len() as usize - 1
}
