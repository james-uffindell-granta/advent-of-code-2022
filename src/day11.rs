use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Operand {
    OldValue,
    SpecificValue(u128)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Monkey {
    starting_items: Vec<u128>,
    operation: char,
    operand: Operand,
    divisibility_check: u128,
    true_monkey_id: usize,
    false_monkey_id: usize,
}

#[aoc_generator(day11)]
pub fn input_generator_part1(input: &str) -> HashMap<usize, Monkey> {
    let monkeys = input.split("\n\n");

    let mut monkey_dict = HashMap::new();

    for monkey in monkeys {
        let mut monkey_lines = monkey.lines();
        let monkey_id = monkey_lines.next().unwrap()
            .strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap()
            .parse::<usize>().unwrap();
        let monkey_items = monkey_lines.next().unwrap().trim()
            .strip_prefix("Starting items: ").unwrap().split(", ")
            .map(|s| s.parse::<_>().unwrap()).collect::<Vec<_>>();
        let (operation, value) = monkey_lines.next().unwrap().trim()
            .strip_prefix("Operation: new = old ").unwrap().split_once(" ").unwrap();

        let operand = match value {
            "old" => Operand::OldValue,
            v => Operand::SpecificValue(v.parse().unwrap()),
        };

        let divisibility_test = monkey_lines.next().unwrap().trim()
            .strip_prefix("Test: divisible by ").unwrap().parse::<_>().unwrap();

        let true_monkey = monkey_lines.next().unwrap().trim()
            .strip_prefix("If true: throw to monkey ").unwrap().parse::<usize>().unwrap();
        let false_monkey = monkey_lines.next().unwrap().trim()
            .strip_prefix("If false: throw to monkey ").unwrap().parse::<usize>().unwrap();

        monkey_dict.insert(monkey_id, Monkey {
            starting_items: monkey_items,
            operation: operation.chars().next().unwrap(),
            operand: operand,
            divisibility_check: divisibility_test,
            true_monkey_id: true_monkey,
            false_monkey_id: false_monkey,
        });
    }

    monkey_dict
}

pub fn run_monkey_loop<F>(monkeys: &HashMap<usize, Monkey>, iteration_count: usize, post_inspection_operation: F)
    -> HashMap<usize, usize>
    where F : Fn(u128) -> u128 {
    let monkey_count = monkeys.keys().len();
    let mut monkeys: HashMap<_, _> = monkeys.clone();

    let mut monkey_business = HashMap::new();

    for _round_count in 1..=iteration_count {
        // each monkey takes turns throwing
        for monkey_id in 0..monkey_count {
            let monkey = monkeys.get_mut(&monkey_id).unwrap();
            let mut thrown_items_dict = HashMap::new();
            for item in monkey.starting_items.iter() {
                let inspected_item = match (monkey.operation, monkey.operand) {
                    ('*', Operand::OldValue) => *item * *item,
                    ('*', Operand::SpecificValue(v)) => *item * v,
                    ('+', Operand::SpecificValue(v)) => *item + v,
                    _ => unreachable!(),
                };
                // we inspected an item - remember this
                *monkey_business.entry(monkey_id).or_insert(0usize) += 1;
                let bored_item = post_inspection_operation(inspected_item);
                let passed_check = bored_item % monkey.divisibility_check == 0;
                let monkey_id_to_throw_to = if passed_check {
                    monkey.true_monkey_id
                } else {
                    monkey.false_monkey_id
                };

                let other_monkey = thrown_items_dict.entry(monkey_id_to_throw_to).or_insert(Vec::new());
                other_monkey.push(bored_item);
            }

            // this monkey has thrown all its stuff now
            monkey.starting_items.clear();

            for (monkey, mut items) in thrown_items_dict.into_iter() {
                let monkey = monkeys.get_mut(&monkey).unwrap();
                monkey.starting_items.append(&mut items);
            }
        }
    }

    monkey_business
}


#[aoc(day11, part1)]
pub fn solve_part1(input: &HashMap<usize, Monkey>) -> usize {
    // post inspection: stop worrying, divide by 3 and lose remainder
    let monkey_business = run_monkey_loop(input, 20, |i| i / 3);
    let mut monkey_inspections = monkey_business.iter().collect::<Vec<_>>();
    monkey_inspections
        .sort_by(|(_, i1), (_, i2)| i2.cmp(i1));
    monkey_inspections[0].1 * monkey_inspections[1].1
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &HashMap<usize, Monkey>) -> usize {
    // remember the product of all the checks
    let monkey_factor = input.values().map(|m| m.divisibility_check).product::<u128>();
    // post inspection: stop number getting too big by taking it mod the product above
    let monkey_business = run_monkey_loop(input, 10_000, |i| i % monkey_factor);
    let mut monkey_inspections = monkey_business.iter().collect::<Vec<_>>();
    monkey_inspections
        .sort_by(|(_, i1), (_, i2)| i2.cmp(i1));
    monkey_inspections[0].1 * monkey_inspections[1].1
}

#[test]
fn test_day11_input1() {
    let input =
r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 10605);
    assert_eq!(part2_result, 2713310158);
}
