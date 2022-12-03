use std::collections::BinaryHeap;

fn proc_input(input: &str) -> Vec<u32> {
    let data = input
        .split("\n\n")
        .into_iter()
        .map(|elf_data| {
            elf_data
                .split('\n')
                .into_iter()
                .map(|v| v.parse().unwrap_or(0))
                .sum()
        })
        .collect::<BinaryHeap<u32>>()
        .into_sorted_vec();

    data.into_iter().rev().take(3).collect()
}
fn part1(input: &str) -> u32 {
    let top3 = proc_input(input);
    top3[0]
}

fn part2(input: &str) -> u32 {
    let top3 = proc_input(input);
    top3.into_iter().sum::<u32>()
}

fn main() {
    let input = include_str!("../inputs/day1.txt");

    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1 {:?}", p1);
    println!("Result part 2 {:?}", p2);
}

#[cfg(test)]
mod day1_tests {
    use super::*;
    const TEST_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    #[test]
    fn part1_ok() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn part2_ok() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}
