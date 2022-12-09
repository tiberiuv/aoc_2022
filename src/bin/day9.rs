use std::collections::HashSet;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const RIGHT: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (-1, 0);
const TOPRIGHT: (i32, i32) = (1, -1);
const TOPLEFT: (i32, i32) = (-1, -1);
const BOTTOMRIGHT: (i32, i32) = (1, 1);
const BOTTOMLEFT: (i32, i32) = (-1, 1);
const ALL: [(i32, i32); 8] = [
    UP,
    DOWN,
    RIGHT,
    LEFT,
    TOPRIGHT,
    TOPLEFT,
    BOTTOMRIGHT,
    BOTTOMLEFT,
];
const DIRECTIONS: [(i32, i32); 4] = [UP, DOWN, RIGHT, LEFT];
const DIAGONALS: [(i32, i32); 4] = [TOPRIGHT, TOPLEFT, BOTTOMRIGHT, BOTTOMLEFT];

const fn apply((x, y): (i32, i32), (dx, dy): (i32, i32)) -> (i32, i32) {
    (x + dx, y + dy)
}

const fn manhatten_distance(point: (i32, i32), other: (i32, i32)) -> i32 {
    (point.0 - other.0).abs() + (point.1 - other.1).abs()
}

fn find_movement(current: (i32, i32), target: (i32, i32)) -> (i32, i32) {
    [DIRECTIONS, DIAGONALS]
        .iter()
        .flatten()
        .map(|&d| apply(current, d))
        .find(|&x| manhatten_distance(x, target) == 1)
        .unwrap_or_else(|| {
            [DIRECTIONS, DIAGONALS]
                .iter()
                .flatten()
                .map(|&d| apply(current, d))
                .find(|&x| manhatten_distance(x, target) == 2)
                .unwrap()
        })
}

fn is_adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
    ALL.iter().any(|x| apply(*x, a) == b)
}

fn direction_to_offset(dir: &str) -> (i32, i32) {
    match dir {
        "U" => UP,
        "R" => RIGHT,
        "L" => LEFT,
        "D" => DOWN,
        _ => panic!("Unexpected direction"),
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ((i32, i32), i32)> + '_ {
    input.lines().map(|l| {
        l.split_once(' ')
            .map(|(dir, step)| (direction_to_offset(dir), step.parse::<i32>().unwrap()))
            .unwrap()
    })
}

fn simulate_rope(direction: (i32, i32), rope: &mut Vec<(i32, i32)>) {
    let head = rope.first_mut().unwrap();
    *head = apply(*head, direction);
    for i in 0..rope.len() - 1 {
        let curr = rope[i];
        let next = rope[i + 1];
        if !(curr == next || is_adjacent(curr, next)) {
            let movement = find_movement(next, curr);
            rope[i + 1] = movement;
        }
    }
}

fn run_simulation(input: &str, rope_size: usize) -> usize {
    let input = parse_input(input);

    let mut rope = vec![(0, 0); rope_size];
    let mut seen = HashSet::from([(0, 0)]);

    for (direction, step) in input {
        for _ in 0..step {
            simulate_rope(direction, &mut rope);
            seen.insert(*rope.last().unwrap());
        }
    }
    seen.len()
}

fn part1(input: &str) -> usize {
    run_simulation(input, 2)
}

fn part2(input: &str) -> usize {
    run_simulation(input, 10)
}

fn main() {
    let input = include_str!("../inputs/day9.txt");

    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    const INPUT_LARGER: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1);
        assert_eq!(super::part2(INPUT_LARGER), 36);
    }
}
