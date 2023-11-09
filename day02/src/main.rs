#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
} 

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

pub struct Game {
    pub me: Play,
    pub opponent: Play,
}

impl Game {
    pub fn outcome(&self) -> Outcome {
        match (self.me, self.opponent) {
            (Play::Rock, Play::Scissors)
            | (Play::Scissors, Play::Paper)
            | (Play::Paper, Play::Rock) => Outcome::Win,
            (Play::Rock, Play::Rock)
            | (Play::Scissors, Play::Scissors)
            | (Play::Paper, Play::Paper) => Outcome::Draw,
            (Play::Rock, Play::Paper)
            | (Play::Scissors, Play::Rock)
            | (Play::Paper, Play::Scissors) => Outcome::Lose,
        }
    }

    pub fn score(&self) -> u32 {
        self.me.score() + self.outcome().score()
    }
}

pub struct Strategy {
    pub opponent: Play,
    pub desired_outcome: Outcome,
}

impl Strategy {
    pub fn choose_play(&self) -> Play {
        match self.desired_outcome {
            Outcome::Draw => self.opponent,
            Outcome::Win => match self.opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            Outcome::Lose => match self.opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
        }
    }
}

pub fn convert_opponent(play: char) -> Play {
    match play {
        'A' => Play::Rock,
        'B' => Play::Paper,
        'C' => Play::Scissors,
        _ => unreachable!(),
    }
}

pub fn convert_mine(play: char) -> Play {
    match play {
        'X' => Play::Rock,
        'Y' => Play::Paper,
        'Z' => Play::Scissors,
        _ => unreachable!(),
    }
}

pub fn convert_outcome(play: char) -> Outcome {
    match play {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => unreachable!(),
    }
}

pub fn input_generator_part1(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let opp = chars.next().unwrap();
            chars.next().unwrap();
            let m = chars.next().unwrap();
            Game {
                opponent: convert_opponent(opp),
                me: convert_mine(m),
            }
        })
        .collect()
}

pub fn input_generator_part2(input: &str) -> Vec<Strategy> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let opp = chars.next().unwrap();
            chars.next().unwrap();
            let o = chars.next().unwrap();
            Strategy {
                opponent: convert_opponent(opp),
                desired_outcome: convert_outcome(o),
            }
        })
        .collect()
}

pub fn solve_part1(input: &[Game]) -> u32 {
    input.iter().map(|g| g.score()).sum()
}

pub fn solve_part2(input: &[Strategy]) -> u32 {
    input
        .iter()
        .map(|s| {
            Game {
                opponent: s.opponent,
                me: s.choose_play(),
            }
            .score()
        })
        .sum()
}

fn main() {
    let input_1 = input_generator_part1(include_str!("../input.txt"));
    let part_1 = solve_part1(&input_1);

    let input_2 = input_generator_part2(include_str!("../input.txt"));
    let part_2  = solve_part2(&input_2);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
