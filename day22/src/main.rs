use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::{complete::digit1, streaming::char},
    combinator::{map, value},
    multi::many0,
    IResult,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum Cell {
    Vacant,
    Wall,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum Movement {
    TurnLeft,
    TurnRight,
    MoveForward(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    pub fn turn_right(&self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }

    pub fn turn_left(&self) -> Facing {
        match self {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

pub struct Input {
    map: BTreeMap<Coord, Cell>,
    movements: Vec<Movement>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_y = self.map.keys().map(|c| c.y).min().unwrap();
        let max_y = self.map.keys().map(|c| c.y).max().unwrap();
        let min_x = self.map.keys().map(|c| c.x).min().unwrap();
        let max_x = self.map.keys().map(|c| c.x).max().unwrap();

        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if let Some(cell) = self.map.get(&(col, row).into()) {
                    match cell {
                        Cell::Vacant => write!(f, ".")?,
                        Cell::Wall => write!(f, "#")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn trace_path(map: &BTreeMap<Coord, Cell>, movements: &Vec<Movement>) -> (Coord, Facing) {
    let start = *map
        .keys()
        .filter(|&c| c.y == 1)
        .min_by_key(|c| c.x)
        .unwrap();
    let mut current_coord = start;
    let mut current_facing = Facing::Right;
    for movement in movements {
        match movement {
            Movement::TurnLeft => current_facing = current_facing.turn_left(),
            Movement::TurnRight => current_facing = current_facing.turn_right(),
            Movement::MoveForward(num) => {
                'moving: for _ in 1..=(*num) {
                    let next_theoretical_coord = match current_facing {
                        Facing::Up => (current_coord.x, current_coord.y - 1).into(),
                        Facing::Right => (current_coord.x + 1, current_coord.y).into(),
                        Facing::Down => (current_coord.x, current_coord.y + 1).into(),
                        Facing::Left => (current_coord.x - 1, current_coord.y).into(),
                    };
                    if let Some(actual_cell) = map.get(&next_theoretical_coord) {
                        match actual_cell {
                            Cell::Vacant => {
                                current_coord = next_theoretical_coord;
                                continue 'moving;
                            }
                            Cell::Wall => {
                                break 'moving;
                            }
                        }
                    } else {
                        // cell isn't in grid - wrap around
                        let next_coord = match current_facing {
                            Facing::Up => {
                                // we went up off the top - go round to the bottom
                                let next_real_y = map
                                    .keys()
                                    .filter(|&c| c.x == current_coord.x)
                                    .map(|c| c.y)
                                    .max()
                                    .unwrap();
                                (current_coord.x, next_real_y).into()
                            }
                            Facing::Right => {
                                let next_real_x = map
                                    .keys()
                                    .filter(|&c| c.y == current_coord.y)
                                    .map(|c| c.x)
                                    .min()
                                    .unwrap();
                                (next_real_x, current_coord.y).into()
                            }
                            Facing::Down => {
                                let next_real_y = map
                                    .keys()
                                    .filter(|&c| c.x == current_coord.x)
                                    .map(|c| c.y)
                                    .min()
                                    .unwrap();
                                (current_coord.x, next_real_y).into()
                            }
                            Facing::Left => {
                                let next_real_x = map
                                    .keys()
                                    .filter(|&c| c.y == current_coord.y)
                                    .map(|c| c.x)
                                    .max()
                                    .unwrap();
                                (next_real_x, current_coord.y).into()
                            }
                        };

                        if let Some(actual_cell) = map.get(&next_coord) {
                            match actual_cell {
                                Cell::Vacant => {
                                    current_coord = next_coord;
                                    continue 'moving;
                                }
                                Cell::Wall => {
                                    break 'moving;
                                }
                            }
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    (current_coord, current_facing)
}

pub fn on_a(coord: &Coord) -> bool {
    coord.x >= 51 && coord.x <= 100 && coord.y >= 1 && coord.y <= 50
}

pub fn on_b(coord: &Coord) -> bool {
    coord.x >= 101 && coord.x <= 150 && coord.y >= 1 && coord.y <= 50
}

pub fn on_c(coord: &Coord) -> bool {
    coord.x >= 51 && coord.x <= 100 && coord.y >= 51 && coord.y <= 100
}

pub fn on_d(coord: &Coord) -> bool {
    coord.x >= 1 && coord.x <= 50 && coord.y >= 101 && coord.y <= 150
}

pub fn on_e(coord: &Coord) -> bool {
    coord.x >= 51 && coord.x <= 100 && coord.y >= 101 && coord.y <= 150
}

pub fn on_f(coord: &Coord) -> bool {
    coord.x >= 1 && coord.x <= 50 && coord.y >= 151 && coord.y <= 200
}

pub fn trace_path_on_cube(
    map: &BTreeMap<Coord, Cell>,
    movements: &Vec<Movement>,
) -> (Coord, Facing) {
    let start = *map
        .keys()
        .filter(|&c| c.y == 1)
        .min_by_key(|&c| c.x)
        .unwrap();
    let mut current_coord = start;
    let mut current_facing = Facing::Right;
    for movement in movements {
        match movement {
            Movement::TurnLeft => current_facing = current_facing.turn_left(),
            Movement::TurnRight => current_facing = current_facing.turn_right(),
            Movement::MoveForward(num) => {
                'moving: for _ in 1..=(*num) {
                    let next_theoretical_coord = match current_facing {
                        Facing::Up => (current_coord.x, current_coord.y - 1).into(),
                        Facing::Right => (current_coord.x + 1, current_coord.y).into(),
                        Facing::Down => (current_coord.x, current_coord.y + 1).into(),
                        Facing::Left => (current_coord.x - 1, current_coord.y).into(),
                    };
                    if let Some(actual_cell) = map.get(&next_theoretical_coord) {
                        match actual_cell {
                            Cell::Vacant => {
                                current_coord = next_theoretical_coord;
                                continue 'moving;
                            }
                            Cell::Wall => {
                                break 'moving;
                            }
                        }
                    } else {
                        // cell isn't in grid - wrap around but on a cube
                        // no idea if there's a clever way to cope with the input here,
                        // just hardcode the shape
                        //   AB
                        //   C
                        //  DE
                        //  F
                        let (next_coord, next_facing) = match current_facing {
                            Facing::Up => {
                                // three possibilities
                                // going up off the top of D - we end up on the left side
                                // of C
                                if on_d(&current_coord) {
                                    let next_real_x = 51;
                                    let next_real_y = 50 + current_coord.x;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_c(&new_coord));
                                    (new_coord, Facing::Right)
                                } else if on_a(&current_coord) {
                                    // gone up off the top of A - we come in on the left of F
                                    let next_real_x = 1;
                                    let next_real_y = 150 + (current_coord.x - 50);
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_f(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Right)
                                } else if on_b(&current_coord) {
                                    // gone up off the top of B - we come in on the bottom of F
                                    let next_real_x = current_coord.x - 100;
                                    let next_real_y = 200;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_f(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Up)
                                } else {
                                    unreachable!();
                                }
                            }
                            Facing::Right => {
                                if on_b(&current_coord) {
                                    // gone off the right of B - we end up on the right of E
                                    // (but upside down)
                                    let next_real_x = 100;
                                    let next_real_y = 151 - current_coord.y;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_e(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Left)
                                } else if on_c(&current_coord) {
                                    // gone off the right of C - we end up on the bottom of B
                                    let next_real_x = 100 + (current_coord.y - 50);
                                    let next_real_y = 50;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_b(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Up)
                                } else if on_e(&current_coord) {
                                    // gone off the right of E - we end up on the right of B
                                    // (but upside down)
                                    let next_real_x = 150;
                                    let next_real_y = 151 - current_coord.y;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_b(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Left)
                                } else if on_f(&current_coord) {
                                    // gone off the right of F - we end up on the bottom of E
                                    let next_real_x = (current_coord.y - 150) + 50;
                                    let next_real_y = 150;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_e(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Up)
                                } else {
                                    unreachable!();
                                }
                            }
                            Facing::Down => {
                                if on_f(&current_coord) {
                                    // gone off the bottom of F - we come in on the top of B
                                    let next_real_x = 100 + current_coord.x;
                                    let next_real_y = 1;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_b(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Down)
                                } else if on_e(&current_coord) {
                                    // gone off the bottom of E - we end up on the right of F
                                    let next_real_x = 50;
                                    let next_real_y = (current_coord.x - 50) + 150;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_f(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Left)
                                } else if on_b(&current_coord) {
                                    // gone up off the bottom of B - we end up on the right of C
                                    let next_real_x = 100;
                                    let next_real_y = (current_coord.x - 100) + 50;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_c(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Left)
                                } else {
                                    unreachable!();
                                }
                            }
                            Facing::Left => {
                                if on_a(&current_coord) {
                                    // gone off the left of A - we end up on the left of D
                                    // (but upside down)
                                    let next_real_x = 1;
                                    let next_real_y = 151 - current_coord.y;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_d(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Right)
                                } else if on_c(&current_coord) {
                                    // gone off the left of C - end up on the top of D
                                    let next_real_x = current_coord.y - 50;
                                    let next_real_y = 101;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_d(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Down)
                                } else if on_d(&current_coord) {
                                    // gone off the left of D - we end up on the left of A
                                    // (but upside down)
                                    let next_real_x = 51;
                                    let next_real_y = 151 - current_coord.y;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_a(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Right)
                                } else if on_f(&current_coord) {
                                    // gone off the left of F - we end up on the top of A
                                    let next_real_x = (current_coord.y - 150) + 50;
                                    let next_real_y = 1;
                                    let new_coord = (next_real_x, next_real_y).into();
                                    assert!(on_a(&new_coord));

                                    ((next_real_x, next_real_y).into(), Facing::Down)
                                } else {
                                    unreachable!();
                                }
                            }
                        };

                        if let Some(actual_cell) = map.get(&next_coord) {
                            match actual_cell {
                                Cell::Vacant => {
                                    current_coord = next_coord;
                                    current_facing = next_facing;
                                    continue 'moving;
                                }
                                Cell::Wall => {
                                    break 'moving;
                                }
                            }
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    (current_coord, current_facing)
}

pub fn input_generator_part1(input: &str) -> Input {
    let mut map = BTreeMap::new();
    let (map_input, movements_input) = input.split_once("\n\n").unwrap();
    for (row_number, line) in map_input.lines().enumerate() {
        for (col_number, c) in line.chars().enumerate() {
            // use 1-based indexing - will maybe make our lives easier at the end
            let coord = (col_number + 1, row_number + 1).into();
            if c == '#' {
                map.insert(coord, Cell::Wall);
            } else if c == '.' {
                map.insert(coord, Cell::Vacant);
            }
        }
    }

    let movements = parse_movements(&(movements_input.to_owned() + "\n"))
        .unwrap()
        .1;

    Input { map, movements }
}

fn parse_movements(input: &str) -> IResult<&str, Vec<Movement>> {
    let parse_int = map(digit1, |x: &str| x.parse::<usize>().unwrap());
    let parse_move = alt((
        map(parse_int, Movement::MoveForward),
        value(Movement::TurnLeft, char('L')),
        value(Movement::TurnRight, char('R')),
    ));
    many0(parse_move)(input)
}

pub fn solve_part1(input: &Input) -> usize {
    let (coord, facing) = trace_path(&input.map, &input.movements);
    println!("Ended up at {coord:?}, facing {facing:?}");
    (1000 * coord.y) + (4 * coord.x) + facing.score()
}

pub fn solve_part2(input: &Input) -> usize {
    let (coord, facing) = trace_path_on_cube(&input.map, &input.movements);
    println!("Ended up at {coord:?}, facing {facing:?}");
    (1000 * coord.y) + (4 * coord.x) + facing.score()
}

#[test]
fn test_day22_input1() {
    let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    let parsed_input = input_generator_part1(input);
    // println!("Map is:\n{}\n, directions are {:?}", parsed_input, parsed_input.movements);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 6032);
    assert_eq!(part2_result, 5031);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}