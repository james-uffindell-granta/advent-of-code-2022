pub enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Self::Noop
        } else if let Some(value) = s.strip_prefix("addx ") {
            Self::Addx(value.parse::<i32>().unwrap())
        } else {
            unreachable!()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum State {
    Beginning,
    StillAdding,
}

pub fn input_generator_part1(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.into()).collect()
}

pub fn solve_part1(input: &Vec<Instruction>) -> i32 {
    let mut current_cycle_number = 1;
    let mut next_cycle_to_save = 20;
    let mut register_value = 1;
    let mut cycle_values_to_use = Vec::new();
    for i in input {
        // remember this one
        if current_cycle_number == next_cycle_to_save {
            cycle_values_to_use.push((next_cycle_to_save, register_value));
            next_cycle_to_save += 40;
        }

        match i {
            Instruction::Noop => {
                current_cycle_number += 1;
            }
            Instruction::Addx(amount) => {
                // add takes 2 steps - check the one we'll skip
                if current_cycle_number + 1 == next_cycle_to_save {
                    cycle_values_to_use.push((next_cycle_to_save, register_value));
                    next_cycle_to_save += 40;
                }

                current_cycle_number += 2;
                register_value += amount;
            }
        }
    }

    cycle_values_to_use.iter().take(6).map(|(c, v)| c * v).sum()
}

pub fn sprite_overlaps(sprite_center: i32, location: i32) -> bool {
    sprite_center == location || sprite_center - 1 == location || sprite_center + 1 == location
}

pub fn solve_part2(input: &[Instruction]) -> String {
    let mut register_value = 1;
    let mut current_row = String::new();
    let mut rows = Vec::new();

    let mut instructions = input.iter();
    let mut current_state = State::Beginning;
    let mut current_instruction = instructions.next().unwrap();

    for cycle_number in 1..=240 {
        let column_number = (cycle_number - 1) % 40;
        current_row += if sprite_overlaps(register_value, column_number) {
            "#"
        } else {
            "."
        };

        match (current_state, current_instruction) {
            (State::Beginning, Instruction::Noop) => {
                // nothing more to do; fetch the next instruction
                current_instruction = instructions.next().unwrap_or(&Instruction::Noop);
            }
            (State::Beginning, Instruction::Addx(_)) => {
                // this takes two cycles, so we need to enter the still-adding state for next time round
                current_state = State::StillAdding;
            }
            (State::StillAdding, Instruction::Addx(value)) => {
                // this is our second cycle of the add, so finish it off
                register_value += value;
                current_state = State::Beginning;
                current_instruction = instructions.next().unwrap_or(&Instruction::Noop);
            }
            _ => unreachable!(),
        }

        // this cycle we just handled was the last one in a row
        if cycle_number % 40 == 0 {
            rows.push(current_row.clone());
            current_row = String::new();
        }
    }

    let mut output = String::from("\n");
    for r in rows {
        output += &r;
        output += "\n"
    }

    output
}

#[test]
fn test_day10_input1() {
    let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    let result = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    let parsed_input = input_generator_part1(input);
    let signal_strength = solve_part1(&parsed_input);
    let letters = solve_part2(&parsed_input);

    assert_eq!(signal_strength, 13140);
    assert_eq!(letters, result);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}