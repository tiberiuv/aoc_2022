use std::str::FromStr;

#[derive(Debug)]
pub struct IdRange {
    from: u32,
    to: u32,
}

impl IdRange {
    fn contains(&self, other: &Self) -> bool {
        self.from >= other.from && self.to <= other.to
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

impl FromStr for IdRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once('-')
            .ok_or_else(|| "Invalid input".to_owned())?;
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        Ok(IdRange { from, to })
    }
}

fn proc_input<F: Fn(&(IdRange, IdRange)) -> bool>(input: &str, predicate: F) -> usize {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            (IdRange::from_str(a).unwrap(), IdRange::from_str(b).unwrap())
        })
        .filter(predicate)
        .count()
}

fn part1(input: &str) -> usize {
    proc_input(input, |(a, b)| (a.contains(b) || b.contains(a)))
}

fn part2(input: &str) -> usize {
    proc_input(input, |(a, b)| a.overlaps(b))
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

#[cfg(test)]
mod day4_tests {
    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 2)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 4)
    }
}
