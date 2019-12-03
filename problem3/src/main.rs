use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

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
    let closest = find_closest_intersection(input);


    println!("part 1: {}", closest.0.abs() + closest.1.abs());
}

fn part2(input: &(Vec<Move>, Vec<Move>)) {
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);
    let mut intersections: HashSet<((i32,i32,u32),(i32,i32,u32))> = HashSet::new();
    for a in &visited1 {
        for b in &visited2 {
            if a.0 == b.0 && a.1 == b.1 {
                intersections.insert((*a,*b));
            }
        }
    }
    //let mut intersections = visited1.intersection(&visited2);

    println!("{:?}",intersections);

    let first_visited: &((i32,i32,u32),(i32,i32,u32)) = intersections.iter().next().expect("no intersections found");
    let mut shortest_len: u32 = (first_visited.0).2 + (first_visited.1).2;
    for ((_, _, a),(_,_,b)) in intersections {
        if (a + b) < shortest_len {
            shortest_len = a+b;
        }
    }

    println!("part 2 {}", shortest_len);
}

fn find_closest_intersection(input: &(Vec<Move>, Vec<Move>)) -> (i32,i32) {
    println!("finding closest");
    let visited1 = visited(&input.0);
    let visited2 = visited(&input.1);
    let mut intersections: HashSet<((i32,i32,u32),(i32,i32,u32))> = HashSet::new();
    for a in &visited1 {
        for b in &visited2 {
            if a.0 == b.0 && a.1 == b.1 {
                intersections.insert((*a,*b));
            }
        }
    }
    //let mut intersections = visited1.intersection(&visited2);

    println!("{:?}",intersections);

    let first_visited: &((i32,i32,u32),(i32,i32,u32)) = intersections.iter().next().expect("no intersections found");
    let mut closest_x = (first_visited.0).0;
    let mut closest_y = (first_visited.0).1;
    for ((x, y, _),(_,_,_)) in intersections {
        println!("{},{} vs {},{}", x, y, closest_x, closest_y);
        if (x.abs() + y.abs()) < (closest_x.abs() + closest_y.abs()) {
            closest_x = x;
            closest_y = y;
        }
    }

    (closest_x, closest_y)
}

fn visited(moves: &Vec<Move>) -> HashSet<(i32, i32, u32)> {
    let mut result: HashSet<(i32,i32,u32)> = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut len: u32 = 0;
    for next in moves {
        match next.direction {
            Direction::UP => {
                for _ in 1..(next.distance+1) {
                    y += 1;
                    len += 1;
                    result.insert((x,y,len));
                }
            }
            Direction::DOWN => {
                for _ in 1..(next.distance+1) {
                    y -= 1;
                    len += 1;
                    result.insert((x,y,len));
                }
            }
            Direction::LEFT => {
                for _ in 1..(next.distance+1) {
                    x -= 1;
                    len += 1;
                    result.insert((x,y,len));
                }
            }
            Direction::RIGHT => {
                for _ in 1..(next.distance+1) {
                    x += 1;
                    len += 1;
                    result.insert((x,y,len));
                }
            }
        }
        println!("({},{})", x, y);
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
        assert_eq!(find_closest_intersection(&input), 159);
    }
}