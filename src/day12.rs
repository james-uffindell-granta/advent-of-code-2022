use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Elevation {
    value: char,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Elevation {
    pub fn can_move_to(&self, next: &Elevation) -> bool {
        (next.value as i32) - (self.value as i32) <= 1
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: HashMap<Coord, Elevation>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Forwards,
    Backwards,
}

impl Grid {
    pub fn moves_from(&self, Coord { x, y }: Coord) -> Vec<Coord> {
        let all_moves = match (x, y) {
            (0, 0) => vec![(x + 1, y), (x, y + 1)],
            (0, _) => vec![(x + 1, y), (x, y - 1), (x, y + 1)],
            (_, 0) => vec![(x - 1, y), (x + 1, y), (x, y + 1)],
            _ => vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)],
            
        };
        all_moves.into_iter()
        .map(|(x, y)| Coord { x, y })
        .filter(|c| self.grid.contains_key(&c))
        .collect()
    }

    // brute force path checking is too slow; use dijkstra's algorithm
    pub fn calculate_distances(&self, start: &Coord, direction: Direction) -> HashMap<Coord, Option<usize>> {
        let mut unvisited_nodes = self.grid.keys().cloned().collect::<HashSet<_>>();
        let mut distance_map = unvisited_nodes.iter().cloned()
            .map(|n| (n, None)).collect::<HashMap<_, _>>();

        *distance_map.get_mut(start).unwrap() = Some(0usize);

        let mut smallest_node = unvisited_nodes.iter().cloned()
        .filter(|c| matches!(distance_map.get(c).unwrap(), Some(_)))
        .min_by_key(|c| {
            match distance_map.get(c).unwrap() {
                Some(x) => x,
                _ => unreachable!(),
            }
        });

        while let Some(current_node) = smallest_node {
            // get the reachable, unvisited, neighbours
            let neighbours = self.moves_from(current_node).into_iter()
                .filter(|c| unvisited_nodes.contains(&c))
                .filter(|c| match direction {
                    Direction::Forwards => self.grid.get(&current_node).unwrap().can_move_to(self.grid.get(c).unwrap()),
                    Direction::Backwards => self.grid.get(c).unwrap().can_move_to(self.grid.get(&current_node).unwrap()),
            }).collect::<Vec<_>>();
            let current_distance = match distance_map.get(&current_node).unwrap() {
                Some(d) => *d,
                None => unreachable!(),
            };

            for n in neighbours {
                let distance = current_distance + 1;
                let new_value = match distance_map.get(&n).unwrap() {
                    Some(existing_value) => {
                        if *existing_value > distance {
                            // found a shorter route
                            distance
                        } else {
                            *existing_value
                        }
                    },
                    None => distance
                };
                distance_map.insert(n, Some(new_value));
            }

            unvisited_nodes.remove(&current_node);
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
}

pub struct Input {
    map: Grid,
    starting_coord: Coord,
    ending_coord: Coord,
}

#[aoc_generator(day12)]
pub fn input_generator_part1(input: &str) -> Input {
    let mut start_coord = Coord { x: 0, y: 0 };
    let mut end_coord = Coord { x: 0, y: 0 };
    let mut grid = HashMap::new();
    for (row_number, line) in input.lines().enumerate() {
        for (column_number, c) in line.chars().enumerate() {
            let ch = match c {
                'S' => { start_coord = Coord { x: column_number, y: row_number }; 'a' },
                'E' => { end_coord = Coord { x: column_number, y: row_number }; 'z' },
                _ => c,
            };
            grid.insert(Coord { x: column_number, y: row_number }, Elevation { value: ch });
        }
    }

    Input { map: Grid { grid }, starting_coord: start_coord, ending_coord: end_coord }
}


#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let distance_map = input.map.calculate_distances(&input.starting_coord, Direction::Forwards);
    distance_map.get(&input.ending_coord).unwrap().unwrap()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let distance_map = input.map.calculate_distances(&input.ending_coord, Direction::Backwards);
    let lowest_elevation_squares = input.map.grid.iter().filter(|(_, e)| e.value == 'a').map(|(c, _)| c).collect::<Vec<_>>();
    lowest_elevation_squares.into_iter().filter_map(|c| *distance_map.get(&c).unwrap()).min().unwrap()
}

#[test]
fn test_day12_input1() {
    let input =
r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 31);
    assert_eq!(part2_result, 29);
}