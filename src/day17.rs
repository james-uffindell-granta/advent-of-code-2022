use std::{collections::{HashSet, HashMap}, ops::Add};
pub struct Input {}

// |..@@@@.|
// |.......|
// |.......|
// |.......| y == 0
// +-------+ 
//  x = 0  x = 7


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i32,
    y: i64,
}

impl From<(i32, i64)> for Coord {
    fn from((x, y): (i32, i64)) -> Self {
        Self { x, y }
    }
}

impl Add<(i32, i64)> for Coord {
    type Output = Coord;

    fn add(self, (other_x, other_y): (i32, i64)) -> Self::Output {
        Self::Output { x: self.x + other_x, y: self.y + other_y }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Rock {
    Bar,
    Plus,
    Corner,
    Pipe,
    Box,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn coord_delta(&self) -> (i32, i64) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        }
    }
}

impl Rock {
    pub fn sequence() -> Vec<Rock> {
        vec![Rock::Bar, Rock::Plus, Rock::Corner, Rock::Pipe, Rock::Box]
    }

    // coordinate deltas relative to start position (in piece bottom-left)
    pub fn relative_coords(&self, start: Coord) -> HashSet<Coord> {
        match self {
            Rock::Bar =>
                HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)].map(|c| start + c)),
            Rock::Pipe =>
                HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)].map(|c| start + c)),
            Rock::Box =>
                HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)].map(|c| start + c)),
            Rock::Corner =>
                HashSet::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].map(|c| start + c)),
            Rock::Plus =>
                HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)].map(|c| start + c)),
        }
    }

    // the new coords a rock would fill, if it moved this way, which it isn't currently filling
    pub fn relative_movefront_coords(&self, start: Coord, direction: &Direction) -> HashSet<Coord> {
        let delta = direction.coord_delta();
        let rock_coords = self.relative_coords(start);
        let new_coords = rock_coords.clone().into_iter().map(|c| c + delta).collect::<HashSet<_>>();
        new_coords.difference(&rock_coords).cloned().collect::<HashSet<_>>()
    }
}

pub enum MoveResult {
    RockLanded,
    Boring,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct State {
    well: Vec<Coord>,
    jet_number: usize,
    rock: Rock,
}

pub struct Game {
    // "current rock location" is always its bounding box's bottom left
    current_rock: (Rock, Coord),
    current_pit: HashSet<Coord>,
    number_of_rocks: usize,
    // a well is: the coords above the 'baseline' (lowest settled rock in any column),
    // plus the offset into the gas jet sequence we saw it at, plus the rock we just placed
    seen_wells: HashMap<State, (usize, i64)>,
    // last time we were here: the number of the rock we placed, and our height then
    last_time_here: Option<(usize, i64)>,
}

impl Game {
    pub fn with_starting_rock(start: Rock) -> Self {
        Self {
            current_rock: (start, (2, 3).into()),
            current_pit: HashSet::new(),
            number_of_rocks: 1,
            seen_wells: HashMap::new(),
            last_time_here: None,
        }
    }

    pub fn baseline(&self) -> i64 {
        (0..7).map(|x| {
            self.current_pit.iter().filter(|c| c.x == x).map(|c| c.y).max().unwrap_or(0)
        }).min().unwrap_or(0)
    }

    pub fn current_well(&self) -> Vec<Coord> {
        let baseline = self.baseline();
        let mut rocks_above_baseline = self.current_pit.iter().filter(|c| c.y >= baseline).map(|c| *c + (0, -baseline)).collect::<Vec<_>>();
        rocks_above_baseline.sort();
        rocks_above_baseline
    }

    pub fn make_move(&mut self, (jet_number, direction): (usize, &Direction)) -> MoveResult {
        let places_rock_wants_to_move_to =
            self.current_rock.0.relative_movefront_coords(self.current_rock.1, direction);
        if places_rock_wants_to_move_to.iter().any(|c| {
            c.x < 0 || c.x >= 7
            || c.y < 0
            || self.current_pit.contains(c)    
        }) {
            // can't move - there's something in the way
            if matches!(direction, Direction::Down) {
                // tried to move down, but can't - we've landed
                self.current_pit.extend(&self.current_rock.0.relative_coords(self.current_rock.1));

                if self.number_of_rocks % 100 == 0 {
                    // println!("Another hundred rocks placed, new baseline is {}", self.baseline());
                }
                // no need to do anything with the rock coords - it's already been merged with the pit
                let new_well = State { well: self.current_well(), jet_number, rock: self.current_rock.0 };
                if let Some((rock_number, height)) = self.seen_wells.get(&new_well) {
                    // println!("Seen this well before - last time was after placing rock number {}, now is after placing rock number {}", rock_number, self.number_of_rocks);
                    self.last_time_here = Some((*rock_number, *height));
                } else {
                    self.seen_wells.insert(new_well, (self.number_of_rocks, self.current_height()));
                }

                return MoveResult::RockLanded;
            }

            return MoveResult::Boring;
        }

        //otherwise we moved successfully
        self.current_rock.1 = self.current_rock.1 + direction.coord_delta();
        MoveResult::Boring
    }

    pub fn current_height(&self) -> i64 {
        self.current_pit.iter().map(|c| c.y).max().unwrap_or(0)
    }

    pub fn spawn_new_rock(&mut self, rock: Rock) {
        let height = self.current_height();
        self.current_rock = (rock, (2, height + 4).into());
        self.number_of_rocks += 1;
    }

}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_rock_coords = self.current_rock.0.relative_coords(self.current_rock.1);
        let mut y = self.current_height() + 7;
        loop {
            y -= 1;

            if y == -1 {
                writeln!(f, "+-------+")?;
                break;
            }

            write!(f, "|")?;
            for x in 0..7 {
                let coord = (x, y).into();
                if current_rock_coords.contains(&coord) {
                    write!(f, "@")?;
                } else if self.current_pit.contains(&coord) {
                    write!(f, "#")?;
                }
                else {
                    write!(f, ".")?;
                }
            }


            write!(f, "|\n")?;
        }
        Ok(())
    }
}


#[aoc_generator(day17)]
pub fn input_generator_part1(input: &str) -> Vec<Direction> {
    input.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!(),
    }).collect()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Vec<Direction>) -> i64 {
    let mut rocks = Rock::sequence().into_iter().cycle();
    let movements = input.iter().enumerate().cycle().map(|(c, d)| [(c, d), (c, &Direction::Down)]).flatten();
    let first_rock = rocks.next().unwrap();
    let mut game = Game::with_starting_rock(first_rock);
    println!("{}", game);

    let answer = 0;
    for m in movements {
        let result = game.make_move(m);
        match result {
            MoveResult::Boring => continue,
            MoveResult::RockLanded => {
                // check for end condition,
                if game.number_of_rocks == 2022 {
                    return game.current_height() + 1;
                }
                // otherwise spawn new rock
                game.spawn_new_rock(rocks.next().unwrap());
                // println!("new rock:");
                // println!("{}", game);
            }
        }
    }

    answer
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Vec<Direction>) -> i64 {
    let mut rocks = Rock::sequence().into_iter().cycle();
    let movements = input.iter().enumerate().cycle().map(|(c, d)| [(c, d), (c, &Direction::Down)]).flatten();
    let first_rock = rocks.next().unwrap();
    let mut game = Game::with_starting_rock(first_rock);
    let number_of_rocks_to_find = 1_000_000_000_000i64;

    let mut remainder = 0;
    let mut height_grown = 0;

    for m in movements {
        let result = game.make_move(m);
        match result {
            MoveResult::Boring => continue,
            MoveResult::RockLanded => {
                if let Some((old_rock_number, old_height)) = game.last_time_here {
                    // we've hit a loop!
                    let new_rock_number = game.number_of_rocks;
                    let new_height = game.current_height();
                    println!("Found a cycle: old rock {} and new rock {}", old_rock_number, new_rock_number);

                    // this is the number of rocks to make up after the cycle starts
                    let number_of_rocks_to_make_up = number_of_rocks_to_find - (old_rock_number as i64);
                    let number_of_rocks_grown_in_cycle = new_rock_number - old_rock_number;
                    let height_grown_in_cycle = new_height - old_height;
                    let number_of_times_to_run_cycle = number_of_rocks_to_make_up / (number_of_rocks_grown_in_cycle as i64);
                    let remainder_to_make_up = number_of_rocks_to_make_up % (number_of_rocks_grown_in_cycle as i64);

                    remainder = remainder_to_make_up + (old_rock_number as i64);

                    height_grown = height_grown_in_cycle * number_of_times_to_run_cycle;
                    break;
                }

                // otherwise spawn new rock
                game.spawn_new_rock(rocks.next().unwrap());
                // println!("new rock:");
                // println!("{}", game);
            }
        }
    }

    // rerun from the start, but only until we hit the remainder we need
    if remainder > 0 {
        let mut rocks = Rock::sequence().into_iter().cycle();
        let movements = input.iter().enumerate().cycle().map(|(c, d)| [(c, d), (c, &Direction::Down)]).flatten();
        let first_rock = rocks.next().unwrap();
        let mut game = Game::with_starting_rock(first_rock);
        for m in movements {
            let result = game.make_move(m);
            match result {
                MoveResult::Boring => continue,
                MoveResult::RockLanded => {
                    // check for end condition,
                    if game.number_of_rocks == (remainder as usize) {
                        return height_grown + game.current_height() + 1;
                    }
                    // otherwise spawn new rock
                    game.spawn_new_rock(rocks.next().unwrap());
                    // println!("new rock:");
                    // println!("{}", game);
                }
            }
        }
    }

    height_grown
}

#[test]
fn test_day17_input1() {
    let input =
r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    println!("{}", part1_result);
    let part2_result = solve_part2(&parsed_input);

    println!("{}", part2_result);
    // assert_eq!(part1_result, 3068);
    // assert_eq!(part2_result, 0);
}