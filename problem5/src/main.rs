use std::fs::File;
use std::io::{BufReader, BufRead};

type Storage = Vec<i64>;

pub struct VM {
    ip: i64,
    storage: Storage,
    outputs: Vec<i64>
}

impl VM {
    fn read(&self, address: i64) -> i64 {
        self.storage[address as usize]
    }

    fn read_ptr(&self, address: i64) -> i64 {
        let ptr = self.read(address);
        self.read(ptr)
    }

    fn write(&mut self, address: i64, value: i64) {
        self.storage[address as usize] = value;
    }

    fn write_ptr(&mut self, address: i64, value: i64) {
        let ptr = self.read(address);
        println!("write {} to {}", value, ptr);
        self.write(ptr, value);
    }


    fn init(&mut self, noun: i64, verb: i64) {
        self.write(1, noun);
        self.write(2, verb);
    }


    fn run(&mut self) {
        while self.execute_step() {}
    }

    fn execute_step(&mut self) -> bool {
        let mut op = self.read(self.ip);
        println!("next: {}", op);
        let mut args = vec!();
        let mut argc: usize = 0;
        let mut arginfo = 0;

        if op > 10 {
            arginfo = op / 100;
            op %= 100;
        }

        if op == 1 || op == 2 {
            argc = 3;
        } else if op == 3 || op == 4 {
            argc = 1;
        }

        for _ in 1..(argc+1) {
            args.push(arginfo % 10 == 1);
            arginfo /= 10;
        }

        println!("op: {}", op);
        println!("argc: {}", argc);
        println!("args: {:?}", args);

        while args.len() < argc {
            args.push(false);
        }

        match op {
            1 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let result = param1+param2;
                self.write_ptr(self.ip+3, result);
            }
            2 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                println!("param1: {}", param1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                println!("param2: {}", param2);
                let result = param1 * param2;
                self.write_ptr(self.ip+3, result);
            }
            3 => {
                let input = get_user_input();
                self.write_ptr(self.ip+1, input);
            }
            4 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                self.outputs.push(param1);
            }
            99 => return false,
            _ => panic!("unrecognised op: {}", op)
        }

        println!("add {} to ip", argc+1);
        self.ip += argc as i64 + 1;

        return true;
    }

    fn resolve_param(&self, imm: bool, address: i64) -> i64 {
        if imm {self.read(address)} else {self.read_ptr(address)}
    }
}

fn main() {
    let input = read_input();
    part1(&input);
    //part2(&input);
}

fn get_user_input() -> i64 {
    return 1;
}

fn read_input() -> Vec<i64> {
    let file = File::open("src/input").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read line");
    input.split(",")
        .map(|tok| tok.parse::<i64>().expect("failed to parse i64"))
        .collect()
}

fn part1(ints: &Vec<i64>) {
    let mut vm = VM {ip: 0, storage: ints.to_vec(), outputs: vec!()};
    vm.run();
    println!("part 1: {:?}", vm.outputs);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut vm = VM {ip: 0, storage: vec!(1002,4,3,4,33), outputs: vec!()};
        vm.run();
        assert_eq!(vm.storage, vec!(1002,4,3,4,99));
    }
}