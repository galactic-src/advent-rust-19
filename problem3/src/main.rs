use std::collections::{hash_map::{HashMap, RandomState},
                       hash_set::{HashSet, Intersection}
};

mod parse;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn add(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

type Wire = HashMap<Point, u32>;

pub struct Wires {
    wire1: Wire,
    wire2: Wire
}

impl Wires {
    fn crossings(&self) -> Intersection<&Point, RandomState> {
        let keys1: HashSet<&Point> = self.wire1.keys().collect();
        let keys2: HashSet<&Point> = self.wire2.keys().collect();

        keys1.intersection(&keys2)
    }
}

fn main() {
    let (moves1, moves2) = parse::read_input();
    let wires = Wires{
        wire1: visited(moves1),
        wire2: visited(moves2)
    };

    part1(&wires);
    part2(&wires);
}

fn part1(input: &Wires) {
    let distance = find_closest_intersection(&input);
    println!("part 1: {}", distance);
}

fn part2(input: &Wires) {

    let distance = input.crossings()
        .map(|intersection| input.wire1[intersection] + input.wire2[intersection])
        .min().expect("no intersections");
    println!("part 2: {}", distance);
}

fn find_closest_intersection(input: &Wires) -> u32 {
    let keys1: HashSet<&Point> = input.wire1.keys().collect();
    let keys2: HashSet<&Point> = input.wire2.keys().collect();
    return keys1.intersection(&keys2)
        .map(|intersection| (intersection.x.abs() as u32) + (intersection.y.abs() as u32))
        .min().expect("no intersections");
}

fn visited(moves: Vec<parse::Move>) -> Wire {
    let mut result: Wire = HashMap::new();

    let mut end = Point{ x: 0, y: 0};
    let end_ptr = &mut end;
    let mut total_len: u32 = 0;

    for next in moves {
        match next.direction() {
            parse::Direction::UP => {
                for _ in 1..(next.distance() + 1) {
                    end_ptr.add(0, 1);
                    total_len += 1;
                    result.entry(*end_ptr).or_insert(total_len);
                }
            }
            parse::Direction::DOWN => {
                for _ in 1..(next.distance() + 1) {
                    end_ptr.add(0,-1);
                    total_len += 1;
                    result.entry(*end_ptr).or_insert(total_len);
                }
            }
            parse::Direction::LEFT => {
                for _ in 1..(next.distance() + 1) {
                    end_ptr.add(-1, 0);
                    total_len += 1;
                    result.entry(*end_ptr).or_insert(total_len);
                }
            }
            parse::Direction::RIGHT => {
                for _ in 1..(next.distance() + 1) {
                    end_ptr.add(1, 0);
                    total_len += 1;
                    result.entry(*end_ptr).or_insert(total_len);
                }
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let wires = Wires {
            wire1: visited(parse::read_input_line("R75,D30,R83,U83,L12,D49,R71,U7,L72")),
            wire2: visited(parse::read_input_line("U62,R66,U55,R34,D71,R55,D58,R83"))
        };
        let distance = find_closest_intersection(&wires);
        assert_eq!(distance, 159);
    }
}