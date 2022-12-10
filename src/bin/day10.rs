const WIDTH: usize = 40;

fn parse_line(l: &str) -> (i32, i32) {
    if let Some(add_op) = l.split_once(' ') {
        let number = add_op.1.parse::<i32>().unwrap();
        (number, 2)
    } else {
        (0, 1)
    }
}

fn part1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strength = 0;
    for l in input.lines() {
        let (number, wait) = parse_line(l);

        for _ in 0..wait {
            cycle += 1;
            if cycle == 20 || cycle as usize % WIDTH == 20 {
                signal_strength += cycle as i32 * register;
            }
        }
        register += number;
    }

    signal_strength
}

fn part2(input: &str) -> String {
    let mut cycle: usize = 0;
    let mut sprite_pos: i32 = 1;
    let mut pixels: String = String::with_capacity(240);

    for l in input.lines() {
        let (number, wait) = parse_line(l);

        for _ in 0..wait {
            let pos = (cycle % WIDTH) as i32;
            let char = if (sprite_pos - 1..=sprite_pos + 1).contains(&pos) {
                '#'
            } else {
                '.'
            };
            pixels.push(char);
            cycle += 1;

            if cycle % 40 == 0 {
                pixels.push('\n')
            }
        }
        sprite_pos += number;
    }

    pixels
}

fn main() {
    let input = include_str!("../inputs/day10.txt");

    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: \n{}", p2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../inputs/day10_example.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13140);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n",
        );
    }
}
