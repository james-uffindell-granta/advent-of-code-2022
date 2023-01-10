use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::delimited, Finish, IResult,
};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    List(Vec<Value>),
    Integer(i32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(is), Value::Integer(io)) => is.partial_cmp(io),
            (Value::List(ls), Value::List(lo)) => ls.partial_cmp(lo),
            (left @ Value::List(_), Value::Integer(io)) => {
                left.partial_cmp(&Value::List(vec![Value::Integer(*io)]))
            }
            (Value::Integer(is), right @ Value::List(_)) => {
                Value::List(vec![Value::Integer(*is)]).partial_cmp(right)
            }
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let list = delimited(tag("["), separated_list0(tag(","), parse_value), tag("]"));
    let parse_int = map(digit1, |x: &str| x.parse::<i32>().unwrap());
    alt((map(parse_int, Value::Integer), map(list, Value::List)))(input)
}

#[aoc_generator(day13, part1)]
pub fn input_generator_part1(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|p| {
            let (first, second) = p.split_once('\n').unwrap();
            (
                parse_value(first).finish().unwrap().1,
                parse_value(second).finish().unwrap().1,
            )
        })
        .collect()
}

#[aoc_generator(day13, part2)]
pub fn input_generator_part2(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_value(l).finish().unwrap().1)
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[(Value, Value)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (first, second))| first.cmp(second) == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Value]) -> usize {
    let divider_1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let divider_2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
    let mut input = input.to_owned();
    // include those packets and sort
    input.push(divider_1.clone());
    input.push(divider_2.clone());
    input.sort();
    let index_1 = input
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, d)| *d == divider_1)
        .unwrap()
        .0;
    let index_2 = input
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, d)| *d == divider_2)
        .unwrap()
        .0;
    (index_1 + 1) * (index_2 + 1)
}

#[test]
fn test_day13_input1() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let parsed_input_2 = input_generator_part2(input);
    let part2_result = solve_part2(&parsed_input_2);

    assert_eq!(part1_result, 13);
    assert_eq!(part2_result, 140);
}
