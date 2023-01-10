use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Entry {
    value: i64,
    original_index: usize,
}

impl Entry {
    pub fn with_value(v: i64, idx: usize) -> Self {
        Self {
            value: v,
            original_index: idx,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator_part1(input: &str) -> Vec<Entry> {
    input
        .lines()
        .enumerate()
        .map(|(idx, l)| Entry::with_value(l.parse().unwrap(), idx))
        .collect()
}

pub fn move_forward_wrapping(len: usize, num: usize) -> usize {
    if num + 1 < len {
        num + 1
    } else {
        0
    }
}

pub fn move_backward_wrapping(len: usize, num: usize) -> usize {
    if num == 0 {
        len - 1
    } else {
        num - 1
    }
}

pub fn shuffle_vector_by_original_index(input: &mut Vec<Entry>) {
    let number = input.len();
    for idx in 0..number {
        if let Some(index) = input.iter().position(|e| e.original_index == idx) {
            let entry_to_move = input.get_mut(index).unwrap();
            // because of how the wraparound works, moving by "length of vector - 1" is the same as
            // not moving, so mod it out
            let amount_to_move = entry_to_move.value % ((number as i64) - 1);

            match amount_to_move.cmp(&0) {
                Ordering::Greater => {
                    let mut current_location = index;
                    for _ in 1..=amount_to_move {
                        // nowhere near the end - move forward
                        if current_location < number - 2 {
                            let new_location = current_location + 1;
                            input.swap(current_location, new_location);
                            current_location = new_location;
                        } else if current_location == number - 2 {
                            // we're "before the final element" - so moving forward one
                            // will put us "before the first element"; ie at the start
                            let e = input.remove(current_location);
                            input.insert(0, e);
                            current_location = 0;
                        } else if current_location == number - 1 {
                            // we're 'before the start' already (but at the end)
                            // so we now want to move "before the second element",
                            // ie to position 1
                            let e = input.remove(current_location);
                            input.insert(1, e);
                            current_location = 1;
                        }
                    }
                }
                Ordering::Equal => continue,
                Ordering::Less => {
                    // moving backwards - this is so confusing
                    let mut current_location = index;
                    for _ in 1..=(amount_to_move.abs()) {
                        // haven't got near the start yet
                        if current_location > 1 {
                            let new_location = current_location - 1;
                            input.swap(current_location, new_location);
                            current_location = new_location;
                        } else if current_location == 1 {
                            // we're "after the first element", which means we need to move to
                            // "after the last element"
                            let e = input.remove(current_location);
                            // this makes the vector one shorter - so we have to subtract an extra one here
                            input.insert(number - 1, e);
                            current_location = number - 1;
                        } else if current_location == 0 {
                            // we're "after the last element" already (but at the start) -
                            // which means we need to move to after the second-to-last element? I think?
                            let e = input.remove(current_location);
                            input.insert(number - 2, e);
                            current_location = number - 2;
                        }
                    }
                }
            }
        } else {
            println!("Shouldn't happen");
            break;
        }
    }
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[Entry]) -> i64 {
    let mut input = input.to_owned();
    let number = input.len();
    shuffle_vector_by_original_index(&mut input);

    let zero_index = input.iter().position(|e| e.value == 0).unwrap();
    let first = (zero_index + 1000) % number;
    let second = (first + 1000) % number;
    let third = (second + 1000) % number;

    input[first].value + input[second].value + input[third].value
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[Entry]) -> i64 {
    let mut input = input
        .iter()
        .map(|e| Entry {
            value: e.value * 811589153,
            original_index: e.original_index,
        })
        .collect::<Vec<_>>();
    let number = input.len();
    for _ in 1..=10 {
        shuffle_vector_by_original_index(&mut input);
    }

    let zero_index = input.iter().position(|e| e.value == 0).unwrap();
    let first = (zero_index + 1000) % number;
    let second = (first + 1000) % number;
    let third = (second + 1000) % number;

    input[first].value + input[second].value + input[third].value
}

#[test]
fn test_day20_input1() {
    let input = r#"1
2
-3
3
-2
0
4
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 3);
    assert_eq!(part2_result, 1623178306);
}
