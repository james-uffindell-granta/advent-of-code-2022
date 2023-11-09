pub struct Elf {
    pub calories: Vec<u32>,
}

impl Elf {
    pub fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

pub fn input_generator(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut current_elf = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            elves.push(Elf {
                calories: current_elf,
            });
            current_elf = Vec::new();
        }

        if let Ok(calories) = l.parse::<u32>() {
            current_elf.push(calories)
        }
    }

    elves
}

pub fn solve_part1(input: &[Elf]) -> u32 {
    input.iter().map(|e| e.total_calories()).max().unwrap_or(0)
}

pub fn solve_part2(input: &[Elf]) -> u32 {
    let mut elves = input.iter().map(|e| e.total_calories()).collect::<Vec<_>>();
    elves.sort();
    elves.iter().rev().take(3).sum()
}


fn main() {
    let input = input_generator(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
