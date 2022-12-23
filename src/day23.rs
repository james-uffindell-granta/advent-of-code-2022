use std::{
    collections::{
        HashSet, VecDeque,
    },
    ops::Add
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn neighbours(self) -> Vec<Coord> {
        vec![
            self + (0, 1),
            self + (1, 1),
            self + (1, 0), 
            self + (1, -1),
            self + (0, -1),
            self + (-1, -1),
            self + (-1, 0),
            self + (-1, 1),
        ]
    }

    // ---> increasing x
    // |
    // v
    // increasing y
    fn neighbours_in_direction(self, direction: &Direction) -> Vec<Coord> {
        match direction {
            Direction::West => vec![self + (-1, -1), self + (-1, 0), self + (-1, 1)],
            Direction::East => vec![self + (1, -1), self + (1, 0), self + (1, 1)],
            Direction::North => vec![self + (-1, -1), self + (0, -1), self + (1, -1)],
            Direction::South => vec![self + (-1, 1), self + (0, 1), self + (1, 1)],
        }
    }
}

impl From<(i64, i64)> for Coord {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl Add<(i64, i64)> for Coord {
    type Output = Coord;

    fn add(self, (other_x, other_y): (i64, i64)) -> Self::Output {
        Self::Output { x: self.x + other_x, y: self.y + other_y }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn as_delta(&self) -> (i64, i64) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    current_coord: Coord,
    // proposed_direction: Option<Direction>,
    proposed_coord: Option<Coord>,
}

#[derive(Clone)]
pub struct Input {
    elves: Vec<Elf>,
}

impl Input {
    pub fn bounding_x_range(&self) -> (i64, i64) {
        let min_x = self.elves.iter().map(|e| e.current_coord.x).min().unwrap();
        let max_x = self.elves.iter().map(|e| e.current_coord.x).max().unwrap();
        (min_x, max_x)
    }

    pub fn bounding_y_range(&self) -> (i64, i64) {
        let min_y = self.elves.iter().map(|e| e.current_coord.y).min().unwrap();
        let max_y = self.elves.iter().map(|e| e.current_coord.y).max().unwrap();
        (min_y, max_y)
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self.elves.iter().map(|e| e.current_coord).collect::<HashSet<_>>();
        let (min_y, max_y) = self.bounding_y_range();
        let (min_x, max_x) = self.bounding_x_range();

        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if let Some(_) = map.get(&(col, row).into()) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[aoc_generator(day23)]
pub fn input_generator_part1(input: &str) -> Input {
    let mut elves = Vec::new();
    for (row_number, line) in input.lines().enumerate() {
        for (col_number, c) in line.chars().enumerate() {
            let coord = (col_number as i64, row_number as i64).into();
            if c == '#' {
                elves.push(Elf { current_coord: coord, proposed_coord: None });
            }
        }
    }

    Input { elves }
}

pub fn step(elves: &mut [Elf], direction_order: &VecDeque<Direction>) -> bool {
    let mut moved = false;
    // part 1: propose a move
    let current_elf_locations = elves.iter().map(|e| e.current_coord).collect::<HashSet<_>>();
    let mut proposed_elf_locations = HashSet::new();
    let mut duplicate_proposed_elf_locations = HashSet::new();
    for mut elf in &mut *elves {
        if elf.current_coord.neighbours().iter().any(|c| current_elf_locations.contains(c)) {
            'directions: for d in direction_order {
                if elf.current_coord.neighbours_in_direction(d).iter().any(|c| current_elf_locations.contains(c)) {
                    // there's already an elf over there - don't go that way
                } else {
                    // found a direction with no elves: move that way
                    let new_coord = elf.current_coord + d.as_delta();
                    elf.proposed_coord = Some(new_coord);
                    // remember it, to check for duplicates
                    if !proposed_elf_locations.insert(new_coord) {
                        // one elf has already tried to go here - remember this as a duplicate
                        duplicate_proposed_elf_locations.insert(new_coord);
                    }
                    break 'directions;
                }
            }
            // has at least one elf nearby - needs to move
        } else {
            // for now use 'none' to mean 'won't move' as well as 'hasn't chosen yet'
            elf.proposed_coord = None;
        }
    }

    // part 2: move elves (but only if they were the only elf to suggest moving there)
    for mut elf in &mut *elves {
        match elf.proposed_coord {
            None => {
                // do nothing - current coord is still fine
            },
            Some(new_coord) => {
                if !duplicate_proposed_elf_locations.contains(&new_coord) {
                    elf.current_coord = new_coord;
                    moved = true;
                } else {
                    // can't move
                }
                // reset for next step
                elf.proposed_coord = None;
            }
        }
    }

    moved

}


#[aoc(day23, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let mut direction_order = VecDeque::from([
        Direction::North, Direction::South, Direction::West, Direction::East]);

    let mut input = input.clone();
    let step_count =10;
    for _ in 1..=step_count {
        step(&mut input.elves, &direction_order);
        // move the directions
        let first_direction = direction_order.pop_front().unwrap();
        direction_order.push_back(first_direction);
    }

    let (min_y, max_y) = input.bounding_y_range();
    let (min_x, max_x) = input.bounding_x_range();
    // both bounds are inclusive
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    area - (input.elves.len() as i64)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut direction_order = VecDeque::from([
        Direction::North, Direction::South, Direction::West, Direction::East]);

    let mut input = input.clone();
    for round_number in 1.. {
        let moved = step(&mut input.elves, &direction_order);
        if !moved {
            return round_number;
        }
        // move the directions
        let first_direction = direction_order.pop_front().unwrap();
        direction_order.push_back(first_direction);
    }
    
    unreachable!()
}

#[test]
fn test_day23_input1() {
    let input =
r#".....
..##.
..#..
.....
..##.
.....
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 25);
    assert_eq!(part2_result, 4);
}

#[test]
fn test_day23_input2() {
    let input =
r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 110);
    assert_eq!(part2_result, 20);
}
