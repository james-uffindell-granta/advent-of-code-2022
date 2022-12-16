use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Coord {
    x : i32,
    y: i32,
}

impl Coord {
    pub fn manhattan_distance_to(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Tile {
    Sensor,
    Beacon,
    Unknown,
}

#[derive(Clone)]
pub struct TunnelNetwork {
    map: HashMap<Coord, Tile>,
}

impl TunnelNetwork {
    pub fn add_sensor(&mut self, sensor_report: &SensorReport) {
        self.map.insert(sensor_report.sensor_location, Tile::Sensor);
        self.map.insert(sensor_report.nearest_beacon, Tile::Beacon);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SensorReport {
    sensor_location: Coord,
    nearest_beacon: Coord,
}

impl SensorReport {
    pub fn distance_scanned(&self) -> i32 {
        self.sensor_location.manhattan_distance_to(&self.nearest_beacon)
    }

    fn x_range_scanned(&self) -> (i32, i32) {
        let d = self.distance_scanned();
        (self.sensor_location.x - d, self.sensor_location.x + d)
    }

    pub fn y_range_scanned_at(&self, x: i32) -> Option<(i32, i32)> {
        let d = self.distance_scanned();
        let dx = (x - self.sensor_location.x).abs();
        if dx > d {
            return None;
        }
        let dy = d - dx;
        Some((self.sensor_location.y - dy, self.sensor_location.y + dy))
    }
}

pub struct Input {
    tunnels: TunnelNetwork,
    sensors: Vec<SensorReport>,
}

#[aoc_generator(day15)]
pub fn input_generator_part1(input: &str) -> Input {
    let mut sensors = Vec::new();
    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let (sensor_x, sensor_y) = sensor.strip_prefix("Sensor at ").unwrap().split_once(", ").unwrap();
        let (beacon_x, beacon_y) = beacon.strip_prefix("closest beacon is at ").unwrap().split_once(", ").unwrap();
        let sensor_location = (sensor_x.strip_prefix("x=").unwrap().parse().unwrap(), sensor_y.strip_prefix("y=").unwrap().parse().unwrap()).into();
        let nearest_beacon = (beacon_x.strip_prefix("x=").unwrap().parse().unwrap(), beacon_y.strip_prefix("y=").unwrap().parse().unwrap()).into();
        sensors.push(SensorReport { sensor_location, nearest_beacon });
    }

    let mut tunnels = TunnelNetwork { map: HashMap::new() };
    for sensor in sensors.clone() {
        tunnels.add_sensor(&sensor);
    }

    Input { tunnels, sensors }
}


#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let min_x = input.sensors.iter().map(|s| s.x_range_scanned().0).min().unwrap();
    let max_x = input.sensors.iter().map(|s| s.x_range_scanned().1).max().unwrap();
    let row = 2_000_000;

    let mut row_cells = HashSet::new();
    for x in min_x..=max_x {
        let c = (x, row).into();
        if matches!(input.tunnels.map.get(&c).unwrap_or(&Tile::Unknown), Tile::Sensor | Tile::Beacon) {
            continue;
        }
        if input.sensors.iter().any(|s| {
            s.sensor_location.manhattan_distance_to(&c) <= s.distance_scanned()
        }) {
            row_cells.insert(c);
        }
    }

    row_cells.len()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Input) -> u128 {
    let limit = 4_000_000;
    for x in 0..=limit {
        // get all the ranges and order them by lower bound
        let mut ranges = input.sensors.iter().filter_map(|s| s.y_range_scanned_at(x)).collect::<Vec<_>>();
        ranges.sort_by_key(|(min, _)| *min);
        let mut ranges = ranges.iter();
        let mut y = 0;
        while y <= limit {
            while let Some((min, max)) = ranges.next() {
                if y < *min {
                    // next range starts after where we currently are - so beacon must be this point
                    return (x as u128) * (4_000_000 as u128) + (y as u128);
                }
                if y >= *min && y < *max {
                    // inside a range, so can't be anything in this range - skip past it
                    y = *max + 1;
                }
            }
        }
    }

    unreachable!()
}

#[test]
fn test_day15_input1() {
    let input =
r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 26);
    assert_eq!(part2_result, 56_000_011);
}