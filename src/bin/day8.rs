use std::fmt::Debug;

fn main() {
    let input = include_str!("../inputs/day8.txt");

    let p1 = part1(input);
    let p2 = part2(input);
    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

fn parse_input(input: &str) -> Grid {
    let mut lines = input.lines().peekable();
    let width = lines.peek().map(|s| s.len()).unwrap_or_default();
    let grid: Vec<i32> = lines
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect();

    Grid {
        points: grid,
        width: width as i32,
    }
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    (0..grid.len()).filter(|idx| grid.is_visible(*idx)).count()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    grid.points
        .iter()
        .enumerate()
        .map(|(idx, _)| grid.scenic_score(idx))
        .max()
        .unwrap()
}

#[derive(Debug)]
pub struct Grid {
    points: Vec<i32>,
    width: i32,
}

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const OPTIONS: [Direction; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
    fn apply(&self, p: Point) -> Point {
        match self {
            Self::Up => Point { x: p.x, y: p.y - 1 },
            Self::Down => Point { x: p.x, y: p.y + 1 },
            Self::Left => Point { x: p.x - 1, y: p.y },
            Self::Right => Point { x: p.x + 1, y: p.y },
        }
    }
}

impl Grid {
    fn idx_to_point(&self, idx: usize) -> Point {
        Point {
            x: idx as i32 % self.width,
            y: idx as i32 / self.width,
        }
    }

    fn point_to_idx(&self, p: Point) -> usize {
        p.x as usize + (p.y * self.width) as usize
    }

    fn trees_in_direction(&self, idx: usize, direction: Direction) -> Vec<usize> {
        let point = self.idx_to_point(idx);
        let mut current = point;
        let mut trees = Vec::new();

        while !self.is_edge(current) {
            let new_point = direction.apply(current);

            current = new_point;
            trees.push(self.point_to_idx(new_point));
        }
        trees
    }

    fn trees_in_all_directions(&self, idx: usize) -> Vec<Vec<usize>> {
        Direction::OPTIONS
            .iter()
            .map(|dir| self.trees_in_direction(idx, *dir))
            .collect()
    }

    fn single_is_visible(&self, idx: usize, trees: &[usize]) -> bool {
        let height = self.points[idx];
        self.is_edge(self.idx_to_point(idx))
            || !trees.iter().any(|&tree| self.points[tree] >= height)
    }

    fn is_visible(&self, idx: usize) -> bool {
        self.trees_in_all_directions(idx)
            .iter()
            .any(|trees| self.single_is_visible(idx, trees))
    }

    fn single_scenic_score(&self, idx: usize, trees: &[usize]) -> usize {
        let height = self.points[idx];
        let mut count = 0;

        for tree in trees {
            count += 1;
            if self.points[*tree] as i32 >= height {
                break;
            }
        }
        count
    }

    fn scenic_score(&self, idx: usize) -> usize {
        self.trees_in_all_directions(idx)
            .iter()
            .map(|trees| self.single_scenic_score(idx, trees))
            .product()
    }

    fn is_edge(&self, p: Point) -> bool {
        p.x == 0 || p.y == 0 || p.y == self.width - 1 || p.x == (self.len() as i32 / self.width) - 1
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod day8_tests {
    const INPUT: &str = r#"30373
25512
65332
33549
35390"#;
    const INPUT_FULL: &str = include_str!("../inputs/day8.txt");
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 21);
        assert_eq!(super::part1(INPUT_FULL), 1711);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 8);
        assert_eq!(super::part2(INPUT_FULL), 301392);
    }
}
