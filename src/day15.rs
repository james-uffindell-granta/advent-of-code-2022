pub struct Input {}

#[aoc_generator(day15)]
pub fn input_generator_part1(_input: &str) -> Input {
    Input {}
}


#[aoc(day15, part1)]
pub fn solve_part1(_input: &Input) -> usize {
    0
}

#[aoc(day15, part2)]
pub fn solve_part2(_input: &Input) -> usize {
    0
}

#[test]
fn test_day15_input1() {
    let input =
r#"puzzle_input_here
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 0);
    assert_eq!(part2_result, 0);
}