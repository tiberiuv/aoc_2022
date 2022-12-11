use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day11.txt");

    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

fn simulate(input: &str, reduce_worry: usize, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|l| Monkey::parse(l, reduce_worry))
        .collect();

    let mut counts = vec![0; monkeys.len()];
    let modulus = monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        let map = simulate_round(&mut monkeys, modulus);
        for (idx, v) in map.iter().enumerate() {
            counts[idx] += v;
        }
    }
    counts.sort();

    counts.iter().rev().take(2).product()
}

fn simulate_round(monkeys: &mut Vec<Monkey>, modulus: usize) -> Vec<usize> {
    let mut counts = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for item in &monkey.items {
            let worry_level = monkey.op.apply(*item) / monkey.reduce_worry;

            let is_divisible = worry_level % monkey.test == 0;
            let idx = if is_divisible {
                monkey.true_target
            } else {
                monkey.false_target
            };
            map.entry(idx as usize)
                .and_modify(|f| f.push(worry_level % modulus))
                .or_insert_with(|| vec![worry_level % modulus]);
        }
        counts[i] += monkey.items.len();

        monkey.items.clear();

        for (idx, items) in map.iter_mut() {
            monkeys[*idx].items.append(items);
        }
    }
    counts
}

fn part1(input: &str) -> usize {
    simulate(input, 3, 20)
}

fn part2(input: &str) -> usize {
    simulate(input, 1, 10000)
}

pub enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        match self {
            Self::Add(v) => old + v,
            Self::Mult(v) => old * v,
            Self::Square => old * old,
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    true_target: usize,
    false_target: usize,
    reduce_worry: usize,
}
impl Monkey {
    fn parse(s: &str, reduce_worry: usize) -> Self {
        fn next_trim_start_matches<'a>(
            it: &mut impl Iterator<Item = &'a str>,
            matching: &str,
        ) -> &'a str {
            it.next().unwrap().trim_start_matches(matching)
        }
        let mut lines = s.lines();
        lines.next();

        let items = next_trim_start_matches(&mut lines, "  Starting items: ")
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let op = next_trim_start_matches(&mut lines, "  Operation: new = old ")
            .split_once(' ')
            .map(|(op, val)| match op {
                "*" if val == "old" => Operation::Square,
                "*" => Operation::Mult(val.parse::<usize>().unwrap()),
                "+" => Operation::Add(val.parse::<usize>().unwrap()),
                _ => panic!("invalid"),
            })
            .unwrap();
        let test = next_trim_start_matches(&mut lines, "  Test: divisible by ")
            .parse::<usize>()
            .unwrap();
        let t = next_trim_start_matches(&mut lines, "    If true: throw to monkey ")
            .parse::<usize>()
            .unwrap();
        let f = next_trim_start_matches(&mut lines, "    If false: throw to monkey ")
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            op,
            test,
            true_target: t,
            false_target: f,
            reduce_worry,
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 10605)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2713310158)
    }
}
