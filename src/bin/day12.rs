use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn main() {
    let input: &str = include_str!("../inputs/day12.txt");

    let p1 = part1(input);
    let p2 = part2(input);
    println!("Result part 1: {}", p1);
    println!("Result part 2: {}", p2);
}

fn part1(input: &str) -> usize {
    let map = HeightMap::from_str(input).unwrap();
    let result = bfs(&map, map.start).unwrap();
    result.1.len()
}

fn part2(input: &str) -> usize {
    let map = HeightMap::from_str(input).unwrap();
    let possible_starts = map.all_start_positions();

    possible_starts
        .iter()
        .filter_map(|x| bfs(&map, *x))
        .map(|(_, path)| path.len())
        .min()
        .unwrap()
}

type Point = (i16, i16);
const MOVES: [Point; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub struct HeightMap {
    heights: Vec<u16>,
    width: u16,
    start: usize,
    goal: usize,
}

impl HeightMap {
    fn as_p(&self, idx: usize) -> Point {
        let y = idx / self.width as usize;
        let x = idx % self.width as usize;
        (x as i16, y as i16)
    }

    fn apply_move(&self, idx: usize, op: Point) -> Point {
        let (x, y) = self.as_p(idx);

        let nx = x as i16 + op.0;
        let ny = y as i16 + op.1;
        (nx, ny)
    }

    fn is_goal(&self, idx: usize) -> bool {
        idx == self.goal
    }

    fn get_possibilites(&self, idx: usize) -> Vec<usize> {
        let height = self.heights[idx];
        MOVES
            .iter()
            .map(|&op| self.apply_move(idx, op))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *x < self.width as i16
                    && *y < (self.len() / self.width as usize) as i16
            })
            .map(|(x, y)| (x + y * self.width as i16) as usize)
            .filter(|&x| self.heights[x as usize] <= height + 1)
            .collect()
    }
    fn len(&self) -> usize {
        self.heights.len()
    }
    fn all_start_positions(&self) -> Vec<usize> {
        self.heights
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == self.heights[self.start] as u16)
            .map(|(idx, _)| idx)
            .collect()
    }
}

impl FromStr for HeightMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let width = lines.peek().unwrap().len();
        let mut height_map = Vec::with_capacity(width * 5);

        let mut start = 0;
        let mut target = 0;

        for (y, l) in lines.enumerate() {
            for (x, c) in l.chars().enumerate() {
                let c = match c {
                    'S' => {
                        start = y * width + x;
                        0
                    }
                    'E' => {
                        target = y * width + x;
                        25
                    }
                    c => c as u16 - 97,
                };
                height_map.push(c);
            }
        }
        Ok(Self {
            heights: height_map,
            width: width as u16,
            start,
            goal: target,
        })
    }
}

fn bfs(height_map: &HeightMap, start: usize) -> Option<(u16, Vec<u16>)> {
    let mut queue = VecDeque::from([(start, vec![])]);
    let mut visited = HashSet::from([start]);

    while let Some((idx, path)) = queue.pop_front() {
        if height_map.is_goal(idx) {
            return Some((idx as u16, path));
        }

        for poss in height_map.get_possibilites(idx) {
            if !visited.contains(&poss) {
                visited.insert(poss);
                let mut new_path = path.clone();
                new_path.push(poss as u16);

                queue.push_back((poss, new_path));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 29);
    }
}
