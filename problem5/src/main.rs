use std::fs::File;
use std::io::{BufReader, BufRead};

const DBG: u8 = 0;

type Storage = Vec<i64>;

struct VM {
    ip: i64,
    storage: Storage,
    input: i64,
    outputs: Vec<i64>
}

const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_IN: i64 = 3;
const OP_OUT: i64 = 4;
const OP_JNZ: i64 = 5;
const OP_JZ: i64 = 6;
const OP_WLT: i64 = 7;
const OP_WEQ: i64 = 8;
const OP_HALT: i64 = 99;

#[derive(Debug)]
enum Instruction {
    Add { add1:i64, add2: i64, dest: i64 },
    Mul { mul1: i64, mul2: i64, dest: i64 },
    In { dest: i64 },
    Out { data: i64 },
    Jnz { test: i64, abs_target: i64 },
    Jz { test: i64, abs_target: i64 },
    WriteLess { test_a: i64, test_b:  i64, dest: i64 },
    WriteEqual { test_a: i64, test_b: i64, dest: i64 },
    Halt
}

fn argc(op: i64) -> i64 {
    match op {
        OP_HALT => 0,
        OP_IN | OP_OUT => 1,
        OP_JNZ | OP_JZ => 2,
        OP_ADD | OP_MUL | OP_WLT | OP_WEQ => 3,
        _ => panic!("argc for op {}", op)
    }
}

fn argc_i(instruction: &Instruction) -> i64 {
    match instruction {
        Instruction::Add {add1: _, add2: _, dest: _} => argc(OP_ADD),
        Instruction::Mul {mul1: _, mul2: _, dest: _} => argc(OP_MUL),
        Instruction::In {dest: _} => argc(OP_IN),
        Instruction::Out {data: _} => argc(OP_OUT),
        Instruction::Jnz {test: _, abs_target: _} => argc(OP_JNZ),
        Instruction::Jz {test: _, abs_target: _} => argc(OP_JZ),
        Instruction::WriteLess {test_a: _, test_b: _, dest: _} => argc(OP_WLT),
        Instruction::WriteEqual {test_a: _, test_b: _, dest: _} => argc(OP_WEQ),
        Instruction::Halt => argc(OP_HALT),
    }
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
        if DBG >= 2 {
            println!("write {} to {}", value, address);
        }
        self.storage[address as usize] = value;
    }

    //fn write_ptr(&mut self, address: i64, value: i64) {
    //    let ptr = self.read(address);
    //    self.write(ptr, value);
    //}

    fn run(&mut self) {
        if DBG >= 3 {
            println!("{:?}", self.storage);
        }
        while self.step() {}
    }

    fn advance_ip(&mut self, inc: i64) {
        self.ip += inc;
    }

    fn read_next_instruction(&mut self) -> Instruction {
        let next = self.read(self.ip);
        let mut arginfo = next / 100;
        let op = next % 100;

        let mut immediate_args = vec!();

        for _ in 0..argc(op) {
            immediate_args.push(arginfo % 10 == 1);
            arginfo /= 10;
        }

        match op {
            OP_ADD => {
                let add1 = self.resolve_param(immediate_args[0], self.ip+1);
                let add2 = self.resolve_param(immediate_args[1], self.ip+2);
                let dest = self.resolve_param(true, self.ip+3);

                Instruction::Add { add1, add2, dest }
            },
            OP_MUL => {
                let mul1 = self.resolve_param(immediate_args[0], self.ip+1);
                let mul2 = self.resolve_param(immediate_args[1], self.ip+2);
                let dest = self.resolve_param(true, self.ip+3);

                Instruction::Mul { mul1, mul2, dest  }
            },
            OP_IN => {
                let dest = self.resolve_param(true, self.ip+1);

                Instruction::In { dest }
            },
            OP_OUT => {
                let data = self.resolve_param(immediate_args[0], self.ip+1);

                Instruction::Out { data }
            },
            OP_JNZ => {
                let test = self.resolve_param(immediate_args[0], self.ip+1);
                let abs_target = self.resolve_param(immediate_args[1], self.ip+2);

                Instruction::Jnz { test, abs_target }
            },
            OP_JZ => {
                let test = self.resolve_param(immediate_args[0], self.ip+1);
                let abs_target = self.resolve_param(immediate_args[1], self.ip+2);

                Instruction::Jz { test, abs_target }
            },
            OP_WLT => {
                let test_a = self.resolve_param(immediate_args[0], self.ip+1);
                let test_b = self.resolve_param(immediate_args[1], self.ip+2);
                let dest = self.resolve_param(true, self.ip+3);

                Instruction::WriteLess { test_a, test_b, dest }
            },
            OP_WEQ => {
                let test_a = self.resolve_param(immediate_args[0], self.ip+1);
                let test_b = self.resolve_param(immediate_args[1], self.ip+2);
                let dest = self.resolve_param(true, self.ip+3);

                Instruction::WriteEqual { test_a, test_b, dest }
            },
            OP_HALT => {
                Instruction::Halt
            },
            _ => panic!("No instruction for op {}", op)
        }
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        let argc = argc_i(&instruction);

        match instruction {
            Instruction::Add { add1, add2, dest } => {
                let result = add1 + add2;
                self.write(dest, result);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Mul { mul1, mul2, dest } => {
                let result = mul1 * mul2;
                self.write(dest, result);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::In { dest } => {
                self.write(dest, self.input);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Out { data } => {
                self.outputs.push(data);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Jnz { test, abs_target } => {
                if test == 0 {
                    self.advance_ip(argc as i64 + 1);
                } else {
                    self.ip = abs_target;
                }
            },
            Instruction::Jz { test, abs_target } => {
                if test == 0 {
                    self.ip = abs_target;
                } else {
                    self.advance_ip(argc as i64 + 1);
                }
            },
            Instruction::WriteLess { test_a, test_b, dest } => {
                if test_a < test_b {
                    self.write(dest, 1);
                } else {
                    self.write(dest, 0);
                }
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::WriteEqual { test_a, test_b, dest } => {
                if test_a == test_b {
                    self.write(dest, 1);
                } else {
                    self.write(dest, 0);
                }
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Halt => {
                return false;
            }
        }

        true
    }

    fn step(&mut self) -> bool {
        let i = self.read_next_instruction();
        if DBG >= 1 {
            println!("{:?}", i);
        }
        self.execute(i)
    }

    fn resolve_param(&self, imm: bool, address: i64) -> i64 {
        if DBG >= 2 {
            println!("resolve {} ({})", address, imm);
        }
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
    println!("part 1: {:?}", vm.outputs[vm.outputs.len()-1]);
}

fn part2(ints: &Vec<i64>) {
    let mut vm = VM {ip: 0, storage: ints.to_vec(), input: 5, outputs: vec!()};
    vm.run();
    println!("part 2: {:?}", vm.outputs[vm.outputs.len()-1]);
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