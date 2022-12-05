use std::str::FromStr;

fn main() {
    let input = include_str!("../inputs/day5.txt");
    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {:?}", p1);
    println!("Result part 2: {:?}", p2);
}

fn part1(input: &str) -> String {
    let (instructions, mut stacks) = parse_input(input);
    for inst in &instructions {
        apply_instruction(inst, &mut stacks, false);
    }
    stacks.iter().map(|x| x.last().unwrap()).cloned().collect()
}

fn part2(input: &str) -> String {
    let (instructions, mut stacks) = parse_input(input);
    for inst in &instructions {
        apply_instruction(inst, &mut stacks, true);
    }
    stacks.iter().map(|x| x.last().unwrap()).cloned().collect()
}

fn parse_input(s: &str) -> (Vec<Instruction>, Vec<Vec<char>>) {
    s.split_once("\n\n")
        .map(|(stacks, instructions)| {
            let mut stacks_rv = stacks.lines().rev();
            let last_line = stacks_rv.next().unwrap();
            let num_stacks: u8 = last_line
                .split_ascii_whitespace()
                .rev()
                .next()
                .and_then(|x| x.parse().ok())
                .unwrap();

            let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks as usize];
            for l in stacks_rv {
                let mut column = 0;
                let mut ws_count = 0;
                let split = l.split(' ');

                for s in split {
                    if s.is_empty() {
                        ws_count += 1;
                        if ws_count > 3 {
                            ws_count = 0;
                            column += 1;
                        }
                        continue;
                    } else {
                        let element = s.chars().nth(1).unwrap();
                        stacks[column].push(element);
                        column += 1;
                    }
                }
            }
            let instructions: Vec<Instruction> = instructions
                .lines()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect();
            (instructions, stacks)
        })
        .unwrap()
}

#[derive(Debug)]
pub struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}

fn apply_instruction(instruction: &Instruction, stacks: &mut [Vec<char>], multiple: bool) {
    let mut to_move: Vec<char> = (0..instruction.count)
        .map(|_| stacks[instruction.from as usize].pop().unwrap())
        .collect();
    if multiple {
        to_move.reverse();
    }
    stacks[instruction.to as usize].extend_from_slice(&to_move)
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(" from ").unwrap();
        let count: u8 = first
            .split_once(' ')
            .and_then(|x| x.1.parse().ok())
            .unwrap();
        let (from, to) = second
            .split_once(" to ")
            .map(|(f, t)| (f.parse::<u8>().unwrap(), t.parse::<u8>().unwrap()))
            .unwrap();
        Ok(Self {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

#[cfg(test)]
mod day5_tests {
    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3  

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "CMZ");
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), "MCD");
    }
}
