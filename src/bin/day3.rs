use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}

fn part1(input: &str) -> u32 {
    input
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
        .sum()
}

fn part2(input: &str) -> u32 {
    input
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
        .sum()
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1_ok() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn part2_ok() {
        assert_eq!(part2(INPUT), 70);
    }
}
