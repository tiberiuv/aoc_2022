use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("./input.txt");

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

    let top3: Vec<&u32> = data.iter().rev().take(3).collect();

    println!("Result part 1 {:?}", top3[0]);

    println!("Result part 2 {:?}", top3.into_iter().sum::<u32>());
}
