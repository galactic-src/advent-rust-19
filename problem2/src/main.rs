use std::fs::File;
use std::io::{BufReader, BufRead};

type Storage = Vec<u32>;

pub struct VM {
    ip: u32,
    storage: Storage
}

impl VM {
    fn read(&self, address: u32) -> u32 {
        self.storage[address as usize]
    }

    fn read_ptr(&self, address: u32) -> u32 {
        let ptr = self.read(address);
        self.read(ptr)
    }

    fn write(&mut self, address: u32, value: u32) {
        self.storage[address as usize] = value;
    }

    fn write_ptr(&mut self, address: u32, value: u32) {
        let ptr = self.read(address);
        self.write(ptr, value);
    }


    fn init(&mut self, noun: u32, verb: u32) {
        self.write(1, noun);
        self.write(2, verb);
    }


    fn run(&mut self) {
        while self.execute_step() {}
    }

    fn execute_step(&mut self) -> bool {
        let op = self.read(self.ip);
        match op {
            1 => {
                let result = self.read_ptr(self.ip+1) + self.read_ptr(self.ip+2);
                self.write_ptr(self.ip+3, result);
                self.ip += 4;
            }
            2 => {
                let result = self.read_ptr(self.ip+1) * self.read_ptr(self.ip+2);
                self.write_ptr(self.ip+3, result);
                self.ip += 4;
            }
            99 => return false,
            _ => panic!("unrecognised op: {}", )
        }
        return true;
    }
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn read_input() -> Vec<u32> {
    let file = File::open("src/input").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read line");
    input.split(",")
        .map(|tok| tok.parse::<u32>().expect("failed to parse u32"))
        .collect()
}

fn part1(ints: &Vec<u32>) {
    let mut vm = VM {ip: 0, storage: ints.to_vec()};
    vm.init(12, 2);
    vm.run();
    println!("part 1: {}", vm.read(0));
}

fn part2(ints: &Vec<u32>) {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut vm = VM {ip: 0, storage: ints.to_vec()};
            vm.init(noun, verb);
            vm.run();
            if vm.read(0) == 19690720 {
                println!("part 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}