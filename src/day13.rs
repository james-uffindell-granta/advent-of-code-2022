pub struct Input;


#[aoc_generator(day13)]
pub fn input_generator_part1(input: &str) -> Input {
    Input{}
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> usize {
    0
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> usize {
    0
}

#[test]
fn test_day13_input1() {
    let input =
r#"puzzle
input
here
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 0);
    assert_eq!(part2_result, 0);
}