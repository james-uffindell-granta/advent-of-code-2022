pub struct SectionRange {
    start_id: u32,
    end_id: u32,
}

impl SectionRange {
    pub fn fully_contains(&self, other: &SectionRange) -> bool {
        self.start_id <= other.start_id && self.end_id >= other.end_id
    }

    pub fn overlaps(&self, other: &SectionRange) -> bool {
        // only way they can't overlap is if one starts after the other ends
        !(self.end_id < other.start_id || other.end_id < self.start_id)
    }
}

pub struct AssignmentPair {
    first: SectionRange,
    second: SectionRange,
}

pub fn input_generator_part1(input: &str) -> Vec<AssignmentPair> {
    let mut pairs = Vec::new();
    for l in input.lines() {
        let (first, second) = l.split_once(',').unwrap();
        let (first_start, first_end) = first.split_once('-').unwrap();
        let (second_start, second_end) = second.split_once('-').unwrap();
        pairs.push(AssignmentPair {
            first: SectionRange {
                start_id: first_start.parse::<u32>().unwrap(),
                end_id: first_end.parse::<u32>().unwrap(),
            },
            second: SectionRange {
                start_id: second_start.parse::<u32>().unwrap(),
                end_id: second_end.parse::<u32>().unwrap(),
            },
        });
    }

    pairs
}

pub fn solve_part1(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|p| p.first.fully_contains(&p.second) || p.second.fully_contains(&p.first))
        .count()
}

pub fn solve_part2(input: &[AssignmentPair]) -> usize {
    input.iter().filter(|p| p.first.overlaps(&p.second)).count()
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
