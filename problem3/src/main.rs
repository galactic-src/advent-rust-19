use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

extern crate nom;

use nom::{IResult,
          combinator::map_res,
          bytes::complete::{take_while_m_n}};

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &(Vec<Move>, Vec<Move>)) {
    let distance = find_closest_intersection(input);
    println!("part 1: {}", distance);
}

fn part2(input: &(Vec<Move>, Vec<Move>)) {
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);

    let keys1: HashSet<&Point> = visited1.keys().collect();
    let keys2: HashSet<&Point> = visited2.keys().collect();

    let distance = keys1.intersection(&keys2)
        .map(|intersection| visited1[intersection] + visited2[intersection])
        .min().expect("no intersections");
    println!("part 2: {}", distance);
}

fn find_closest_intersection(input: &(Vec<Move>, Vec<Move>)) -> u32 {
    println!("finding closest");
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);

    let keys1: HashSet<&Point> = visited1.keys().collect();
    let keys2: HashSet<&Point> = visited2.keys().collect();
    return keys1.intersection(&keys2)
        .map(|intersection| (intersection.x.abs() as u32) + (intersection.y.abs() as u32))
        .min().expect("no intersections");
}

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

    fn copy(&self) -> Point {
        Point {x: self.x, y: self.y}
    }
}

fn visited(moves: &Vec<Move>) -> HashMap<Point, u32> {
    let mut result: HashMap<Point, u32> = HashMap::new();

    let mut end = Point{ x: 0, y: 0};
    let end_ptr = &mut end;
    let mut total_len: u32 = 0;

    for next in moves {
        match next.direction {
            Direction::UP => {
                for _ in 1..(next.distance+1) {
                    end_ptr.add(0, 1);
                    total_len += 1;
                    result.entry(end_ptr).or_insert(total_len);
                }
            }
            Direction::DOWN => {
                for _ in 1..(next.distance+1) {
                    end_ptr.add(0,-1);
                    total_len += 1;
                    result.entry(end_ptr.copy()).or_insert(total_len);
                }
            }
            Direction::LEFT => {
                for _ in 1..(next.distance+1) {
                    end_ptr.add(-1, 0);
                    total_len += 1;
                    result.entry(end_ptr.copy()).or_insert(total_len);
                }
            }
            Direction::RIGHT => {
                for _ in 1..(next.distance+1) {
                    end_ptr.add(1, 0);
                    total_len += 1;
                    result.entry(end_ptr.copy()).or_insert(total_len);
                }
            }
        }
        //println!("{}", end);
    }

    //println!("{:?}",result);
    result
}

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: Direction,
    distance: u32
}

fn convert_direction(input: &str) -> Result<Direction, std::num::ParseIntError> {
    match input {
        "U" => Ok(Direction::UP),
        "D" => Ok(Direction::DOWN),
        "L" => Ok(Direction::LEFT),
        "R" => Ok(Direction::RIGHT),
        _ => panic!("unrecognised direction: {}", input)
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map_res(
        take_while_m_n(1, 1, |_|true),
        convert_direction
    )(input)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 4, is_digit),
        parse_u32
    )(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, direction) = parse_direction(input)?;
    let (input, distance) = parse_int(input)?;

    Ok((input, Move{direction, distance}))
}

fn read_input_line(line: &str) -> Vec<Move> {
    line.split(",").map(|tok|parse_move(&tok).expect("failed to parse move").1).collect()
}

fn read_input() -> (Vec<Move>, Vec<Move>) {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut input: Vec<Vec<Move>> = reader
        .lines().map(|line| read_input_line(&line.unwrap())).collect();
    (input.remove(0), input.remove(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        assert_eq!(parse_move("U123"), Ok(("", Move{direction: Direction::UP, distance: 123})))
    }

//    #[test]
 //   fn test_simple() {
//        let input = (read_input_line("U5,L3,D10"), read_input_line("D5,L10"));
//        assert_eq!(find_closest_intersection(&input), 159);
//    }

    #[test]
    fn test_example1() {
        let input = (read_input_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"), read_input_line("U62,R66,U55,R34,D71,R55,D58,R83"));
        let distance = find_closest_intersection(&input);
        assert_eq!(distance, 159);
    }
}