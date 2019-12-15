extern crate num;
use num::Integer;

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Universe {
    moons: [MoonState; 4]
}

#[derive(Debug,Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct MoonAxis {
    pos: i16, vel: i16
}

type UniverseAxis = [MoonAxis; 4];

impl Universe {
    fn to_axes(&self) -> [UniverseAxis; 3] {
        [
            [MoonAxis {pos: self.moons[0].location.x, vel: self.moons[0].velocity.v_x},
                MoonAxis {pos: self.moons[1].location.x, vel: self.moons[1].velocity.v_x},
                MoonAxis {pos: self.moons[2].location.x, vel: self.moons[2].velocity.v_x},
                MoonAxis {pos: self.moons[3].location.x, vel: self.moons[3].velocity.v_x}],
            [MoonAxis {pos: self.moons[0].location.y, vel: self.moons[0].velocity.v_y},
                MoonAxis {pos: self.moons[1].location.y, vel: self.moons[1].velocity.v_y},
                MoonAxis {pos: self.moons[2].location.y, vel: self.moons[2].velocity.v_y},
                MoonAxis {pos: self.moons[3].location.y, vel: self.moons[3].velocity.v_y}],
            [MoonAxis {pos: self.moons[0].location.z, vel: self.moons[0].velocity.v_z},
                MoonAxis {pos: self.moons[1].location.z, vel: self.moons[1].velocity.v_z},
                MoonAxis {pos: self.moons[2].location.z, vel: self.moons[2].velocity.v_z},
                MoonAxis {pos: self.moons[3].location.z, vel: self.moons[3].velocity.v_z}]
        ]
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
    let universe_axes = Universe {
        moons: [
            MoonState {location: locations[0], velocity: MoonVelocity::new()},
            MoonState {location: locations[1], velocity: MoonVelocity::new()},
            MoonState {location: locations[2], velocity: MoonVelocity::new()},
            MoonState {location: locations[3], velocity: MoonVelocity::new()}
        ]
    }.to_axes();

    let mut lcm: usize = 1;

    for axis in &universe_axes {
        let axis_cycle = get_axis_cycle(*axis);
        println!("axis_cycle: {}", axis_cycle);
        lcm = lcm.lcm(&axis_cycle);
    }

    println!("part 2: {}", lcm);
}

fn get_axis_cycle(mut moons: UniverseAxis) -> usize {
    let initial_state = moons.clone();
    let moons_count = moons.len();

    let mut count: usize = 0;

    loop {
        for l1 in 1..moons_count {
            for l2 in 0..l1 {
                let p1 = moons[l1].pos;
                let p2 = moons[l2].pos;

                if p1 > p2 {
                    moons[l1].vel -= 1;
                    moons[l2].vel += 1;
                } else if p1 < p2 {
                    moons[l1].vel+= 1;
                    moons[l2].vel -= 1;
                }
            }
        }

        for i in 0..moons_count {
            moons[i].pos += moons[i].vel;
        }

        count += 1;

        if moons == initial_state {
            return count;
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
        run_step(&mut locations, &mut velocities);
        //let result = find_repeated_state(locations);

//        pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
//        pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
//        pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
//        pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
        assert_eq!(locations, vec!(
            MoonLocation{x: 2, y: -1,z: 1},
            MoonLocation{x: 3, y:-7, z:-4},
            MoonLocation{x: 1, y: -7, z: 5},
            MoonLocation{x: 2,y: 2, z:0}));
        //assert_eq!(velocities, vec!())
    }
    
    #[test]
    fn test_find_state_repeat() {
        let universe = Universe {moons: [
            MoonState {location: MoonLocation{x: -8, y: -10,z: 0}, velocity: MoonVelocity::new()},
            MoonState {location: MoonLocation{x: 5, y:5, z:10}, velocity: MoonVelocity::new()},
            MoonState {location: MoonLocation{x: 2, y: -7, z: 3}, velocity: MoonVelocity::new()},
            MoonState {location: MoonLocation{x: 9,y:-8,z:-3}, velocity: MoonVelocity::new()}]
        };

        let result = find_repeated_state_u(universe);
        assert_eq!(result, 4686774924);
    }
}