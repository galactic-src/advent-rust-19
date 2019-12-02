use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let module_masses = read_input();
    part1(&module_masses);
    part2(&module_masses);
}

fn read_input() -> Vec<u32> {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines()
        .map(|line| line.expect("failed to read line")
            .parse::<u32>().expect("failed to parse u32"))
        .collect()
}

fn part1(module_masses: &Vec<u32>) {
    let total_fuel: u32 = module_masses.iter()
        .map(|mass| mass/3 - 2)
        .sum();
    println!("part 1: {}", total_fuel);
}

fn part2(module_masses: &Vec<u32>) {
    let total_fuel: u32 = module_masses.iter()
        .map(|mass| fuel_required_recursive(*mass))
        .sum();
    println!("part 2: {}", total_fuel);
}

fn fuel_required_recursive(module_mass: u32) -> u32 {
    match module_mass {
        d if d < 9 => return 0,
        _ => {
            let extra_fuel = module_mass / 3 - 2;
            return extra_fuel + fuel_required_recursive(extra_fuel);
        }
    }
}