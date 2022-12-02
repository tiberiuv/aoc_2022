use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayOption {
    Rock,
    Paper,
    Scissors,
}

impl PlayOption {
    const OPTIONS: [PlayOption; 3] = [PlayOption::Paper, PlayOption::Scissors, PlayOption::Rock];
    fn option_score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for PlayOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(PlayOption::Rock),
            "B" | "Y" => Ok(PlayOption::Paper),
            "C" | "Z" => Ok(PlayOption::Scissors),
            _ => Err(format!("Invalid option {}", s)),
        }
    }
}

impl PartialOrd for PlayOption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if let PlayOption::Rock = self {
            match other {
                PlayOption::Scissors => Some(Ordering::Greater),
                PlayOption::Paper => Some(Ordering::Less),
                _ => unreachable!(),
            }
        } else if let PlayOption::Paper = self {
            match other {
                PlayOption::Rock => Some(Ordering::Greater),
                PlayOption::Scissors => Some(Ordering::Less),
                _ => unreachable!(),
            }
        } else {
            match other {
                PlayOption::Paper => Some(Ordering::Greater),
                PlayOption::Rock => Some(Ordering::Less),
                _ => unreachable!(),
            }
        }
    }
}

pub enum Expectation {
    Win,
    Lose,
    Draw,
}

impl FromStr for Expectation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Unknown variant {}", s)),
        }
    }
}
impl Expectation {
    fn get_play(&self, enemy_play: &PlayOption) -> PlayOption {
        fn find_option<F: Fn(&PlayOption) -> bool>(predicate: F) -> PlayOption {
            PlayOption::OPTIONS.into_iter().find(predicate).unwrap()
        }

        match self {
            Self::Win => find_option(|x| enemy_play.lt(x)),
            Self::Lose => find_option(|x| enemy_play.gt(x)),
            Self::Draw => find_option(|x| enemy_play.eq(x)),
        }
    }
}

fn calculate_games_result(plays: &[(PlayOption, PlayOption)]) -> u32 {
    plays.iter().fold(0, |accum, (a, b)| {
        let result_points = match b.partial_cmp(a).unwrap() {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        };

        accum + result_points + (b.option_score() as u32)
    })
}

fn part2() {
    let input = include_str!("./input.txt");

    let plays: Vec<(PlayOption, PlayOption)> = input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (
                PlayOption::from_str(a).unwrap(),
                Expectation::from_str(b).unwrap(),
            )
        })
        .map(|(enemy_turn, expectation)| {
            let own_turn = expectation.get_play(&enemy_turn);
            (enemy_turn, own_turn)
        })
        .collect();

    let points = calculate_games_result(plays.as_slice());
    println!("Part 2 {}", points);
}

fn part1() {
    let input = include_str!("./input.txt");

    let plays: Vec<(PlayOption, PlayOption)> = input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            (FromStr::from_str(a).unwrap(), FromStr::from_str(b).unwrap())
        })
        .collect();

    let points = calculate_games_result(plays.as_slice());
    println!("Points {}", points);
}

fn main() {
    part1();
    part2();
}
