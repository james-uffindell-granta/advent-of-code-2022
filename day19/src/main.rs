use std::{
    collections::{HashMap, HashSet},
    iter::Sum,
    ops::Add,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    pub fn robots() -> Vec<Self> {
        vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode]
    }

    pub fn retrieved_stock(&self) -> Stock {
        match self {
            Robot::Ore => (1, 0, 0, 0).into(),
            Robot::Clay => (0, 1, 0, 0).into(),
            Robot::Obsidian => (0, 0, 1, 0).into(),
            Robot::Geode => (0, 0, 0, 1).into(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Stock {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Stock {
    pub fn empty() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn try_afford(&self, other: &Stock) -> Option<Stock> {
        let new_ore = self.ore - other.ore;
        let new_clay = self.clay - other.clay;
        let new_obs = self.obsidian - other.obsidian;
        let new_geode = self.geode - other.geode;
        if new_ore >= 0 && new_clay >= 0 && new_obs >= 0 && new_geode >= 0 {
            Some((new_ore, new_clay, new_obs, new_geode).into())
        } else {
            None
        }
    }
}

impl From<(i64, i64, i64, i64)> for Stock {
    fn from((ore, clay, obsidian, geode): (i64, i64, i64, i64)) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl Add<Stock> for Stock {
    type Output = Stock;

    fn add(self, other: Stock) -> Self::Output {
        Self::Output {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sum for Stock {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Stock::empty();
        for s in iter {
            sum = sum + s;
        }
        sum
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    current_stock: Stock,
    // did have a vec<robot> before, but this is like three times faster
    current_robots: (u8, u8, u8, u8),
    time_left: usize,
}

impl Factory {
    pub fn new(time: usize) -> Self {
        Self {
            current_stock: Stock::empty(),
            current_robots: (1, 0, 0, 0),
            time_left: time,
        }
    }

    pub fn resources_gathered(&self) -> Stock {
        let (ore, clay, obs, geode) = self.current_robots;
        (ore as i64, clay as i64, obs as i64, geode as i64).into()
    }

    pub fn worth_building(&self, prices: &HashMap<Robot, Stock>, robot: &Robot) -> bool {
        let geode_robot_cost = prices.get(&Robot::Geode).unwrap();

        let number_obsidian_robots = self.current_robots.2;
        let obsidian_robot_cost = prices.get(&Robot::Obsidian).unwrap();

        let number_clay_robots = self.current_robots.1;
        let clay_robot_cost = prices.get(&Robot::Clay).unwrap();
        let ore_robot_cost = prices.get(&Robot::Ore).unwrap();

        let number_ore_robots = self.current_robots.0;

        let ore_robots_worth_building = geode_robot_cost
            .ore
            .max(obsidian_robot_cost.ore)
            .max(clay_robot_cost.ore)
            .max(ore_robot_cost.ore);

        match robot {
            // always worth building geode robots
            Robot::Geode => true,
            // otherwise, it's only (potentially) worth building a robot if we have
            // fewer than the amount of materials needed to build another robot
            // we can only build one robot a turn, so (for example) if an obsidian
            // robot requires 5 clay, there's no point in ever having more than 5 clay
            // robots, since they'll be collecting clay at a faster rate than we could
            // ever spend - we may as well not have bothered making the excess clay robots
            Robot::Obsidian => (number_obsidian_robots as i64) < geode_robot_cost.obsidian,
            Robot::Clay => (number_clay_robots as i64) < obsidian_robot_cost.clay,
            Robot::Ore => (number_ore_robots as i64) < ore_robots_worth_building,
        }
    }

    pub fn next_steps(&self, prices: &HashMap<Robot, Stock>) -> Vec<Factory> {
        if self.time_left == 0 {
            return vec![];
        }

        let new_resources_gathered = self.resources_gathered();

        let robots_worth_building_by_time = match self.time_left {
            // one turn left: not worth building anything
            1 => HashSet::new(),
            // worth building a geode robot (if we can);
            // anything else can't be converted to a geode robot quickly enough
            2 | 3 => HashSet::from([Robot::Geode]),
            // not worth building clay robots - they take 2 turns to have got
            // us clay, and that's only useful for obsidian robots,
            // which we aren't going to build more of
            4 | 5 => HashSet::from([Robot::Geode, Robot::Obsidian, Robot::Ore]),

            _ => HashSet::from([Robot::Geode, Robot::Obsidian, Robot::Clay, Robot::Ore]),
        };

        let robot_options = prices
            .iter()
            .filter_map(|(r, p)| {
                // robot has to be worth building
                if !robots_worth_building_by_time.contains(r) || !self.worth_building(prices, r) {
                    return None;
                }
                let (old_ore, old_clay, old_obs, old_geode) = self.current_robots;
                let new_robots: (u8, u8, u8, u8) = match r {
                    Robot::Ore => (old_ore + 1, old_clay, old_obs, old_geode),
                    Robot::Clay => (old_ore, old_clay + 1, old_obs, old_geode),
                    Robot::Obsidian => (old_ore, old_clay, old_obs + 1, old_geode),
                    Robot::Geode => (old_ore, old_clay, old_obs, old_geode + 1),
                };

                // we have to be able to afford the robot with our current stocks
                self.current_stock.try_afford(p).map(|leftover| Factory {
                    current_stock: leftover + new_resources_gathered,
                    current_robots: new_robots,
                    time_left: self.time_left - 1,
                })
            })
            .collect::<Vec<_>>();

        // if we could build all four robots, then there's no point doing nothing
        if robot_options.len() == 4 {
            return robot_options;
        }

        // otherwise always possible to do nothing
        // one possible optimization here we aren't doing:
        // if we do nothing, but we could have built robots this turn, then
        // we shouldn't built those robots next turn (there's no point building
        // a robot later than you have to, if you're going to build it anyway)
        robot_options
            .into_iter()
            .chain(std::iter::once(Factory {
                current_stock: self.current_stock + new_resources_gathered,
                current_robots: self.current_robots,
                time_left: self.time_left - 1,
            }))
            .collect()
    }
}

// assuming we don't have any robots to start with - how long to collect 'number' things?
pub fn min_turns_to_collect(number: i64) -> i64 {
    let mut sum = 0;
    for i in 0.. {
        sum += i;
        if sum >= number {
            // one more because we have to build the first robot too
            return i + 1;
        }
    }

    unreachable!();
}

pub fn get_most_geodes(
    start: &Factory,
    prices: &HashMap<Robot, Stock>,
    seen_states: &mut HashMap<Factory, i64>,
) -> i64 {
    let number_geode_robots = start.current_robots.3 as i64;
    let geode_robot_cost = prices.get(&Robot::Geode).unwrap();

    let number_obsidian_robots = start.current_robots.2 as i64;
    let obsidian_robot_cost = prices.get(&Robot::Obsidian).unwrap();

    let number_clay_robots = start.current_robots.1;

    // if we already know the best answer for this factory, then return it
    if let Some(geodes) = seen_states.get(start) {
        // already figured out the best for this combination
        return *geodes;
    }

    // otherwise if we haven't figured out this factory before, check if it
    // is out of time - in which case the geodes it has are the best it can do
    // (and remember that)
    if start.time_left == 0 {
        let geodes_gathered = start.current_stock.geode;
        seen_states.insert(start.clone(), geodes_gathered);
        return geodes_gathered;
    }

    // the last few states are very amenable to manual analysis
    if start.time_left == 1 {
        // no point building robots
        let geodes_gathered = start.current_stock.geode + number_geode_robots;
        seen_states.insert(start.clone(), geodes_gathered);
        return geodes_gathered;
    }

    if start.time_left == 2 {
        // no point building robots that aren't geode robots
        let geodes_gathered = if start.current_stock.try_afford(geode_robot_cost).is_some() {
            // assume we build this robot and send it off - it will get us one more geode on the last turn
            start.current_stock.geode + (number_geode_robots * 2) + 1
        } else {
            start.current_stock.geode + (number_geode_robots * 2)
        };
        seen_states.insert(start.clone(), geodes_gathered);
        return geodes_gathered;
    }

    if start.time_left == 3 {
        // building a non-geode robot here is pointless - it can't gather materials here quickly enough
        // for them to be of use.
        // if we can build a geode robot, we should - it will get us 2 geodes after it's finished.
        if let Some(remainder) = start.current_stock.try_afford(geode_robot_cost) {
            let stock_when_two_minutes_left = remainder + start.resources_gathered();
            if stock_when_two_minutes_left
                .try_afford(geode_robot_cost)
                .is_some()
            {
                // we can build another next turn too - so do so
                let geodes_gathered = 3 + start.current_stock.geode + (number_geode_robots * 3);
                seen_states.insert(start.clone(), geodes_gathered);
                return geodes_gathered;
            } else {
                // can't build one next turn; ah well
                let geodes_gathered = 2 + start.current_stock.geode + (number_geode_robots * 3);
                seen_states.insert(start.clone(), geodes_gathered);
                return geodes_gathered;
            }
        } else {
            let stock_when_two_minutes_left = start.current_stock + start.resources_gathered();
            if stock_when_two_minutes_left
                .try_afford(geode_robot_cost)
                .is_some()
            {
                // we can build one next turn - so do so
                let geodes_gathered = 1 + start.current_stock.geode + (number_geode_robots * 3);
                seen_states.insert(start.clone(), geodes_gathered);
                return geodes_gathered;
            } else {
                // can't build one next turn either; ah well
                let geodes_gathered = start.current_stock.geode + (number_geode_robots * 3);
                seen_states.insert(start.clone(), geodes_gathered);
                return geodes_gathered;
            }
        }
    }

    // another bunch of branches to prune:
    // haven't started making obsidian yet - not enough time
    if number_geode_robots == 0
        && number_obsidian_robots == 0
        && min_turns_to_collect(geode_robot_cost.obsidian) > (start.time_left - 1) as i64
    {
        // collecting all the obsidian would only leave us one turn - no way to get geodes
        seen_states.insert(start.clone(), 0);
        return 0;
    }

    // haven't started making clay yet - not enough time
    if number_geode_robots == 0
        && number_obsidian_robots == 0
        && number_clay_robots == 0
        && min_turns_to_collect(geode_robot_cost.obsidian)
            + min_turns_to_collect(obsidian_robot_cost.clay)
            > (start.time_left - 1) as i64
    {
        // collecting all the clay and obsidian would only leave us one turn - no way to get geodes
        seen_states.insert(start.clone(), 0);
        return 0;
    }

    // otherwise we still have some time left - if we don't have any geode robots,
    // see if there would still be time to make any
    if number_geode_robots == 0 {
        // no geode robots yet - can we get any geodes in the remaining time?
        let obsidian_so_far = start.current_stock.obsidian;
        // need to spend a turn building the geode robot too
        let time_to_stock_up = (start.time_left - 1) as i64;
        let obsidian_needed = geode_robot_cost.obsidian - obsidian_so_far;
        // suppose we can build an obsidian robot every turn from now until then
        let max_obsidian_gatherable =
            (1..=time_to_stock_up).sum::<i64>() + number_obsidian_robots * time_to_stock_up;
        // let's try and prune some more branches:
        // how many obsidian robots could we possibly build?

        if obsidian_needed > 0 && max_obsidian_gatherable < obsidian_needed {
            // no way we could build a geode robot and send it out in time - remember that this is
            // a dead end
            seen_states.insert(start.clone(), 0);
            return 0;
        }
    }

    // otherwise it has at least one step left - move one time unit forward and see
    // what the options are
    let mut best_geodes = 0;
    for step in start.next_steps(prices) {
        // if we took a step forward to a situation we've seen before,
        // we can just fetch the answer - otherwise, recurse (remembering)
        // and calculate the answer afresh
        let most_geodes_for_this_step = if let Some(geodes) = seen_states.get(&step) {
            *geodes
        } else {
            let most_geodes = get_most_geodes(&step, prices, seen_states);
            seen_states.insert(step, most_geodes);
            most_geodes
        };
        best_geodes = best_geodes.max(most_geodes_for_this_step);
    }

    // remember that this is the best we can do for this state
    seen_states.insert(start.clone(), best_geodes);
    best_geodes
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Blueprint {
    id: u8,
    prices: HashMap<Robot, Stock>,
}

pub fn input_generator_part1(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let (intro, defn) = l.strip_suffix('.').unwrap().split_once(": ").unwrap();
            let id = intro.strip_prefix("Blueprint ").unwrap().parse().unwrap();
            let mut robots = defn.split(". ");
            let ore_robot = robots.next().unwrap();
            let ore_robot_ore_cost = ore_robot
                .strip_prefix("Each ore robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap();

            let clay_robot = robots.next().unwrap();
            let clay_robot_ore_cost = clay_robot
                .strip_prefix("Each clay robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap();

            let obsidian_robot = robots.next().unwrap();
            let obsidian_robot_ore_costs = obsidian_robot
                .strip_prefix("Each obsidian robot costs ")
                .unwrap();
            let (ore_amount, clay_amount) = obsidian_robot_ore_costs.split_once(" and ").unwrap();
            let obsidian_ore_amount = ore_amount.strip_suffix(" ore").unwrap().parse().unwrap();
            let obsidian_clay_amount = clay_amount.strip_suffix(" clay").unwrap().parse().unwrap();

            let geode_robot = robots.next().unwrap();
            let geode_robot_ore_costs =
                geode_robot.strip_prefix("Each geode robot costs ").unwrap();
            let (ore_amount, obs_amount) = geode_robot_ore_costs.split_once(" and ").unwrap();
            let geode_ore_amount = ore_amount.strip_suffix(" ore").unwrap().parse().unwrap();
            let geode_obs_amount = obs_amount
                .strip_suffix(" obsidian")
                .unwrap()
                .parse()
                .unwrap();

            Blueprint {
                id,
                prices: HashMap::from([
                    (Robot::Ore, (ore_robot_ore_cost, 0, 0, 0).into()),
                    (Robot::Clay, (clay_robot_ore_cost, 0, 0, 0).into()),
                    (
                        Robot::Obsidian,
                        (obsidian_ore_amount, obsidian_clay_amount, 0, 0).into(),
                    ),
                    (
                        Robot::Geode,
                        (geode_ore_amount, 0, geode_obs_amount, 0).into(),
                    ),
                ]),
            }
        })
        .collect()
}

pub fn solve_part1(input: &Vec<Blueprint>) -> i64 {
    let mut total = 0;
    for b in input {
        let mut seen_states = HashMap::new();
        let blueprint_optimum = get_most_geodes(&Factory::new(24), &b.prices, &mut seen_states);
        let quality_level = blueprint_optimum * (b.id as i64);
        total += quality_level;
    }

    total
}

pub fn solve_part2(input: &[Blueprint]) -> i64 {
    let mut total = 1;
    for b in input.iter().take(3) {
        let mut seen_states = HashMap::new();
        let blueprint_optimum = get_most_geodes(&Factory::new(32), &b.prices, &mut seen_states);
        total *= blueprint_optimum;
    }

    total
}

#[test]
fn test_day19_input1() {
    let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    // let part2_result = solve_part2(&parsed_input);

    println!("{}", part1_result)
    // assert_eq!(part1_result, 33);
    // assert_eq!(part2_result, 0);
}

#[test]
fn test_number_turns() {
    assert_eq!(min_turns_to_collect(1), 2);
    assert_eq!(min_turns_to_collect(2), 3);
    assert_eq!(min_turns_to_collect(3), 3);
    assert_eq!(min_turns_to_collect(7), 5);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt").trim());

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}