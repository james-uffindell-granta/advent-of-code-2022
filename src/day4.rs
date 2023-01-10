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

#[aoc_generator(day4)]
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

#[aoc(day4, part1)]
pub fn solve_part1(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|p| p.first.fully_contains(&p.second) || p.second.fully_contains(&p.first))
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[AssignmentPair]) -> usize {
    input.iter().filter(|p| p.first.overlaps(&p.second)).count()
}
