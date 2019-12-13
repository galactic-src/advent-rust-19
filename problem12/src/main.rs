use std::collections::{HashSet, BTreeSet};

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Universe {
    moons: [MoonState; 4]
}

impl Universe {
    fn to_locations(&self) -> UniverseLocations {
        return UniverseLocations {
            moons: [
                self.moons[0].location.clone(),
                self.moons[1].location.clone(),
                self.moons[2].location.clone(),
                self.moons[3].location.clone()
            ]
        }
    }
}

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct UniverseLocations {
    moons: [MoonLocation; 4]
}

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct MoonLocation {
    x: i16,
    y: i16,
    z: i16
}

impl MoonLocation {
    fn new () -> MoonLocation {
        MoonLocation {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct MoonVelocity {
    v_x: i16,
    v_y: i16,
    v_z: i16
}

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct MoonState {
    location: MoonLocation,
    velocity: MoonVelocity
}

impl MoonVelocity {
    fn new() -> MoonVelocity {
        MoonVelocity {v_x: 0, v_y: 0, v_z: 0}
    }
}

fn main() {
    let input = vec!(
        MoonLocation{x: 15, y: -2, z: -6},
        MoonLocation{x: 0, y: -6, z: 0},
        MoonLocation{x: -5, y: -4, z: -11},
        MoonLocation{x: 5, y: 9, z: 6}
    );

    part1(input.to_vec());
    part2(input);
}

fn part1(mut locations: Vec<MoonLocation>) {
    let mut velocities = vec!(MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new());

    run_n_steps(1000, &mut locations, &mut velocities);

    println!("part 1: {}", energy(&locations, &velocities));
}

fn part2(locations: Vec<MoonLocation>) {
    //let steps = find_repeated_state(locations);
    let universe = Universe {
        moons: [
            MoonState {location: locations[0], velocity: MoonVelocity::new()},
            MoonState {location: locations[1], velocity: MoonVelocity::new()},
            MoonState {location: locations[2], velocity: MoonVelocity::new()},
            MoonState {location: locations[3], velocity: MoonVelocity::new()}
        ]
    };
    let steps = find_repeated_state_u(universe);
    println!("part 2: {}", steps);
}

fn find_repeated_state(mut locations: Vec<MoonLocation>) -> usize {
    let mut velocities = vec!(MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new());
    let mut first_state: BTreeSet<MoonState> = BTreeSet::new();
    for i in 0..4 {
        first_state.insert(MoonState{location: locations[i].clone(), velocity: velocities[i].clone()});
    }
    let mut states: HashSet<BTreeSet<MoonState>> = HashSet::new();
    states.insert(first_state);


    let mut steps: usize = 0;
    loop {
        run_step(&mut locations, &mut velocities);

        let mut state: BTreeSet<MoonState> = BTreeSet::new();
        for i in 0..4 {
            state.insert(MoonState{location: locations[i].clone(), velocity: velocities[i].clone()});
        }

        steps += 1;
        if steps % 100000 == 0 {
            println!("{}", steps);
        }

        if states.contains(&state) {
            return steps;
        } else {
            states.insert(state);
        }
    }
}

fn find_repeated_state_u(mut universe: Universe) -> usize {
    let universe_origin = UniverseLocations {
        moons: [
            MoonLocation::new(),
            MoonLocation::new(),
            MoonLocation::new(),
            MoonLocation::new(),
        ]
    };
    let mut universe_delta = universe_origin.clone();

    let mut steps: usize = 0;

    loop {
        run_step_u(&mut universe);

        for i in 0..4 {
            universe_delta.moons[i].x += universe.moons[i].velocity.v_x;
            universe_delta.moons[i].y += universe.moons[i].velocity.v_y;
            universe_delta.moons[i].z += universe.moons[i].velocity.v_z;
        }

        steps += 1;

        if steps % 100000000 == 0 {
            println!("{}", steps);
        }

        if universe_delta == universe_origin {
            return steps;
        }
    }
}

fn run_n_steps(n: usize, mut locations: &mut Vec<MoonLocation>, mut velocities: &mut Vec<MoonVelocity>) {
    for _ in 0..n {
        // update velocities
        run_step(&mut locations, &mut velocities);
    }
}

fn run_step(locations: &mut Vec<MoonLocation>, velocities: &mut Vec<MoonVelocity>) {
    for l1 in 1..locations.len() {
        for l2 in 0..l1 {

            let x1 = locations[l1].x;
            let y1 = locations[l1].y;
            let z1 = locations[l1].z;
            let x2 = locations[l2].x;
            let y2 = locations[l2].y;
            let z2 = locations[l2].z;

            if x1 > x2 {
                velocities[l1].v_x -= 1;
                velocities[l2].v_x += 1;
            } else if x1 < x2 {
                velocities[l1].v_x += 1;
                velocities[l2].v_x -= 1;
            }
            if y1 > y2 {
                velocities[l1].v_y -= 1;
                velocities[l2].v_y += 1;
            } else if y1 < y2 {
                velocities[l1].v_y += 1;
                velocities[l2].v_y -= 1;
            }
            if z1 > z2 {
                velocities[l1].v_z -= 1;
                velocities[l2].v_z += 1;
            } else if z1 < z2 {
                velocities[l1].v_z += 1;
                velocities[l2].v_z -= 1;
            }
        }
    }
    //update locations
    for i in 0..locations.len() {
        locations[i].x += velocities[i].v_x;
        locations[i].y += velocities[i].v_y;
        locations[i].z += velocities[i].v_z;
    }
}

fn run_step_u(universe: &mut Universe) {
    for l1 in 1..4 {
        for l2 in 0..l1 {

            let x1 = universe.moons[l1].location.x;
            let y1 = universe.moons[l1].location.y;
            let z1 = universe.moons[l1].location.z;
            let x2 = universe.moons[l2].location.x;
            let y2 = universe.moons[l2].location.y;
            let z2 = universe.moons[l2].location.z;

            if x1 > x2 {
                universe.moons[l1].velocity.v_x -= 1;
                universe.moons[l2].velocity.v_x += 1;
            } else if x1 < x2 {
                universe.moons[l1].velocity.v_x += 1;
                universe.moons[l2].velocity.v_x -= 1;
            }
            if y1 > y2 {
                universe.moons[l1].velocity.v_y -= 1;
                universe.moons[l2].velocity.v_y += 1;
            } else if y1 < y2 {
                universe.moons[l1].velocity.v_y += 1;
                universe.moons[l2].velocity.v_y -= 1;
            }
            if z1 > z2 {
                universe.moons[l1].velocity.v_z -= 1;
                universe.moons[l2].velocity.v_z += 1;
            } else if z1 < z2 {
                universe.moons[l1].velocity.v_z += 1;
                universe.moons[l2].velocity.v_z -= 1;
            }
        }
    }
    //update locations
    for i in 0..4 {
        universe.moons[i].location.x += universe.moons[i].velocity.v_x;
        universe.moons[i].location.y += universe.moons[i].velocity.v_y;
        universe.moons[i].location.z += universe.moons[i].velocity.v_z;
    }
}

fn energy(locations: &Vec<MoonLocation>, velocities: &Vec<MoonVelocity>) -> usize {
    locations.iter().zip(velocities).map(|(l,v)|((l.x.abs() + l.y.abs() + l.z.abs())*(v.v_x.abs()+v.v_y.abs()+v.v_z.abs())) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let locations = vec!(
            MoonLocation{x: 2,y: 1,z: -3},
            MoonLocation{x: 1,y:-8,z:0},
            MoonLocation{x: 3,y: -6,z: 1},
            MoonLocation{x: 2,y:0,z:4});
        let velocities = vec!(
            MoonVelocity{v_x: -3,v_y: -2,v_z: 1},
            MoonVelocity{v_x: -1,v_y: 1,v_z: 3},
            MoonVelocity{v_x: 3,v_y: 2,v_z: -3},
            MoonVelocity{v_x: 1,v_y: -1,v_z: -1});
        assert_eq!(energy(&locations,&velocities), 179);
    }

    #[test]
    fn test_find_state_repeat_0() {
        let locations = vec!(
            MoonLocation{x: 0, y: 0,z: 0},
            MoonLocation{x: 0, y:0, z:0},
            MoonLocation{x: 0, y: 0, z: 0},
            MoonLocation{x: 0,y: 0,z:0});

        let result = find_repeated_state(locations);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_steps() {
//        <x=-1, y=0, z=2>
//        <x=2, y=-10, z=-7>
//        <x=4, y=-8, z=8>
//        <x=3, y=5, z=-1>
        let mut locations = vec!(
            MoonLocation{x: -1, y: 0,z: 2},
            MoonLocation{x: 2, y:-10, z:-7},
            MoonLocation{x: 4, y: -8, z: 8},
            MoonLocation{x: 3,y: 5,z:-1});
        let mut velocities = vec!(MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new(), MoonVelocity::new());
        run_step(locations, velocities);
        //let result = find_repeated_state(locations);

//        pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
//        pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
//        pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
//        pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
        assert_eq!(locations, vec!(
            MoonLocation{x: 2, y: -1,z: 1},
            MoonLocation{x: 3, y:-7, z:-4},
            MoonLocation{x: 1, y: -7, z: 5},
            MoonLocation{x: 3,y: 5,z:-1}));
        assert_eq!(velocities, vec!())
    }



    #[test]
    fn test_find_state_repeat() {
        let locations = vec!(
            MoonLocation{x: -8, y: -10,z: 0},
            MoonLocation{x: 5, y:5, z:10},
            MoonLocation{x: 2, y: -7, z: 3},
            MoonLocation{x: 9,y:-8,z:-3});

        let result = find_repeated_state(locations);
        assert_eq!(result, 4686774924);
    }
}