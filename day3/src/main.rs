use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}

fn part1() {
    let part1: u32 = include_str!("./input.txt")
        .lines()
        .map(|ruck| ruck.split_at(ruck.len() / 2))
        .map(|(a, b)| {
            (
                HashSet::<char>::from_iter(a.chars()),
                HashSet::<char>::from_iter(b.chars()),
            )
        })
        .map(|(a, b)| a.intersection(&b).next().cloned().unwrap())
        .map(priority)
        .sum();

    println!("Result part 1: {}", part1);
}

fn part2() {
    let part2: u32 = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|triple| {
            triple
                .iter()
                .map(|s| -> HashSet<char> { HashSet::from_iter(s.chars()) })
                .fold(HashSet::<char>::new(), |diff, a| {
                    if diff.is_empty() {
                        a
                    } else {
                        diff.intersection(&a).cloned().collect()
                    }
                })
                .into_iter()
                .next()
                .unwrap()
        })
        .map(priority)
        .sum();
    println!("Result part 2: {}", part2);
}
