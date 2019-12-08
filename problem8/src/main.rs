use std::fs::File;
use std::io::{BufReader, BufRead};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Layer = [[u8; WIDTH]; HEIGHT];

fn main() {
    let layers: Vec<Layer> = parse_input();
    part1(&layers);
    part2(&layers);
}

fn part1(layers: &Vec<Layer>) {
    let zero_tallies: Vec<(usize, u64)> = layers.iter().enumerate()
        .map(|(ix, layer)|(ix, count_ns(layer, 0)))
        .collect();

    let mut min: (usize, u64) = zero_tallies[0];

    for zero_tally in zero_tallies {
        if zero_tally.1 < min.1 {
            min = zero_tally;
        }
    }

    let result = count_ns(&layers[min.0], 1) * count_ns(&layers[min.0], 2);
    println!("part 1: {}", result);
}

fn count_ns(layer: &Layer, n: u8) -> u64 {
    let mut count: u64 = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if layer[y][x] == n {
                count += 1;
            }
        }
    }
    count
}

fn part2(layers: &Vec<Layer>) {
    let mut combined = [[b'-'; WIDTH]; HEIGHT];

    for layer in layers {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if combined[y][x] == b'-' {
                    combined[y][x] = match layer[y][x] {
                        2 => b'-',
                        1 => b'@',
                        0 => b' ',
                        _ => panic!("unexpected layer data")
                    };
                }
            }
        }
    }

    println!("part 2:");
    for y in 0..HEIGHT {
        println!("{}", std::str::from_utf8(&combined[y]).expect("failed to parse to string"));
    }
}

fn parse_input() -> Vec<Layer> {
    let file = File::open("src/input").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read line");

    let check = input.len() % (WIDTH * HEIGHT);
    if check != 0 {panic!("doesn't look like an exact number of layers")}
    let layers = input.len() / (WIDTH * HEIGHT);

    let input = input.into_bytes();
    let mut c = input.iter();

    let mut result: Vec<Layer> = vec!();
    for layer in 0..layers {
        result.push([[0; WIDTH]; HEIGHT]);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[layer][y][x] = c.next().expect("ran out of input") - b'0';
            }
        }
    }

    result
}