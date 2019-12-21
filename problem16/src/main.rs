use std::fs::File;
use std::io::{BufReader, BufRead};

use num::Integer;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

fn main() {
    let input = read_input();
    println!("{:?}", &input);
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i64>) {
    let result = apply_phases(input, 100);
    let code: String = result.iter().map(|i|i.to_string()).collect();
    println!("part 1: {}", code);
}

fn part2(input: &Vec<i64>) {
    let offset_s: String = input[0..7].iter()
        .map(|i|i.to_string()).collect();
    let offset = offset_s.parse::<usize>().expect("couldn't parse offset");
    let total_len = input.len() * 10000;

    println!("offset: {}, total_len: {}", offset, total_len);

    let mut input_from_offset = repeat(input.to_vec()).flatten().skip(offset % input.len())
        .take(input.len() * repeats - offset).collect();
    let mut next_output;

    for _ in 0..100 {
        next_output = vec![0; input_from_offset.len()];
        for i in 0..(input_from_offset.len()) {

        }

        input_from_offset = next_output;
    }

    //println!("part 2: {}", code);
}

fn identity(i: usize) -> Vec<Vec<i64>> {
    (0..i).map(|n|(0..i).map(|m| if m == n {1} else {0}).collect()).collect()
}

fn apply_n_phases(input: &Vec<i64>, mut phase_count: usize) -> Vec<i64> {
    let input_len = input.len();
    let mut phase_matrix: Vec<Vec<i64>> = (1..=input_len).map(|i|{
            println!("phase matrix row {}", i); pattern(input_len, i)
        }).collect();
    println!("made initial phase_matrix");
    let mut accumulator = identity(input_len);

    loop {
        if phase_count % 2 == 1 {
            accumulator = matrix_mul(&accumulator, &phase_matrix)
        }

        phase_count /= 2;
        println!("{}", phase_count);
        if phase_count == 0 {
            break;
        }

        phase_matrix = matrix_mul(&phase_matrix, &phase_matrix);
    }

    vector_mul(input, &accumulator)
}

fn matrix_mul(m1: &Vec<Vec<i64>>, m2: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let width = m1[0].len();
    let height = m1.len();

    (0..height).map(|row|
        (0..width).map(|col|
            (0..height).map(|ix| {
                println!("{} x {} = {}", m1[row][ix], m2[ix][col], m1[row][ix] * m2[ix][col]);
                m1[row][ix] * m2[ix][col]}
            ).sum()
        ).collect()
    ).collect()
}

fn vector_mul(v: &Vec<i64>, m: &Vec<Vec<i64>>) -> Vec<i64> {
    (0..v.len())
        .map(|i|
            v.iter()
                .zip(m[i].iter())
                .map(|(v_i, m_i)| v_i * m_i)
                .sum())
        .collect()
}

fn apply_phases(input: &Vec<i64>, phase_count: usize) -> Vec<i64> {
    let mut accumulator: Vec<i64> = (*input).clone();

    for phase in 0..phase_count {
        println!("phase {}", phase);

        let mut next_accumulator = vec!();

        for ix in 1..=input.len() {
            next_accumulator.push(apply_phase(&accumulator, ix).abs()%10);
            //println!("added to accumulator: {:?}", next_accumulator);
        }

        accumulator = next_accumulator;
        //println!("{:?}", accumulator);
    }

    accumulator
}

fn apply_phase(input: &Vec<i64>, iteration: usize) -> i64 {
    input.iter()
        .zip(&pattern(input.len(), iteration))
        .filter_map(|(a, b)| if *a == 0 || *b == 0 { None} else {Some(a*b)})
        .sum()
}

fn pattern(input_length: usize, iteration: usize) -> Vec<i64> {
    (1..(input_length+1)).map(|i| BASE_PATTERN[(i/iteration)%4]).collect()
}

fn read_input() -> Vec<i64> {
    let file = File::open("src/input").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read line");
    input.chars().map(|c| c.to_digit(10).expect("wasn't a u8") as i64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern3() {
        assert_eq!(pattern(11, 3), vec!(0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1));
    }

    #[test]
    fn test_example_phases() {
        let input = vec!(1i64, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(apply_phases(&input, 1), vec!(4i64, 8, 2, 2, 6, 1, 5, 8))
    }

//    #[test]
//    fn test_example_n_phases() {
//        let input = vec!(1i64, 2, 3, 4, 5, 6, 7, 8);
//        assert_eq!(apply_phases(&input, 1), vec!(4i64, 8, 2, 2, 6, 1, 5, 8))
//    }
    #[test]
    fn test_matrix_mul_1() {
        let m1 = vec!(
            vec!(1i64, 2),
            vec!(3, 4));
        let m2 = vec!(
            vec!(5i64, 6),
            vec!(7, 8));
        assert_eq!(matrix_mul(&m1, &m2), vec!(
           vec!(19i64, 22),
           vec!(43, 50))
        )
    }

    #[test]
    fn test_vec_mul_1() {
        let v = vec!(1i64, 2);
        let m = vec!(
            vec!(5i64, 6),
            vec!(7, 8));
        assert_eq!(vector_mul(&v, &m), vec!(17i64, 23))
    }

    #[test]
    fn test_self_1() {

    }

    #[test]
    fn test_identity() {
        assert_eq!(identity(1), vec!(vec!(1)));
        assert_eq!(identity(2), vec!(vec!(1, 0), vec!(0,1)));
    }
}