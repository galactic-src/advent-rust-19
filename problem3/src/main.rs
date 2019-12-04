use std::collections::{HashSet, HashMap};

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

fn main() {
    let input = parse::read_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &(Vec<parse::Move>, Vec<parse::Move>)) {
    let distance = find_closest_intersection(input);
    println!("part 1: {}", distance);
}

fn part2(input: &(Vec<parse::Move>, Vec<parse::Move>)) {
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);

    let keys1: HashSet<&Point> = visited1.keys().collect();
    let keys2: HashSet<&Point> = visited2.keys().collect();

    let distance = keys1.intersection(&keys2)
        .map(|intersection| visited1[intersection] + visited2[intersection])
        .min().expect("no intersections");
    println!("part 2: {}", distance);
}

fn find_closest_intersection(input: &(Vec<parse::Move>, Vec<parse::Move>)) -> u32 {
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);

    let keys1: HashSet<&Point> = visited1.keys().collect();
    let keys2: HashSet<&Point> = visited2.keys().collect();
    return keys1.intersection(&keys2)
        .map(|intersection| (intersection.x.abs() as u32) + (intersection.y.abs() as u32))
        .min().expect("no intersections");
}

fn visited(moves: &Vec<parse::Move>) -> HashMap<Point, u32> {
    let mut result: HashMap<Point, u32> = HashMap::new();

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
        let input = (
            parse::read_input_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse::read_input_line("U62,R66,U55,R34,D71,R55,D58,R83")
        );
        let distance = find_closest_intersection(&input);
        assert_eq!(distance, 159);
    }
}