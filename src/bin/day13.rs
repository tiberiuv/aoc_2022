use std::borrow::Cow;
use std::cmp::Ordering;

use serde::Deserialize;
use serde_json::json;

fn main() {
    let input: &str = include_str!("../inputs/day13.txt");
    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

fn parse_input(input: &str) -> Vec<Packet> {
    input
        .replace("\n\n", "\n")
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, v)| v[0] < v[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let dividers = [2, 6].map(|x| packet!([[x]]));
    let mut input = parse_input(input);
    input.extend(dividers.clone());
    input.sort();

    input
        .iter()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Packet<'a>(Cow<'a, Value>);

#[macro_export]
macro_rules! packet {
    ($l: expr) => {
        serde_json::from_value::<Packet>(json!($l)).unwrap()
    };
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Value {
    Number(u8),
    Array(Vec<Value>),
}

impl<'a> PartialOrd for Packet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl<'a> Ord for Packet<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match (left.0.as_ref(), right.0.as_ref()) {
        (Value::Number(l), Value::Number(r)) => l.cmp(r),
        (Value::Array(_), Value::Number(r)) => compare(left, &packet!([r])),
        (Value::Number(l), Value::Array(_)) => compare(&packet!([l]), right),
        (Value::Array(l), Value::Array(r)) => {
            for i in 0..std::cmp::min(l.len(), r.len()) {
                if l[i] != r[i] {
                    return compare(&Packet(Cow::Borrowed(&l[i])), &Packet(Cow::Borrowed(&r[i])));
                }
            }
            l.len().cmp(&r.len())
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    const INPUT_FULL: &str = include_str!("../inputs/day13.txt");
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
        assert_eq!(super::part1(INPUT_FULL), 6478);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 140);
        assert_eq!(super::part2(INPUT_FULL), 21922);
    }
}
