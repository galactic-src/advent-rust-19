use std::fs::File;
use std::io::{BufReader, BufRead};

type Storage = Vec<i64>;

pub struct VM {
    ip: i64,
    storage: Storage,
    input: i64,
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
        //println!("write {} to {}", value, address);
        self.storage[address as usize] = value;
    }

    fn write_ptr(&mut self, address: i64, value: i64) {
        let ptr = self.read(address);
        self.write(ptr, value);
    }

    fn run(&mut self) {
        //println!("{:?}", self.storage);
        while self.execute_step() {}
    }

    fn execute_step(&mut self) -> bool {
        let next = self.read(self.ip);
        //println!("\tnext@{}: {}", self.ip, next);

        let mut args = vec!();
        let mut argc: usize = 0;

        let mut arginfo = next / 100;
        let op = next % 100;

        if op == 1 || op == 2 || op == 7 || op == 8 {
            argc = 3;
        } else if op == 5 || op == 6 {
            argc = 2;
        } else if op == 3 || op == 4 {
            argc = 1;
        }

        for _ in 1..(argc+1) {
            args.push(arginfo % 10 == 1);
            arginfo /= 10;
        }

        //println!("op: {}", op);
        //println!("argc: {}", argc);
        //println!("args: {:?}", args);

        while args.len() < argc {
            args.push(false);
        }

        match op {
            1 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let result = param1+param2;
                self.write_ptr(self.ip+3, result);
                self.ip += argc as i64 + 1;
            }
            2 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let result = param1 * param2;
                self.write_ptr(self.ip+3, result);
                self.ip += argc as i64 + 1;
            }
            3 => {
                let param1 = self.resolve_param(true, self.ip+1);
                self.write(param1, self.input);
                self.ip += argc as i64 + 1;
            }
            4 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                self.outputs.push(param1);
                self.ip += argc as i64 + 1;
            }
            5 => { //jump if true
                let param1 = self.resolve_param(args[0], self.ip+1);
                if param1 == 0 {
                    self.ip += argc as i64 + 1;
                } else {
                    self.ip = self.resolve_param(args[1], self.ip+2);
                }
            }
            6 => { // jump if false
                let param1 = self.resolve_param(args[0], self.ip+1);
                if param1 == 0 {
                    self.ip = self.resolve_param(args[1], self.ip+2);
                } else {
                    self.ip += argc as i64 + 1;
                }
            }
            7 => { // less than
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let param3 = self.ip+3;
                if param1 < param2 {
                    self.write_ptr(param3, 1);
                } else {
                    self.write_ptr(param3, 0);
                }
                self.ip += argc as i64 + 1;
            }
            8 => { // equal
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let param3 = self.ip+3;
                if param1 == param2 {
                    self.write_ptr(param3, 1);
                } else {
                    self.write_ptr(param3, 0);
                }
                self.ip += argc as i64 + 1;
            }
            99 => return false,
            _ => panic!("unrecognised op: {}", op)
        }

        return true;
    }

    fn resolve_param(&self, imm: bool, address: i64) -> i64 {
        //println!("resolve {} ({})", address, imm);
        if imm {self.read(address)} else {self.read_ptr(address)}
    }
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
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
    let mut vm = VM {ip: 0, storage: ints.to_vec(), input: 1, outputs: vec!()};
    vm.run();
    println!("part 1: {:?}", vm.outputs);
}

fn part2(ints: &Vec<i64>) {
    let mut vm = VM {ip: 0, storage: ints.to_vec(), input: 5, outputs: vec!()};
    vm.run();
    println!("part 2: {:?}", vm.outputs);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut vm = VM {ip: 0, storage: vec!(1002,4,3,4,33), input: 1, outputs: vec!()};
        vm.run();
        assert_eq!(vm.storage, vec!(1002,4,3,4,99));
    }

    #[test]
    fn test_op1() {
        let mut vm = VM {ip: 0, storage: vec!(1001,4,2,0,99), input: 1, outputs: vec!()};
        vm.run();
        assert_eq!(vm.storage, vec!(101,4,2,0,99));
    }

    #[test]
    fn test_op2() {
        let mut vm = VM {ip: 0, storage: vec!(1002,4,2,0,99), input: 1, outputs: vec!()};
        vm.run();
        assert_eq!(vm.storage, vec!(198,4,2,0,99));
    }

    #[test]
    fn test_op3() {
        let mut vm = VM {ip: 0, storage: vec!(1002,4,2,0,99), input: 1, outputs: vec!()};
        vm.run();
        assert_eq!(vm.storage, vec!(198,4,2,0,99));
    }

    #[test]
    fn test_position_je() {
        let mut vm = VM {ip: 0, storage: vec!(3,9,8,9,10,9,4,9,99,-1,8), input: 1, outputs: vec!()};
        vm.run();
        assert_eq!(vm.outputs, vec!(0));
    }

    #[test]
    fn test_position_je2() {
        let mut vm = VM {ip: 0, storage: vec!(3,9,8,9,10,9,4,9,99,-1,8), input: 8, outputs: vec!()};
        vm.run();
        assert_eq!(vm.outputs, vec!(1));
    }



}