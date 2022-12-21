use std::collections::{HashSet, HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Operation {
    Add, Sub, Mul, Div,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct MonkeyId(String);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Yell {
    Specific(i64),
    Result(MonkeyId, Operation, MonkeyId)
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Monkey {
    id: MonkeyId,
    yell: Yell,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MonkeyGraph {
    vertices: HashSet<MonkeyId>,
    // (m1, m2) means m1 is required for m2
    edges: HashSet<(MonkeyId, MonkeyId)>,
}

impl MonkeyGraph {
    pub fn remove_edge(&mut self, (m1, m2): (MonkeyId, MonkeyId)) {
        self.edges.remove(&(m1, m2));
    }
}

// topological sort of the graph
pub fn sort_graph(graph: &MonkeyGraph) -> Vec<MonkeyId> {
    let mut graph: MonkeyGraph = graph.clone();
    let dependents = graph.edges.iter().cloned().map(|(_, m)| m).collect::<HashSet<_>>();
    // first find everything that doesn't depend on anything else - these can go first
    let mut remaining_leaves = graph.vertices.difference(&dependents).cloned().collect::<HashSet<_>>();
    let mut sorted_nodes = vec![];
    loop {
        // nothing left to process
        if remaining_leaves.is_empty() {
            break;
        }

        // anything in remaining_leaves can go next, because nothing there depends on anything unprocessed
        let n = remaining_leaves.iter().next().unwrap().clone();
        remaining_leaves.remove(&n);
        sorted_nodes.push(n.clone());
        
        // find what things depended on the node we just took
        let edges_from_here = graph.edges
            .iter().cloned().filter(|(m, _)| m == &n).collect::<HashSet<_>>();

        for (m1, m2) in edges_from_here {
            // for each of those: remove the edge from the graph
            // this means the graph only contains dependencies involving things we haven't processed yet
            graph.remove_edge((m1.clone(), m2.clone()));
            if !graph.edges.iter().cloned().any(|(_, m)| m == m2.clone()) {
                // if we just took the last dependency of some m2 out of the graph, then m2 is a leaf
                // and can go in the sorted list any time from now
                remaining_leaves.insert(m2);
            }
        }
    }

    sorted_nodes
}

pub struct Input {}

#[aoc_generator(day21)]
pub fn input_generator_part1(input: &str) -> Vec<Monkey> {
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();
    let mut lookup = HashMap::new();
    for line in input.lines() {
        let (monkey, operation) = line.split_once(": ").unwrap();
        let monkey_id = MonkeyId(monkey.to_owned());
        if let Ok(num) = operation.parse() {
            let monkey = Monkey {
                id: monkey_id.clone(),
                yell: Yell::Specific(num),
            };
            vertices.insert(monkey_id.clone());
            lookup.insert(monkey_id, monkey);
        } else {
            let mut components = operation.split(" ");
            let left_id = components.next().unwrap();
            let left_monkey = MonkeyId(left_id.to_owned());
            let operation = match components.next().unwrap() {
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Mul,
                "/" => Operation::Div,
                _ => unreachable!(),
            };
            let right_id = components.next().unwrap();
            let right_monkey = MonkeyId(right_id.to_owned());

            vertices.insert(monkey_id.clone());
            edges.insert((left_monkey.clone(), monkey_id.clone()));
            edges.insert((right_monkey.clone(), monkey_id.clone()));
            let monkey = Monkey {
                id: monkey_id.clone(),
                yell: Yell::Result(left_monkey.clone(), operation, right_monkey.clone()),
            };
            lookup.insert(monkey_id, monkey);
        }
    }

    let graph = MonkeyGraph { vertices, edges };
    let sorted_ids = sort_graph(&graph);
    sorted_ids.into_iter().map(|id| lookup.get(&id).unwrap().clone()).collect()
}


#[aoc(day21, part1)]
pub fn solve_part1(input: &Vec<Monkey>) -> i64 {
    let mut results = HashMap::new();
    // input is already in order, so just go through and build the results
    for m in input {
        match &m.yell {
            Yell::Specific(num) => { results.insert(m.id.clone(), *num); },
            Yell::Result(m1, op, m2) => {
                let m1_result = *results.get(m1).unwrap();
                let m2_result = *results.get(m2).unwrap();
                match op {
                    Operation::Add => { results.insert(m.id.clone(), m1_result + m2_result); },
                    Operation::Sub => { results.insert(m.id.clone(), m1_result - m2_result); },
                    Operation::Mul => { results.insert(m.id.clone(), m1_result * m2_result); },
                    Operation::Div => { results.insert(m.id.clone(), m1_result / m2_result); },
                }
            }
        }
    }

    *results.get(&MonkeyId("root".to_owned())).unwrap()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Vec<Monkey>) -> i64 {
    let mut partial_results = HashMap::new();
    let human = MonkeyId("humn".to_owned());
    let root = MonkeyId("root".to_owned());
    // a bit trickier here - use Option to track calculations we don't know the answer to yet
    for m in input {
        if m.id.clone() == human {
            // human doesn't know what to say
            partial_results.insert(m.id.clone(), None);
        } else {
            match &m.yell {
                Yell::Specific(num) => { partial_results.insert(m.id.clone(), Some(*num)); },
                Yell::Result(m1, op, m2) => {
                    let m1_result = *partial_results.get(m1).unwrap();
                    let m2_result = *partial_results.get(m2).unwrap();
                    match op {
                        // map the operation into the Options
                        Operation::Add => { partial_results.insert(m.id.clone(), m1_result.and_then(|x| m2_result.map(|y| x + y))); },
                        Operation::Sub => { partial_results.insert(m.id.clone(), m1_result.and_then(|x| m2_result.map(|y| x - y))); },
                        Operation::Mul => { partial_results.insert(m.id.clone(), m1_result.and_then(|x| m2_result.map(|y| x * y))); },
                        Operation::Div => { partial_results.insert(m.id.clone(), m1_result.and_then(|x| m2_result.map(|y| x / y))); },
                    }
                }
            }
        }
    }

    // at this point the results are complete for anything that doesn't require my input
    // but None for anything that does
    // now we go back in the opposite direction and figure out what all the answers should have been
    let mut monkeys = input.clone();
    monkeys.reverse();

    for m in monkeys {
        if m.id.clone() == root {
            let Yell::Result(m1, _, m2) = m.yell else { unreachable!() };
            let m1_result = *partial_results.get(&m1).unwrap();
            let m2_result = *partial_results.get(&m2).unwrap();
            match (m1_result, m2_result) {
                // the other number should be the same, so tell that monkey what its answer was supposed to be
                (Some(num), None) => { partial_results.insert(m2, Some(num)); },
                (None, Some(num)) => { partial_results.insert(m1, Some(num)); },
                _ => unreachable!(),
            }
        } else {
            // we must have reverse-calculated everything this far at least
            let Some(m_result) = *partial_results.get(&m.id).unwrap() else { 
                unreachable!()
            };
            match &m.yell {
                Yell::Specific(_) => { 
                    // nothing to do - monkey knows its number
                },
                Yell::Result(m1, op, m2) => {
                    let m1_result = *partial_results.get(m1).unwrap();
                    let m2_result = *partial_results.get(m2).unwrap();
                    match (m1_result, m2_result) {
                        (Some(_), Some(_)) => {
                            // both this monkey's dependents already know their answers - no need to do anything
                        },
                        (Some(num1), None) => {
                            // first monkey knows, but we don't know what the second monkey should have said
                            // figure it out from the answer and the first monkey, and then remember what
                            // it was supposed to calculate
                            match op {
                                Operation::Add => { partial_results.insert(m2.clone(), Some(m_result - num1)); },
                                Operation::Sub => { partial_results.insert(m2.clone(), Some(num1 - m_result)); },
                                Operation::Mul => { partial_results.insert(m2.clone(), Some(m_result / num1)); },
                                Operation::Div => { partial_results.insert(m2.clone(), Some(num1 / m_result)); },
                            }
                        },
                        (None, Some(num2)) => {
                            // same as above, but monkeys the other way round
                            match op {
                                Operation::Add => { partial_results.insert(m1.clone(), Some(m_result - num2)); },
                                Operation::Sub => { partial_results.insert(m1.clone(), Some(num2 + m_result)); },
                                Operation::Mul => { partial_results.insert(m1.clone(), Some(m_result / num2)); },
                                Operation::Div => { partial_results.insert(m1.clone(), Some(num2 * m_result)); },
                            }
                        },
                        // can't happen (human is only in one branch)
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    // everything filled in now - what should human have said?
    partial_results.get(&human).unwrap().unwrap()
}

#[test]
fn test_day21_input1() {
    let input =
r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 152);
    assert_eq!(part2_result, 301);
}