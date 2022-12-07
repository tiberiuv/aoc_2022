use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let p1 = part1(input);
    let p2 = part2(input);

    println!("Result part 1: {:?}", p1);
    println!("Result part 2: {:?}", p2);
}

fn parse_paths(input: &str) -> HashMap<PathBuf, usize> {
    let mut current_path = PathBuf::from("/");
    let mut paths: HashMap<PathBuf, usize> = HashMap::new();

    for line in input.lines() {
        if let Some(line) = line.strip_prefix("$ ") {
            if let Some(target) = line.strip_prefix("cd ") {
                if target == ".." {
                    current_path.pop();
                } else if target == "/" {
                    current_path = Path::new("/").to_owned();
                } else {
                    current_path.push(Path::new(&target));
                }
            }
        } else if !line.starts_with("dir ") {
            let (size, _) = line
                .split_once(' ')
                .map(|(size, file)| (size.parse::<usize>().unwrap(), file))
                .unwrap();

            for ancestor in current_path.ancestors() {
                paths
                    .entry(ancestor.to_path_buf())
                    .and_modify(|s| *s += size)
                    .or_insert_with(|| size);
            }
        }
    }
    paths
}

fn part1(input: &str) -> usize {
    let paths = parse_paths(input);
    paths
        .values()
        .cloned()
        .filter(|size| size <= &100_000usize)
        .sum()
}

fn part2(input: &str) -> usize {
    const REQUIRED_SIZE: usize = 30_000_000usize;
    const MAX_SIZE: usize = 70_000_000usize;
    let paths = parse_paths(input);

    let root_size = paths.get(Path::new("/")).unwrap();
    let required_to_free = REQUIRED_SIZE - (MAX_SIZE - root_size);

    paths
        .values()
        .cloned()
        .filter(|&size| size >= required_to_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod day7_tests {
    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 95437)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 24933642)
    }
}
