use std::collections::HashSet;

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

pub struct Instruction {
    direction: Direction,
    amount: i32,
}

pub fn move_one((start_x, start_y): (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (start_x, start_y + 1),
        Direction::Down => (start_x, start_y - 1),
        Direction::Left => (start_x - 1, start_y),
        Direction::Right => (start_x + 1, start_y),
    }
}

pub fn find_new_tail(
    tail @ (tail_x, tail_y): (i32, i32),
    (head_x, head_y): (i32, i32),
) -> (i32, i32) {
    let x_distance_moved = (tail_x - head_x).abs();
    let y_distance_moved = (tail_y - head_y).abs();
    // if the head is at most one away, the tail doesn't move
    if x_distance_moved <= 1 && y_distance_moved <= 1 {
        return tail;
    }

    match (x_distance_moved, y_distance_moved) {
        // moved diagonally - move both components 1 in the right direction
        (2, 2) => {
            let new_tail_x = if tail_x < head_x {
                tail_x + 1
            } else {
                tail_x - 1
            };
            let new_tail_y = if tail_y < head_y {
                tail_y + 1
            } else {
                tail_y - 1
            };
            (new_tail_x, new_tail_y)
        }
        // only one dimension has moved 2 - tail moves one in that direction; the other dimension ends up the same as where the head has gone
        (2, 0 | 1) if tail_x < head_x => (tail_x + 1, head_y),
        (2, 0 | 1) if tail_x > head_x => (tail_x - 1, head_y),
        (0 | 1, 2) if tail_y < head_y => (head_x, tail_y + 1),
        (0 | 1, 2) if tail_y > head_y => (head_x, tail_y - 1),
        // the previous knot can never have moved more than 2
        // and we handled the cases <2 higher up
        _ => unreachable!(),
    }
}

pub fn input_generator_part1(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (direction, amount) = l.split_once(' ').unwrap();
            let amount = amount.parse::<i32>().unwrap();
            let direction = direction.into();
            Instruction { direction, amount }
        })
        .collect()
}

pub fn solve_part1(input: &Vec<Instruction>) -> usize {
    let start_position = (0, 0);
    let mut head_position = start_position;
    let mut tail_position = start_position;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(tail_position);
    for Instruction { direction, amount } in input {
        for _ in 1..=*amount {
            head_position = move_one(head_position, direction);
            tail_position = find_new_tail(tail_position, head_position);
            visited_positions.insert(tail_position);
        }
    }

    visited_positions.len()
}

pub fn solve_part2(input: &Vec<Instruction>) -> usize {
    let start_position = (0, 0);
    // keep head separate - these are the 'tails'
    let mut remaining_knot_positions = [start_position; 9];
    let mut head_position = start_position;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(remaining_knot_positions[8]);
    for Instruction { direction, amount } in input {
        for _ in 1..=*amount {
            head_position = move_one(head_position, direction);
            let mut last_moved_knot_position = head_position;
            for knot in &mut remaining_knot_positions {
                *knot = find_new_tail(*knot, last_moved_knot_position);
                last_moved_knot_position = *knot;
            }

            visited_positions.insert(remaining_knot_positions[8]);
        }
    }

    visited_positions.len()
}

#[test]
fn test_day9() {
    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    let parsed_input = input_generator_part1(input);
    let visited_count = solve_part1(&parsed_input);
    let knots_visited_count = solve_part2(&parsed_input);

    assert_eq!(visited_count, 13);
    assert_eq!(knots_visited_count, 1);
}

#[test]
fn test_day9_larger() {
    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    let parsed_input = input_generator_part1(input);
    let knots_visited_count = solve_part2(&parsed_input);

    assert_eq!(knots_visited_count, 36);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}