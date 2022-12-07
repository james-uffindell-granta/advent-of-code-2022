use std::collections::HashSet;
use itertools::Itertools;

type Item = char;

pub fn priority(item: Item) -> u32 {
    let ascii = item as u32;
    if ascii >= 97 {
        ascii - 96
    } else {
        ascii - 64 + 26
    }
}

pub struct Compartment {
    items: HashSet<Item>,
}

pub struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    pub fn common_item(&self) -> Item {
        let mut intersection = self.first_compartment.items.intersection(&self.second_compartment.items);
        *intersection.next().unwrap() as Item
    }

    pub fn items(&self) -> HashSet<Item> {
        self.first_compartment.items.union(&self.second_compartment.items).cloned().collect::<HashSet<_>>()
    }

    pub fn new(input: &str) -> Self {
        let length = input.len();
        let (first, second) = input.split_at(length / 2);
        Self {
            first_compartment: Compartment { items: first.chars().map(|c| c as Item).collect::<HashSet<_>>() },
            second_compartment: Compartment { items: second.chars().map(|c| c as Item).collect::<HashSet<_>>() },
        }
    }
}

pub struct ElfGroup {
    first: Rucksack,
    second: Rucksack,
    third: Rucksack,
}

impl ElfGroup {
    pub fn common_item(&self) -> Item {
        let first_intersection = self.first.items().intersection(&self.second.items()).cloned().collect::<HashSet<_>>();
        let third = &self.third.items();
        let mut intersection = first_intersection.intersection(&third);
        *intersection.next().unwrap() as Item
    }
}

#[aoc_generator(day3, part1)]
pub fn input_generator_part1(input: &str) -> Vec<Rucksack> {
    input.lines().map(|l| Rucksack::new(l)).collect()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_part2(input: &str) -> Vec<ElfGroup> {
    let mut groups = Vec::new();

    for (f, s, t) in input.lines().tuples() {
        groups.push(ElfGroup { first: Rucksack::new(f), second: Rucksack::new(s), third: Rucksack::new(t) });

    }

    groups
}


#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u32 {
    input.iter().map(|r| priority(r.common_item())).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[ElfGroup]) -> u32 {
    input.iter().map(|g| priority(g.common_item())).sum()
}


