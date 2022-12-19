use std::{
    collections::HashMap,
    ops::Add
};

pub struct Input {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    pub fn cube_neighbours(self) -> Vec<Coord> {
        vec![self + (1, 0, 0),
        self + (-1, 0, 0),
        self + (0, 1, 0),
        self + (0, -1, 0),
        self + (0, 0, 1),
        self + (0, 0, -1)]
    }  
}

impl Add<(i32, i32, i32)> for Coord {
    type Output = Coord;

    fn add(self, (other_x, other_y, other_z): (i32, i32, i32)) -> Self::Output {
        Self::Output { x: self.x + other_x, y: self.y + other_y, z: self.z + other_z }
    }
}

impl From<(i32, i32, i32)> for Coord {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum CubeState {
    ReachedAndExternal,
    Reached,
    NotReached,
}


#[derive(Copy, Clone, Debug, Hash)]
pub struct Cube {
    visible_faces: u8,
    state: CubeState,
}

#[derive(Clone)]
pub struct Droplet {
    cubes: HashMap<Coord, Cube>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    internal_voids: HashMap<Coord, Cube>,
}

impl Droplet {
    pub fn new() -> Self {
        Self {
            cubes: HashMap::new(),
            min_x: i32::MAX, max_x: i32::MIN,
            min_y: i32::MAX, max_y: i32::MIN,
            min_z: i32::MAX, max_z: i32::MIN,
            internal_voids: HashMap::new(), }
    }

    pub fn add_cube_to(map: &mut HashMap<Coord, Cube>, coord: &Coord) {
        let mut neighbours_existing = 0;
        for c in coord.cube_neighbours() {
            if let Some(cube) = map.get_mut(&c) {
                cube.visible_faces -= 1;
                neighbours_existing += 1;
            }
        }
        map.insert(*coord, Cube { visible_faces: 6 - neighbours_existing, state: CubeState::NotReached });
    }

    pub fn add_cube_at(&mut self, coord: &Coord) {
        Self::add_cube_to(&mut self.cubes, coord);
        self.min_x = self.min_x.min(coord.x);
        self.max_x = self.max_x.max(coord.x);
        self.min_y = self.min_y.min(coord.y);
        self.max_y = self.max_y.max(coord.y);
        self.min_z = self.min_z.min(coord.z);
        self.max_z = self.max_z.max(coord.z);
    }

    pub fn visible_faces(&self) -> usize {
        self.cubes.iter().map(|(_, cube)| cube.visible_faces as usize).sum::<usize>()
    }

    pub fn visible_external_faces(&self) -> usize {
        let visible_faces = self.visible_faces();
        let internal_void_faces = self.internal_voids.iter().map(|(_, cube)| cube.visible_faces as usize).sum::<usize>();
        println!("{} visible faces, {} internal", visible_faces, internal_void_faces);
        visible_faces - internal_void_faces
    }

    pub fn resolve_voids(&mut self) {
        let mut unfilled_cells = HashMap::new();
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                for z in self.min_z..=self.max_z {
                    let coord = (x, y, z).into();
                    if let Some(_) = self.cubes.get(&coord) {
                        //nothing
                    } else {
                        Self::add_cube_to(&mut unfilled_cells, &coord);
                    }
                }
            }
        }

        // unfilled_cells now contains all cells within the boundary that are not part of the droplet
        loop {
            if unfilled_cells.is_empty() {
                break;
            }

            let mut new_state = unfilled_cells.clone();
            let mut updated = false;

            // check everything that we had last time that we hadn't assessed yet
            for (coord, _) in unfilled_cells.iter()
                .filter(|(_, &c)| matches!(c.state, CubeState::NotReached)) {

                let neighbours = coord.cube_neighbours();
                // ignore everything that's part of the lava droplet - we can't go that way
                let neighbours = neighbours.iter()
                    .filter(|c| !self.cubes.contains_key(c)).collect::<Vec<_>>();

                if neighbours.iter().any(|c| {
                    c.x < self.min_x || c.x > self.max_x
                    || c.y < self.min_y || c.y > self.max_y
                    || c.z < self.min_z || c.z > self.max_z
                }) {
                    // we can reach the outside from here - so this cube isn't an interal void
                    // println!("found the outside");
                    let external_cube = new_state.get_mut(coord).unwrap();
                    external_cube.state = CubeState::ReachedAndExternal;
                    updated = true;
                } else if neighbours.iter().any(|c| {
                    matches!(unfilled_cells.get(c),
                        Some(Cube { visible_faces: _, state: CubeState::ReachedAndExternal }))
                }) {
                    // println!("found another cube that found the outside");
                    let external_cube = new_state.get_mut(coord).unwrap();
                    external_cube.state = CubeState::ReachedAndExternal;
                    updated = true;
                }
            }

            unfilled_cells = new_state;

            if !updated {
                // we didn't update any cubes last time - everything is checked
                break;
            }
        }

        for (c, cube) in unfilled_cells {
            if cube.state == CubeState::NotReached {
                Self::add_cube_to(&mut self.internal_voids, &c);
            }
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator_part1(input: &str) -> Droplet {
    let mut droplet = Droplet::new();
    for line in input.lines() {
        let mut components = line.split(",");
        let x = components.next().unwrap().parse().unwrap();
        let y = components.next().unwrap().parse().unwrap();
        let z = components.next().unwrap().parse().unwrap();
        let coord = (x, y, z).into();
                droplet.add_cube_at(&coord);
    }
    droplet
}


#[aoc(day18, part1)]
pub fn solve_part1(input: &Droplet) -> usize {
    println!("Droplet spans {} to {}, {} to {}, {} to {}",
input.min_x, input.max_x, input.min_y, input.max_y, input.min_z, input.max_z);
    input.visible_faces()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Droplet) -> usize {
    let mut droplet = input.clone();
    droplet.resolve_voids();
    droplet.visible_external_faces()
}

#[test]
fn test_day18_input1() {
    let input =
r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, 64);
    assert_eq!(part2_result, 58);
}