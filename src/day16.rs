use std::{collections::{HashSet, HashMap}, thread::current};

pub struct Input {}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
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
    tunnels.get(from).unwrap().distanced_valves.iter().filter(|(v, d)| v == to).next().unwrap().1
}

pub fn find_max_pressure_in_pairs(
    tunnels: &HashMap<ValveId, DistancedValve>,
    already_on_valves: HashSet<ValveId>,
    in_progress_valves: HashSet<ValveId>,
    // (me, elephant)
    starting_from: (&State, &State),
    time_remaining:  usize,
    // flow rate of all open valves so far - use this when travelling
    current_flow_rate: usize,
) -> usize {
    // assume current location is already 'on'
    // (not technically true for the start, but there's no point turning it on)
    if time_remaining == 0 {
        // cheap bail out point, just in case
        return 0;
    }   

    let (my_state, elephant_state) = starting_from;

    match (my_state, elephant_state) {
        (State::TravellingTo(v_m, d_m), State::TravellingTo(v_e, d_e)) => {
            // both still on the way. fast-forward until one of us arrives.
            let time_to_fast_forward = *d_m.min(d_e);
            let pressure_added_while_travelling = current_flow_rate * time_to_fast_forward;
            let time_left = time_remaining - time_to_fast_forward;
            let my_new_state = if *d_m == time_to_fast_forward {
                State::At(v_m.clone())
            } else {
                State::TravellingTo(v_m.clone(), *d_m - time_to_fast_forward)
            };
            let elephant_new_state = if *d_e == time_to_fast_forward {
                State::At(v_e.clone())
            } else {
                State::TravellingTo(v_e.clone(), *d_e - time_to_fast_forward)
            };

            // no choices to be made; we're both travelling
            return pressure_added_while_travelling + 
            find_max_pressure_in_pairs(
                tunnels,
                already_on_valves.clone(),
                in_progress_valves.clone(),
                (&my_new_state, &elephant_new_state),
                time_left,
                current_flow_rate,
            );
        },
        (State::TravellingTo(v_m, d_m), State::At(v_e)) => {
            // I'm still travelling, elephant is at its valve - so turn it on
            let pressure_added_while_turning_valve = current_flow_rate;
            let time_left = time_remaining - 1;
            let elephant_location = tunnels.get(v_e).unwrap();

            let new_flow_rate = current_flow_rate + match elephant_location.flow_rate {
                FlowRate::Flow(x) => x,
                _ => unreachable!(),
            };        
            let mut valves_now_on = already_on_valves.clone();
            valves_now_on.insert(v_e.clone());

            // technically + 1 but this turn accounts for the 1
            let time_until_me_free = d_m;
            let valves_worth_considering = elephant_location.distanced_valves.iter().cloned()
            .filter(|(destination, distance)| {
                !( // valve already on - no point
                    already_on_valves.contains(&destination)
                    // I'm doing this one - no point
                    // (elephant might get there faster - another path will catch that)
                    || in_progress_valves.contains(&destination)
                    // I would get there first, even after what I'm currently doing - no point in the elephant trying it
                    || distance_to(tunnels, v_m, destination) + time_until_me_free < *distance
                    // nowhere possible to get to in time
                    || distance + 1 > time_left)
            }).collect::<HashSet<_>>();
            let mut max_pressure = 0;

            // check what the pressure would be for each move the elephant could make
            for (destination, distance) in valves_worth_considering {
                // I've moved one step further to my destination this step
                let my_new_state = if *d_m == 1 {
                    State::At(v_m.clone())
                } else {
                    State::TravellingTo(v_m.clone(), *d_m - 1)
                };
                // elephant's already done its work this step - it's now travelling
                let elephant_new_state = State::TravellingTo(destination.clone(), distance);
                // remember the elephant is going to do this one - so I don't
                let mut updated_in_progress_valves = in_progress_valves.clone();
                updated_in_progress_valves.insert(destination);

                let max_pressure_added_via_this_route = 
                pressure_added_while_turning_valve + 
                find_max_pressure_in_pairs(
                    tunnels,
                    valves_now_on.clone(),
                    updated_in_progress_valves,
                    (&my_new_state, &elephant_new_state),
                    time_left,
                    new_flow_rate,
                );
        
                max_pressure = max_pressure.max(max_pressure_added_via_this_route);
            }

            // check what the pressure would be if the elephant did nothing more
            // (includes 1 to turn the valve on - we've already taken 'one step' above)
            let remaining_time_for_my_journey = *d_m;
            let pressure_contributed_while_travelling = new_flow_rate * remaining_time_for_my_journey;
            let all_pressure_added_this_step = pressure_added_while_turning_valve + pressure_contributed_while_travelling;

            // turn my valve on too - I'm there now
            let my_location = tunnels.get(v_m).unwrap();

            let updated_flow_rate = new_flow_rate + match my_location.flow_rate {
                FlowRate::Flow(x) => x,
                _ => unreachable!(),
            }; 
            valves_now_on.insert(v_m.clone());
            let pressure_added_only_by_me = all_pressure_added_this_step + find_max_pressure(
                tunnels,
                valves_now_on,
                v_m,
                time_left - remaining_time_for_my_journey,
                updated_flow_rate);
            max_pressure = max_pressure.max(pressure_added_only_by_me);
            
            return max_pressure;
        },
        (me @ State::At(_), elephant @ State::TravellingTo(_, _)) => {
            // swap places with the elephant - logic is the same, who cares what we're called
            return find_max_pressure_in_pairs(
                tunnels,
                already_on_valves,
                in_progress_valves,
                (&elephant, &me),
                time_remaining,
                current_flow_rate);
        }
        (State::At(v_m), State::At(v_e)) => {
            if time_remaining == 0 {
                // make sure we don't spend a free turn here
                return 0;
            }  

            let at_start = v_m.id == "AA";
            // we both reached our valves at the same time
            // cheeky hack here to cope with start position
            let pressure_added_while_turning_valves = if at_start { 0 } else { current_flow_rate };
            let time_left = if at_start { time_remaining } else { time_remaining - 1 };
            let elephant_location = tunnels.get(v_e).unwrap();
            let my_location = tunnels.get(v_m).unwrap();

            let new_flow_rate = if at_start { current_flow_rate } else { current_flow_rate
            + match elephant_location.flow_rate {
                FlowRate::Flow(x) => x,
                _ => unreachable!(),
            }
            + match my_location.flow_rate {
                FlowRate::Flow(x) => x,
                _ => unreachable!(),
            }};  

            // we both turn on our valves
            let mut valves_now_on = already_on_valves.clone();
            valves_now_on.insert(v_e.clone());
            valves_now_on.insert(v_m.clone());

            let my_valves_worth_considering = my_location.distanced_valves.iter().cloned()
            .filter(|(destination, distance)| {
                !( // valve already on - no point
                    valves_now_on.contains(&destination)
                    // this might not be necessary here, but who cares
                    || in_progress_valves.contains(&destination)
                    // no point me picking anywhere the elephant could get to faster
                    || distance_to(tunnels, v_e, destination) < *distance
                    // nowhere possible to get to in time
                    || distance + 1 > time_left)
            }).collect::<HashSet<_>>();

            // two options:
            // either there's a valve for me to pick (and I pick it),
            // or it's better for me to just sit the rest of the time out.

            // or, four options:
            // we both move (nested for loop);
            // only I move (simple case);
            // only elephant moves (simple case);
            // neither of us move (v simple case)
            let mut max_pressure = 0;
            for (m_destination, m_distance) in &my_valves_worth_considering {
                let valves_now_on = valves_now_on.clone();
                let my_new_state = State::TravellingTo(m_destination.clone(), *m_distance);
                // remember I'm going to do this one
                let mut updated_in_progress_valves = in_progress_valves.clone();
                updated_in_progress_valves.insert(m_destination.clone());

                // let the elephant also pick a valve
                let time_until_me_free = m_distance + 1;
                let my_destination = m_destination.clone();
                let elephant_valves_worth_considering = elephant_location.distanced_valves.iter().cloned()
                .filter(|(destination, distance)| {
                    !( // valve already on - no point
                        valves_now_on.contains(&destination)
                        // check it's not the one I'm about to go and do
                        || updated_in_progress_valves.contains(&destination)
                        // I would get there first, even after what I'm currently doing - no point in the elephant trying it
                        || distance_to(tunnels, &my_destination, destination) + time_until_me_free < *distance
                        // nowhere possible to get to in time
                        || distance + 1 > time_left)
                }).collect::<HashSet<_>>();

                for (e_destination, e_distance) in elephant_valves_worth_considering {
                    let elephant_new_state = State::TravellingTo(e_destination.clone(), e_distance);
                    let mut new_in_progress_valves = updated_in_progress_valves.clone();
                    new_in_progress_valves.insert(e_destination.clone());
                    let max_pressure_added_via_this_route = pressure_added_while_turning_valves +
                    find_max_pressure_in_pairs(
                        tunnels,
                        valves_now_on.clone(),
                        new_in_progress_valves,
                        (&my_new_state, &elephant_new_state),
                        time_left,
                        new_flow_rate);
                    
                    max_pressure = max_pressure.max(max_pressure_added_via_this_route);
                }
            }
            // otherwise assume elephant never moves again
            // see what the best I could do is
            let max_pressure_only_from_me = pressure_added_while_turning_valves + find_max_pressure(
                tunnels,
                valves_now_on.clone(),
                v_m,
                time_left,
                new_flow_rate);
            max_pressure = max_pressure.max(max_pressure_only_from_me);
            // same again, but for the elephant
            let max_pressure_only_from_elephant = pressure_added_while_turning_valves + find_max_pressure(
                tunnels,
                valves_now_on.clone(),
                v_e,
                time_left,
                new_flow_rate);
            max_pressure = max_pressure.max(max_pressure_only_from_elephant);
            // now if neither of us move
            let pressure_if_we_do_nothing = pressure_added_while_turning_valves + new_flow_rate * time_left;
            max_pressure = max_pressure.max(pressure_if_we_do_nothing);
            return max_pressure;
        }
    }
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

#[aoc(day16, part2)]
pub fn solve_part2(input: &HashMap<ValveId, DistancedValve>) -> usize {
    let start = ValveId { id: "AA".to_owned() };
    find_max_pressure_in_pairs(
        input,
        HashSet::from([start.clone()]),
        HashSet::new(),
        (&State::At(start.clone()), &State::At(start.clone())),
        26,
        0)
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
    println!("{:?}", parsed_input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);
    println!("{}", part1_result);
    println!("{}", part2_result);


    // assert_eq!(part1_result, 0);
    // assert_eq!(part2_result, 0);
}