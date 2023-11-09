use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    Vacant,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vacant => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn next_moves(&self) -> Vec<Coord> {
        let new_y = self.y + 1;
        // first straight down, then left, then right
        vec![
            Coord {
                x: self.x,
                y: new_y,
            },
            Coord {
                x: self.x - 1,
                y: new_y,
            },
            Coord {
                x: self.x + 1,
                y: new_y,
            },
        ]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    AtRest(Coord),
    MovedTo(Coord),
    FellOff,
}

pub struct Bounds {
    left: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
}

impl Bounds {
    pub fn unbounded() -> Self {
        Self {
            left: None,
            right: None,
            bottom: None,
        }
    }

    pub fn within_bounds(&self, c: &Coord) -> bool {
        self.left.map(|l| c.x >= l).unwrap_or(true)
            && self.right.map(|r| c.x <= r).unwrap_or(true)
            && self.bottom.map(|b| c.y <= b).unwrap_or(true)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cave {
    map: HashMap<Coord, Tile>,
    bottom: i32,
    left: i32,
    right: i32,
}

impl Cave {
    // in part 2, it turns out that there's a secret floor
    // 2 rows below the bottom -
    // so hack the map to pretend it knows about this
    // (it's infinitely wide, so I'd rather not insert it)
    // technically this would mess up some bounding-boxes, but
    // not the one we use here for part 1
    pub fn get_with_floor(&self, coord: &Coord) -> Option<&Tile> {
        if coord.y == self.bottom + 2 {
            Some(&Tile::Rock)
        } else {
            self.map.get(coord)
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            left: Some(self.left),
            right: Some(self.right),
            bottom: Some(self.bottom),
        }
    }

    pub fn drop_sand(&mut self, bounds: &Bounds) -> Option<()> {
        let sand_start = Coord { x: 500, y: 0 };
        if let Some(Tile::Sand) = self.map.get(&sand_start) {
            // sand already fills the start point - we can't do anything
            return None;
        }

        let mut last_movement = Move::MovedTo(sand_start);
        while let Move::MovedTo(position) = last_movement {
            let new_position = position
                .next_moves()
                .into_iter()
                .map(|c| (c, self.get_with_floor(&c).unwrap_or(&Tile::Vacant)))
                .find(|(_, &t)| matches!(t, Tile::Vacant));
            match new_position {
                Some((c, _)) => {
                    // sand wants to move to this coord
                    if bounds.within_bounds(&c) {
                        last_movement = Move::MovedTo(c)
                    } else {
                        // things that go out of bounds vanish
                        last_movement = Move::FellOff
                    }
                }
                None => last_movement = Move::AtRest(position),
            }
        }

        match last_movement {
            Move::FellOff => None,
            Move::AtRest(position) => {
                self.map.insert(position, Tile::Sand);
                Some(())
            }
            _ => unreachable!(),
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_y = 0;
        // cave grows in part 2, so recalculate these here
        let max_y = self.map.keys().map(|c| c.y).max().unwrap();
        let min_x = self.map.keys().map(|c| c.x).min().unwrap();
        let max_x = self.map.keys().map(|c| c.x).max().unwrap();
        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if row == 0 && col == 500 {
                    write!(f, "+")?;
                } else {
                    let tile = self
                        .map
                        .get(&Coord { x: col, y: row })
                        .unwrap_or(&Tile::Vacant);
                    tile.fmt(f)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

pub fn input_generator_part1(input: &str) -> Cave {
    let mut map = HashMap::new();
    for l in input.lines() {
        let corners = l.split(" -> ");
        let coords = corners.map(|c| {
            let (x, y) = c.split_once(',').unwrap();
            Coord {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        });
        for (first, second) in coords.tuple_windows() {
            if first.y == second.y {
                // moving left or right
                for x in min(first.x, second.x)..=max(first.x, second.x) {
                    map.insert(Coord { x, y: first.y }, Tile::Rock);
                }
            } else if first.x == second.x {
                // moving up or down
                for y in min(first.y, second.y)..=max(first.y, second.y) {
                    map.insert(Coord { x: first.x, y }, Tile::Rock);
                }
            } else {
                // bad input?
                unreachable!()
            }
        }
    }

    // defines the portion of the cave that we scanned
    let bottom = map.keys().map(|c| c.y).max().unwrap();
    let left = map.keys().map(|c| c.x).min().unwrap();
    let right = map.keys().map(|c| c.x).max().unwrap();

    Cave {
        map,
        bottom,
        left,
        right,
    }
}

pub fn solve_part1(input: &Cave) -> usize {
    let mut cave = input.clone();
    let mut counter = 0;
    while cave.drop_sand(&cave.bounds()).is_some() {
        counter += 1;
    }

    counter
}

pub fn solve_part2(input: &Cave) -> usize {
    let mut cave = input.clone();
    let mut counter = 0;
    while cave.drop_sand(&Bounds::unbounded()).is_some() {
        counter += 1;
    }

    counter
}

#[test]
fn test_day14_input1() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 24);
    assert_eq!(part2_result, 93);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}