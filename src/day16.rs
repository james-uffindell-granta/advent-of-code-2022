use std::{collections::{HashSet, HashMap}};

pub struct Input {}

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct ValveId {
    id: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FlowRate {
    Starting,
    Broken,
    Flow(usize),
}

pub struct Valve {
    flow_rate: FlowRate,
    subsequent_valves: HashSet<ValveId>,
}

#[derive(Debug)]
pub struct DistancedValve {
    flow_rate: FlowRate,
    distanced_valves: HashSet<(ValveId, usize)>,
}


// use dijkstra's algorithm
// calculate "shortest distance from start to here" for all points in the grid
pub fn calculate_distances(tunnels: &HashMap<ValveId, Valve>, start: &ValveId) -> HashMap<ValveId, Option<usize>> {
    // we haven't handled these points yet
    let mut unvisited_nodes = tunnels.keys().cloned().collect::<HashSet<_>>();
    // distances to point from start (value=Some(x) means we found a route of length x; value=None means we didn't find a route yet)
    let mut distance_map = unvisited_nodes.iter().cloned()
        .map(|n| (n, None)).collect::<HashMap<_, _>>();

    // start point is 0 away from start
    *distance_map.get_mut(start).unwrap() = Some(0usize);

    // find a point in the grid where:
    // - we've found a route from the start to it already
    // - it's the shortest distance away from the start of all the places we've found routes to
    // (for now, there's only one of these - the start point itself, with distance 0)
    let mut smallest_node = unvisited_nodes.iter().cloned()
    .filter(|c| matches!(distance_map.get(c).unwrap(), Some(_)))
    .min_by_key(|c| {
        match distance_map.get(c).unwrap() {
            Some(x) => x,
            _ => unreachable!(),
        }
    });

    // this will keep going until either we handle every node, or until every remaining node has a value of None
    // (which means all the remaining nodes are unreachable from the start)
    while let Some(current_node) = smallest_node {
        // get the neighbours of the point we're looking at...
        let neighbours = tunnels.get(&current_node).unwrap().subsequent_valves.iter().cloned()
            // ... which we haven't already finished working with
            .filter(|c| unvisited_nodes.contains(&c))
            .collect::<Vec<_>>();

        // the current node is this far away from the start
        let current_distance = distance_map.get(&current_node).unwrap().unwrap();
        for n in neighbours {
            // each neighbour is one away from here - see if that's a closer route to what we've found to that neighbour already
            let distance = current_distance + 1;
            let new_value = match distance_map.get(&n).unwrap() {
                Some(existing_value) => {
                    if *existing_value > distance {
                        // found a shorter route
                        distance
                    } else {
                        // this route is longer - leave the value alone
                        *existing_value
                    }
                },
                // this is the first route there we've found
                None => distance
            };
            // remember the shortest route to that neighbour
            distance_map.insert(n, Some(new_value));
        }

        // we've handled all neighbours of this node - no need to ever go back to it again
        unvisited_nodes.remove(&current_node);
        // find a new node that we haven't handled yet which is closest to the start, and repeat
        smallest_node = unvisited_nodes.iter().cloned()
        .filter(|c| matches!(distance_map.get(c).unwrap(), Some(_)))
        .min_by_key(|c| {
            match distance_map.get(c).unwrap() {
                Some(x) => x,
                _ => unreachable!(),
            }
        });
    }

    distance_map
}

#[aoc_generator(day16)]
pub fn input_generator_part1(input: &str) -> HashMap<ValveId, DistancedValve> {
    let mut all_valves = HashMap::new();

    for line in input.lines() {
        let (valve, tunnels) = line.split_once("; ").unwrap();
        let valve = valve.strip_prefix("Valve ").unwrap();
        let (valve_id, flow) = valve.split_once(" ").unwrap();
        let flow = flow.strip_prefix("has flow rate=").unwrap().parse().unwrap();
        let flow_rate = match flow {
            0 if valve_id == "AA" => FlowRate::Starting,
            0 => FlowRate::Broken,
            i => FlowRate::Flow(i),
        };
        let subsequent_valves = if let Some(vs) = tunnels.strip_prefix("tunnels lead to valves ") {
            vs.split(", ").map(|s| ValveId { id: s.to_owned() }).collect()
        } else if let Some(v) = tunnels.strip_prefix("tunnel leads to valve ") {
            HashSet::from([ValveId { id: v.to_owned() }])
        } else {
            unreachable!()
        };
        // let subsequent_valves = tunnels.strip_prefix("tunnels lead to valves ").unwrap()
        //     .split(", ").map(|s| ValveId { id: s.to_owned() }).collect();
        all_valves.insert(ValveId { id: valve_id.to_owned()}, Valve { flow_rate, subsequent_valves });
    }

    let mut all_distanced_valves = HashMap::new();

    // there's no point remembering the valves with flow rate 0 - it will take too long to
    // explore all the paths.
    for (valve_id, valve) in all_valves.iter() {
        // only check ones that are worth remembering
        if matches!(valve.flow_rate, FlowRate::Broken) {
            continue;
        }
        let distances = calculate_distances(&all_valves, valve_id);
        let mut distanced_valves = HashSet::new();
        for (target_valve, distance) in distances.into_iter() {
            match distance {
                Some(d) => { 
                    let target_flow_rate = all_valves.get(&target_valve).unwrap().flow_rate;
                    if d > 0 && !matches!(target_flow_rate, FlowRate::Broken | FlowRate::Starting) {
                        distanced_valves.insert((target_valve, d));
                    }
                },
                _ => () // no way to get there from here (or we're already there)
            }
        }
        all_distanced_valves.insert(valve_id.clone(), DistancedValve { flow_rate: valve.flow_rate, distanced_valves  });
    }

    all_distanced_valves
}

pub fn find_max_pressure(
    tunnels: &HashMap<ValveId, DistancedValve>,
    already_on_valves: HashSet<ValveId>,
    starting_from: &ValveId,
    time_remaining:  usize,
    // flow rate of all open valves so far - use this when travelling
    current_flow_rate: usize,
) -> usize {
    // assume current location is already 'on'
    // (not technically true for the start, but there's no point turning it on)
    let mut max_pressure = 0;
    let current_location = tunnels.get(&starting_from).unwrap();

    let valves_worth_considering = current_location.distanced_valves.iter().cloned()
    .filter(|(destination, distance)| {
        !(already_on_valves.contains(&destination) || distance + 1 > time_remaining)
    }).collect::<HashSet<_>>();

    if valves_worth_considering.is_empty() {
        return current_flow_rate * time_remaining;
    }

    for (destination, distance) in valves_worth_considering {
        // otherwise: go there, turn on the valve, and see how that does
        // we're only going there to turn on the valve, so include the minute that takes
        let pressure_added_while_travelling = current_flow_rate * (distance + 1);
        let time_left = time_remaining - distance - 1;
        let mut valves_now_on = already_on_valves.clone();
        valves_now_on.insert(destination.clone());
        let destination_flow_rate = tunnels.get(&destination).unwrap().flow_rate;
        let new_flow_rate = current_flow_rate + match destination_flow_rate {
            FlowRate::Flow(x) => x,
            _ => unreachable!(),
        };
        let max_pressure_added_via_this_route = 
        pressure_added_while_travelling + 
        find_max_pressure(
            tunnels,
            valves_now_on,
            &destination,
            time_left,
            new_flow_rate,
        );

        max_pressure = max_pressure.max(max_pressure_added_via_this_route);
    }
  
    max_pressure
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    At(ValveId),
    // number of steps left to get there
    TravellingTo(ValveId, usize),
    // Done(usize),
}

pub fn distance_to(tunnels: &HashMap<ValveId, DistancedValve>, from: &ValveId, to: &ValveId) -> usize {
    tunnels.get(from).unwrap().distanced_valves.iter().filter(|(v, _)| v == to).next().unwrap().1
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &HashMap<ValveId, DistancedValve>) -> usize {
    let start = ValveId { id: "AA".to_owned() };
    find_max_pressure(
        input,
        HashSet::from([start.clone()]),
        &start,
        30,
        0,
    )
}

pub fn number_to_subset(valves_by_id: &Vec<ValveId>, number: usize) -> HashSet<ValveId> {
    let mut valves = HashSet::new();
    for (index, valve_id) in valves_by_id.iter().enumerate() {
        let bitmask = 1 << index;
        if number & bitmask != 0 {
            valves.insert(valve_id.clone());
        }
    }
    valves
}

pub fn number_to_complementary_number(total_valves: usize, number: usize) -> usize {
    let mask = (1 << total_valves) - 1;
    let inverse = !number;
    inverse & mask
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &HashMap<ValveId, DistancedValve>) -> usize {
    let start = ValveId { id: "AA".to_owned() };

    let mut valves_to_turn = input.keys().cloned().filter(|v| v.id != "AA").collect::<Vec<_>>();
    valves_to_turn.sort();
    let valves_to_turn = valves_to_turn;
    let mut partitions_checked = HashSet::new();
    let mut max_pressure = 0;
    for i in 0..(2_usize.pow((valves_to_turn.len() - 1) as u32)) {
        let mut valves_for_me = number_to_subset(&valves_to_turn, i);
        let complement = number_to_complementary_number(valves_to_turn.len(), i);
        let mut valves_for_elephant = number_to_subset(&valves_to_turn, complement);

        let my_valves = valves_for_me.clone();
        assert_eq!(my_valves.union(&valves_for_elephant).collect::<HashSet<_>>(), valves_to_turn.iter().collect::<HashSet<_>>());

        valves_for_me.insert(start.clone());
        valves_for_elephant.insert(start.clone());

        let my_max_pressure_this_partition = find_max_pressure(input, valves_for_elephant.clone(), &start, 26, 0);
        let elephant_max_pressure_this_partition = find_max_pressure(input, valves_for_me.clone(), &start, 26, 0);
        max_pressure = max_pressure.max(my_max_pressure_this_partition + elephant_max_pressure_this_partition);

        partitions_checked.insert(i);
        partitions_checked.insert(complement);
    }

    max_pressure
}

#[test]
fn test_bitmasking() {
    let number = 0b101;
    let number_of_valves = 5;
    assert_eq!(0b11010, number_to_complementary_number(number_of_valves, number));
}

#[test]
fn test_day16_input1() {
    let input =
r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 1651);
    assert_eq!(part2_result, 1707);
}