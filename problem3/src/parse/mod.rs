use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate nom;

use nom::{IResult,
          combinator::map_res,
          bytes::complete::{take_while_m_n}};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
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

impl Move {
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }
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

pub fn read_input_line(line: &str) -> Vec<Move> {
    line.split(",").map(|tok|parse_move(&tok).expect("failed to parse move").1).collect()
}

pub fn read_input() -> (Vec<Move>, Vec<Move>) {
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
        assert_eq!(parse_move("U123"), Ok(("", Move { direction: Direction::UP, distance: 123 })))
    }
}