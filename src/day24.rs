use std::{
    collections::{
        HashMap, HashSet
    },
    ops::Add,
};
use multimap::MultiMap;
use num::integer::lcm;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: u8,
    y: u8,
}

impl From<(u8, u8)> for Coord {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

impl Add<(u8, u8)> for Coord {
    type Output = Coord;

    fn add(self, (other_x, other_y): (u8, u8)) -> Self::Output {
        Self::Output { x: self.x + other_x, y: self.y + other_y }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blizzard {
    current_coord: Coord,
    direction: Direction,
}


#[derive(Clone)]
pub struct Input {
    blizzards: Vec<Blizzard>,
    height: u8,
    width: u8,
}

impl Input {
    pub fn move_blizzards(&mut self) {
        for mut b in &mut *self.blizzards {
            let c = b.current_coord;
            let new_position = match b.direction {
                // blizzards do wrap around
                Direction::Up => if c.y == 1 {(c.x, self.height).into()} else {(c.x, c.y - 1).into()},
                Direction::Down => if c.y == self.height {(c.x, 1).into()} else {(c.x, c.y + 1).into()},
                Direction::Left => if c.x == 1 {(self.width, c.y).into()} else {(c.x - 1, c.y).into()},
                Direction::Right => if c.x == self.width {(1, c.y).into()} else {(c.x + 1, c.y).into()},
            };
            b.current_coord = new_position;
        }
    }

    pub fn get_elf_moves_from(&self, c: Coord) -> Vec<Coord> {
        let mut ret = Vec::with_capacity(5);
        ret.push(c);
        // elves can't wrap around
        if c.x > 1 { ret.push((c.x - 1, c.y).into()); } 
        if c.x < self.width { ret.push((c.x + 1, c.y).into()); } 
        if c.y > 1 { ret.push((c.x, c.y - 1).into()); }
        if c.y < self.height { ret.push((c.x, c.y + 1).into()); }
        ret
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self.blizzards.iter()
            .map(|b| (b.current_coord, b.direction)).collect::<MultiMap<_, _>>();
        for row in 1..=self.height {
            for col in 1..=self.width {
                if let Some(v) = map.get_vec(&(col, row).into()) {
                    if v.len() > 1 {
                        write!(f, "{}", v.len())?;
                    } else {
                        match v[0] {
                            Direction::Up => write!(f, "^")?,
                            Direction::Down => write!(f, "v")?,
                            Direction::Left => write!(f, "<")?,
                            Direction::Right => write!(f, ">")?,
                        };
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// find the fastest time from the start to the target.
// this assumes we start off one square away from the start in a tile that will never be
// hit by a blizzard - so it will take us 1 minute to move to the start - and it assumes
// our 'real' target is similar, but one square away from the target passed into this method.
pub fn find_fastest_time(input: &mut Input, start: Coord, target: Coord) -> i32 {
    let no_options = HashSet::new();

    // the blizzards repeat after this long - if we were somewhere this long ago, there's
    // no point in going there again (we'd just repeat; would be better off with another
    // path from that square back then)
    // this is 600 for me, I think
    let repeat_time = lcm(input.height as i32, input.width as i32);

    // remember where we could have got to, each minute.
    let mut possible_locations_at_time = HashMap::new();
    // at minute 0, there's nowhere we can be (we aren't in the grid yet)
    possible_locations_at_time.insert(0, HashSet::<Coord>::new());

    for minute in 1.. {
        // see where we could have been last minute
        let places_last_minute = possible_locations_at_time.get(&(minute - 1)).unwrap();
        // move the blizzards forward - see where they end up (we can't move to those squares)
        input.move_blizzards();
        let blizzard_locations = input.blizzards.iter().map(|b| b.current_coord).collect::<HashSet<_>>();
        // see where we were one blizzard cycle ago - no point going back to any of those squares either
        // (we'd just form a pointless loop)
        let positions_one_repeat_ago = if minute < repeat_time {
            &no_options
        } else {
            possible_locations_at_time.get(&(minute - repeat_time)).unwrap()
        };

        let positions_reachable_at_current_minute = places_last_minute.iter()
             // everywhere we can move to from last time
            .flat_map(|c| input.get_elf_moves_from(*c))
            // plus the first square (could always wait indefinitely at start before moving here)
            .chain(std::iter::once(start)) 
            // can't go anywhere a blizzard is
            .filter(|c| !blizzard_locations.contains(c)) 
            // shouldn't go anywhere we were one blizzard cycle ago
            // (I think we actually never hit this, but for a bigger board maybe we would)
            .filter(|c| !positions_one_repeat_ago.contains(c) ) 
            .collect::<HashSet<_>>();

        // this is the first minute we could have made it to the target - bail out
        // (it takes us one more minute to reach the 'real' target)
        if positions_reachable_at_current_minute.contains(&target) {
           return minute + 1;
        }

        // otherwise remember our new options and keep going
        possible_locations_at_time.insert(minute, positions_reachable_at_current_minute);
    }

    unreachable!()

}

#[aoc_generator(day24)]
pub fn input_generator_part1(input: &str) -> Input {
    let mut blizzards = Vec::new();
    // ignore the # borders on the input - only consider the inner grid
    let row_count = input.lines().count() - 2;
    let col_count = input.lines().next().unwrap().len() - 2;
    for (row_number, line) in input.lines().enumerate() {
        for (col_number, c) in line.chars().enumerate() {

            let coord = (col_number as u8, row_number as u8).into();
            match c {
                '^' => blizzards.push(Blizzard { current_coord: coord, direction: Direction::Up }),
                'v' => blizzards.push(Blizzard { current_coord: coord, direction: Direction::Down }),
                '>' => blizzards.push(Blizzard { current_coord: coord, direction: Direction::Right }),
                '<' => blizzards.push(Blizzard { current_coord: coord, direction: Direction::Left }),
                _ => {},
            }
        }
    }

    Input { blizzards, width: col_count as u8, height: row_count as u8 }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let mut input = input.clone();
    let (width, height) = (input.width, input.height);
    find_fastest_time(&mut input, (1, 1).into(), (width, height).into())
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Input) -> i32 {
    let mut input = input.clone();
    let (width, height) = (input.width, input.height);

    // fastest route there, back, and there again is the same as doing each route as fast
    // as possible.
    // even if it's better to wait a bit before starting the journey back, there's no point
    // getting to the original target any later - any extra time spent on a longer journey
    // there could have just been spent waiting at the target before setting off again,
    // and our logic already handles that.

    let fastest_time_to_end = find_fastest_time(&mut input, (1, 1).into(), (width, height).into());
    // for the repeat journey, we want to reuse the blizzard state from where it ended up
    // (but we have to fast-forward by one minute, to account for our move to the 'real' target)
    input.move_blizzards();
    // we _don't_ want to reuse any of our move history, though, because this is a different journey
    let fastest_time_back = find_fastest_time(&mut input, (width, height).into(), (1, 1).into());
    // asame again
    input.move_blizzards();
    let fastest_time_to_end_again = find_fastest_time(&mut input, (1, 1).into(), (width, height).into());
    fastest_time_to_end + fastest_time_back + fastest_time_to_end_again
}

#[test]
fn test_day24_input1() {
    let input =
r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 18);
    assert_eq!(part2_result, 54);
}