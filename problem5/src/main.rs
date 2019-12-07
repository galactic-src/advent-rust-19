use std::fs::File;
use std::io::{BufReader, BufRead};

type Storage = Vec<i64>;

struct VM {
    ip: i64,
    storage: Storage,
    input: i64,
    outputs: Vec<i64>
}

enum Param {
    Src,
    Dest,
    JumpTarget,
    CondWriteTarget
}

struct Ins0Args {
    name: &'static str,
    action: dyn Fn(&mut VM) -> bool
}

struct Ins1Arg {
    name: &'static str,
    params: [Param; 1],
    action: dyn Fn(&mut VM, [i64; 1]) -> bool
}

struct Ins2Args {
    name: &'static str,
    params: [Param; 2],
    action: dyn Fn(&mut VM, [i64; 2]) -> bool
}

struct Ins3Args {
    name: &'static str,
    params: [Param; 3],
    action: dyn Fn(&mut VM, [i64; 3]) -> bool
}

const ADD = Ins3Args { name: "ADD",
    params: [Param::Src, Param::Src, Param::Dest],
    action: |vm: &mut VM, args: [i64, 1]| {
        let result = args[0] + args[1];
        vm.write(args[2], result);
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const MUL = Ins3Args { name: "MUL", params: [Param::Src, Param::Src, Param::Dest],
    action: | vm: & mut VM, args: [i64; 1]| {
        let result = args[0] * args[1];
        vm.write(args[2], result);
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const IN = Ins1Arg { name: "IN", params: [Param::Dest],
    action: | vm: & mut VM, args: Vec<i64>| {
        vm.write(args[0], vm.input);
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const OUT = Ins1Arg { name: "OUT", params: vec!(Param::Src),
    action: | vm: & mut VM, args: Vec<i64>| {
        vm.outputs.push(args[0]);
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const JNZ: InstructionType = InstructionType { name: "JNZ", params: vec!(Param::Src, Param::JumpTarget),
    action: | vm: & mut VM, args: Vec<i64>| {
        if args[0] != 0 {
            vm.ip = args[1];
        } else {
            vm.ip += args.len() as i64 + 1;
        }
        true
    }
};
const JZ: InstructionType = InstructionType { name: "JZ", params: vec!(Param::Src, Param::JumpTarget),
    action: | vm: & mut VM, args: Vec<i64>| {
        if args[0] == 0 {
            vm.ip = args[1];
        } else {
            vm.ip += args.len() as i64 + 1;
        }
        true
    }
};
const W_LT: InstructionType = InstructionType { name: "W_LT", params: vec!(Param::Src, Param::Src, Param::CondWriteTarget),
    action: | vm: & mut VM, args: Vec<i64>| {
        if args[0] < args[1] {
            vm.write(args[2], 1);
        } else {
            vm.write(args[2], 0);
        }
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const W_EQ: InstructionType = InstructionType { name: "W_EQ", params: vec!(Param::Src, Param::Src, Param::CondWriteTarget),
    action: | vm: & mut VM, args: Vec<i64>| {
        if args[0] == args[1] {
            vm.write(args[2], 1);
        } else {
            vm.write(args[2], 0);
        }
        vm.ip += args.len() as i64 + 1;
        true
    }
};
const HALT: InstructionType = InstructionType { name: "HALT", params: vec!(),
    action: | vm: & mut VM, args: Vec<i64>| {
        false
    }
};

struct Instruction {
    i_type: InstructionType,
    args: Vec<i64>
}

fn instruction_type(op: i64) -> &'static InstructionType {
    match op {
        1 => &ADD,
        2 => &MUL,
        3 => &IN,
        4 => &OUT,
        5 => &JNZ,
        6 => &JZ,
        7 => &W_LT,
        8 => &W_EQ,
        99 => &HALT,
        _ => panic!("Unrecognised op: {}", op)
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
                let param3 = self.resolve_param(true, self.ip+3);
                let result = param1 + param2;
                self.write(param3, result);
                self.ip += argc as i64 + 1;
            }
            2 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let param3 = self.resolve_param(true, self.ip+3);

                let result = param1 * param2;
                self.write(param3, result);
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
                let param2 = self.resolve_param(args[1], self.ip+2);

                if param1 == 0 {
                    self.ip += argc as i64 + 1;
                } else {
                    self.ip = param2;
                }
            }
            6 => { // jump if false
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);

                if param1 == 0 {
                    self.ip = param2;
                } else {
                    self.ip += argc as i64 + 1;
                }
            }
            7 => { // less than
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let param3 = self.resolve_param(true, self.ip+3);

                if param1 < param2 {
                    self.write(param3, 1);
                } else {
                    self.write(param3, 0);
                }
                self.ip += argc as i64 + 1;
            }
            8 => { // equal
                let param1 = self.resolve_param(args[0], self.ip+1);
                let param2 = self.resolve_param(args[1], self.ip+2);
                let param3 = self.resolve_param(true, self.ip+3);

                if param1 == param2 {
                    self.write(param3, 1);
                } else {
                    self.write(param3, 0);
                }
                self.ip += argc as i64 + 1;
            }
            99 => return false,
            _ => panic!("unrecognised op: {}", op)
        }

        return true;
    }

    fn instruction(&self) -> Instruction {
        let op = self.read(self.ip);

        let mut arg_bits = op / 100;
        let op = op % 100;
        let i_type = instruction_type(op);

        let arg_info: Vec<bool> = vec!();
        for _ in 1..(i_type.params.len()+1) {
            arg_info.push(arg_bits % 10 == 1);
            arg_bits /= 10;
        }

        let args: Vec<i64> = i_type.params().iter().enumerate().map(|(param, i)|
            match param {
                Param::Src => self.resolve_param(arg_info[i], i),
                Param::Dest => self.resolve_param(true, i),
                Param::CondWriteTarget => self.resolve_param(true, i),
                Param::JumpTarget => self.resolve_param(arg_info[i], i)
            }).collect();

        Instruction {
            i_type,
            args
        }
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