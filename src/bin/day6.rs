use std::collections::HashSet;

fn find(input: &str, windows_len: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(windows_len)
        .enumerate()
        .find(|(_, chars)| HashSet::<&char>::from_iter(chars.iter()).len() == windows_len)
        .map(|(idx, _)| idx + windows_len)
        .unwrap_or(0)
}
fn part1(input: &str) -> usize {
    find(input, 4)
}
fn part2(input: &str) -> usize {
    find(input, 14)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day6.txt");
    let p1 = part1(INPUT);
    let p2 = part2(INPUT);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

#[cfg(test)]
mod day6_tests {
    const CASES: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];
    #[test]
    fn part1() {
        for (input, expected, _) in CASES {
            assert_eq!(super::part1(input), expected)
        }
    }

    #[test]
    fn part2() {
        for (input, _, expected) in CASES {
            assert_eq!(super::part2(input), expected)
        }
    }
}
