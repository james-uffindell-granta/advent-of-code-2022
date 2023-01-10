use std::collections::HashMap;

type Stack = Vec<char>;
type Dock = HashMap<u32, Stack>;

#[derive(PartialEq, Eq, Debug)]
pub struct Instruction {
    start_stack: u32,
    end_stack: u32,
    number_to_move: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Input {
    starting_layout: Dock,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day5)]
pub fn input_generator_part1(input: &str) -> Input {
    // start the dock off with something - if we find more columns we'll add them as we go
    let mut dock: Dock = (1..2).map(|i| (i, Vec::new())).collect();
    let mut instructions = Vec::new();
    for l in input.lines() {
        // the line specifies some crates - add them to the dock
        let crates = l.match_indices('[');
        for (idx, _) in crates {
            let stack_number = (idx / 4 + 1) as u32;
            let _crate = l.chars().nth(idx + 1).unwrap();
            dock.entry(stack_number)
                .or_insert(Vec::new())
                .insert(0, _crate);
        }

        if l.starts_with("move") {
            let mut words = l.split_ascii_whitespace();
            _ = words.next();
            let number_to_move = words.next().unwrap().parse::<usize>().unwrap();
            _ = words.next();
            let starting_stack = words.next().unwrap().parse::<u32>().unwrap();
            _ = words.next();
            let ending_stack = words.next().unwrap().parse::<u32>().unwrap();
            instructions.push(Instruction {
                number_to_move,
                start_stack: starting_stack,
                end_stack: ending_stack,
            })
        }
    }

    Input {
        starting_layout: dock,
        instructions,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    let mut dock = input.starting_layout.clone();
    for i in &input.instructions {
        for _ in 0..i.number_to_move {
            let _crate = dock.get_mut(&i.start_stack).unwrap().pop().unwrap();
            dock.get_mut(&i.end_stack).unwrap().push(_crate);
        }
    }

    let number_of_stacks = dock.keys().count() as u32;
    (1..=number_of_stacks)
        .map(|i| dock.get_mut(&i).unwrap().pop().unwrap())
        .collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let mut dock = input.starting_layout.clone();
    for i in &input.instructions {
        let mut temp_stack = Vec::new();
        for _ in 0..i.number_to_move {
            let _crate = dock.get_mut(&i.start_stack).unwrap().pop().unwrap();
            temp_stack.push(_crate);
        }

        for _ in 0..i.number_to_move {
            let _crate = temp_stack.pop().unwrap();
            dock.get_mut(&i.end_stack).unwrap().push(_crate);
        }
    }

    let number_of_stacks = dock.keys().count() as u32;
    (1..=number_of_stacks)
        .map(|i| dock.get_mut(&i).unwrap().pop().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

        let parsed_input = input_generator_part1(input);
        let expected = Input {
            starting_layout: HashMap::from([
                (1, vec!['Z', 'N']),
                (2, vec!['M', 'C', 'D']),
                (3, vec!['P']),
            ]),
            instructions: vec![
                Instruction {
                    number_to_move: 1,
                    start_stack: 2,
                    end_stack: 1,
                },
                Instruction {
                    number_to_move: 3,
                    start_stack: 1,
                    end_stack: 3,
                },
                Instruction {
                    number_to_move: 2,
                    start_stack: 2,
                    end_stack: 1,
                },
                Instruction {
                    number_to_move: 1,
                    start_stack: 1,
                    end_stack: 2,
                },
            ],
        };

        assert_eq!(parsed_input, expected);
    }

    #[test]
    fn test_instructions_part_1() {
        let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

        let parsed_input = input_generator_part1(input);
        let result = solve_part1(&parsed_input);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_instructions_part_2() {
        let input = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

        let parsed_input = input_generator_part1(input);
        let result = solve_part2(&parsed_input);

        assert_eq!(result, "MCD");
    }
}
