use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_IN: i64 = 3;
const OP_OUT: i64 = 4;
const OP_JNZ: i64 = 5;
const OP_JZ: i64 = 6;
const OP_WLT: i64 = 7;
const OP_WEQ: i64 = 8;
const OP_BASE: i64 = 9;
const OP_HALT: i64 = 99;

#[derive(Debug)]
enum Instruction {
    Add { add1: Arg, add2: Arg, dest: Arg },
    Mul { mul1: Arg, mul2: Arg, dest: Arg },
    In { dest: Arg },
    Out { data: Arg },
    Jnz { test: Arg, abs_target: Arg },
    Jz { test: Arg, abs_target: Arg },
    WriteLess { test_a: Arg, test_b:  Arg, dest: Arg },
    WriteEqual { test_a: Arg, test_b: Arg, dest: Arg },
    SetBase { base: Arg },
    Halt
}

fn argc(op: i64) -> i64 {
    match op {
        OP_HALT => 0,
        OP_IN | OP_OUT | OP_BASE => 1,
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
        Instruction::SetBase { base: _ } => argc(OP_BASE),
        Instruction::Halt => argc(OP_HALT)
    }
}

const DBG: u8 = 0;

type Storage = Vec<i64>;

struct VM {
    ip: i64,
    storage: Storage,
    input: i64,
    outputs: Vec<i64>,
    base: i64,
    more_storage: HashMap<i64, i64>,
    game: Pong
}

#[derive(Debug)]
struct Arg {
    value: i64,
    mode: Mode
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Normal,
    Imm,
    Base
}

struct Pong {
    display: Vec<Vec<char>>,
    score: i64,
    ball: (usize, usize),
    ball_direction: (i16, i16)
}

impl Pong {
    fn new() -> Pong {
        Pong {
            display: vec![vec![' '; 37]; 26],
            score: 0,
            ball: (0, 0),
            ball_direction: (0, 0)
        }
    }

    fn paint(&mut self, data: &Vec<i64>) {
        let x = data[0] as usize;
        let y = data[1] as usize;

        self.display[y][x] = match data[2] {
            0 => ' ',
            1 => 'X',
            2 => '#',
            3 => '_',
            4 => 'o',
            x => panic!("Unexpected paint value {}", x)
        };

        if data[2] == 4 {
            self.ball_direction = (x as i16 - self.ball.0 as i16 , y as i16 - self.ball.1 as i16 );
            self.ball = (x, y);
        }
    }
}

impl VM {

    fn new(storage: Storage, input: i64) -> VM {
        VM {
            ip: 0,
            storage,
            input,
            outputs: vec!(),
            base: 0,
            more_storage: HashMap::new(),
            game: Pong::new()
        }
    }

    fn read(&mut self, address: i64) -> i64 {
        let result = if address < self.storage.len() as i64 {
            self.storage[address as usize]
        } else if self.more_storage.contains_key(&address) {
            self.more_storage[&address]
        } else {
            0
        };

        if DBG >= 2 {
            println!("read {} from {}", result, address);
        }

        result
    }

    //fn read_ptr(&mut self, address: i64) -> i64 {
    //    let ptr = self.read(address);
    //    self.read(ptr)
    //}

    fn write(&mut self, address: i64, value: i64) {
        if DBG >= 2 {
            println!("write {} to {}", value, address);
        }

        let use_more_storage = address >= self.storage.len() as i64;

        if use_more_storage {
            self.more_storage.insert(address, value);
        } else {
            self.storage[address as usize] = value;
        }
    }

    //fn write_ptr(&mut self, address: i64, value: i64) {
    //    let ptr = self.read(address);
    //    self.write(ptr, value);
    //}

    fn run(&mut self) {
        if DBG >= 1 {
            println!("{:?}", self.storage);
        }
        while self.step() {}
    }

    fn advance_ip(&mut self, inc: i64) {
        if DBG >=2 {
            println!("ip->{}+{}", self.ip, inc);
        }
        self.ip += inc;
    }

    fn get_op(&mut self) -> (i64, i64){
        let next = self.read(self.ip);
        let arginfo = next / 100;
        let op = next % 100;

        return (op, arginfo)
    }

    fn read_next_instruction(&mut self, op: i64, arg_modes: Vec<Mode>) -> Instruction {

        match op {
            OP_ADD => {
                let add1 = Arg { mode: arg_modes[0], value: self.read(self.ip+1) };
                let add2 = Arg { mode: arg_modes[1], value: self.read(self.ip+2) };
                //let dest = Arg { mode: Mode::Imm, value: self.read(self.ip+3)};
                let dest = Arg { mode: arg_modes[2], value: self.read(self.ip+3)};

                Instruction::Add { add1, add2, dest }
            },
            OP_MUL => {
                let mul1 = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                let mul2 = Arg { mode: arg_modes[1], value: self.read(self.ip+2)};
                //let dest = Arg { mode: Mode::Imm, value: self.read(self.ip+3)};
                let dest = Arg { mode: arg_modes[2], value: self.read(self.ip+3)};

                Instruction::Mul { mul1, mul2, dest  }
            },
            OP_IN => {
                let dest = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};

                Instruction::In { dest }
            },
            OP_OUT => {
                let data = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};

                Instruction::Out { data }
            },
            OP_JNZ => {
                let test = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                let abs_target = Arg { mode: arg_modes[1], value: self.read(self.ip+2)};

                Instruction::Jnz { test, abs_target }
            },
            OP_JZ => {
                let test = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                let abs_target = Arg { mode: arg_modes[1], value: self.read(self.ip+2)};

                Instruction::Jz { test, abs_target }
            },
            OP_WLT => {
                let test_a = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                let test_b = Arg { mode: arg_modes[1], value: self.read(self.ip+2)};
                let dest = Arg { mode: arg_modes[2], value: self.read(self.ip+3)};
                //let dest = Arg { mode: Mode::Imm, value: self.read(self.ip+3)};

                Instruction::WriteLess { test_a, test_b, dest }
            },
            OP_WEQ => {
                let test_a = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                let test_b = Arg { mode: arg_modes[1], value: self.read(self.ip+2)};
                let dest = Arg { mode: arg_modes[2], value: self.read(self.ip+3)};
                //let dest = Arg { mode: Mode::Imm, value: self.read(self.ip+3)};

                Instruction::WriteEqual { test_a, test_b, dest }
            },
            OP_BASE => {
                let base = Arg { mode: arg_modes[0], value: self.read(self.ip+1)};
                //let base = Arg { mode: Mode::Imm, value: self.read(self.ip+1)};

                Instruction::SetBase { base }
            }
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
                let result = self.resolve_param(&add1) + self.resolve_param(&add2);
                let address = self.resolve_param_w(&dest);
                self.write(address, result);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Mul { mul1, mul2, dest } => {
                let result = self.resolve_param(&mul1) * self.resolve_param(&mul2);
                let address = self.resolve_param_w(&dest);
                self.write(address, result);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::In { dest } => {
                let address = self.resolve_param_w(&dest);
                self.write(address, self.input);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Out { data } => {
                let value = self.resolve_param(&data);
                self.outputs.push(value);
                if self.outputs.len() == 3 {
                    self.game.paint(&self.outputs);
                    if self.outputs[2] == 4 {
                        println!("storage for ball draw: {:?}", self.storage);
                    }
                    self.outputs.clear();
                }
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Jnz { test, abs_target } => {
                if DBG >= 1 {
                    println!("JNZ...({:?} = {}?) {:?} = {}", test, self.resolve_param(&test), abs_target, self.resolve_param(&abs_target));
                }
                let result = self.resolve_param(&test) == 0;
                if result {
                    self.advance_ip(argc as i64 + 1);
                } else {
                    self.ip = self.resolve_param(&abs_target);
                }
            },
            Instruction::Jz { test, abs_target } => {
                let result = self.resolve_param(&test) == 0;
                if result {
                    self.ip = self.resolve_param(&abs_target);
                } else {
                    self.advance_ip(argc as i64 + 1);
                }
            },
            Instruction::WriteLess { test_a, test_b, dest } => {
                let a = self.resolve_param(&test_a);
                let b = self.resolve_param(&test_b);
                let address = self.resolve_param_w(&dest);
                if a < b {
                    self.write(address, 1);
                } else {
                    self.write(address, 0);
                }
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::WriteEqual { test_a, test_b, dest } => {
                let a = self.resolve_param(&test_a);
                let b = self.resolve_param(&test_b);
                let address = self.resolve_param_w(&dest);
                if a == b {
                    self.write(address, 1);
                } else {
                    self.write(address, 0);
                }
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::SetBase { base } => {
                self.base += self.resolve_param(&base);
                self.advance_ip(argc as i64 + 1)
            },
            Instruction::Halt => {
                return false;
            }
        }

        true
    }

    fn step(&mut self) -> bool {
        let (op, mut arginfo) = self.get_op();

        let mut arg_modes = vec!();

        for _ in 0..argc(op) {
            arg_modes.push(match arginfo % 10 {
                0 => Mode::Normal,
                1 => Mode::Imm,
                2 => Mode::Base,
                m => panic!("unrecognised mode {}", m)
            });
            arginfo /= 10;
        }

        let i = self.read_next_instruction(op, arg_modes);
        if DBG >= 1 {
            println!("{:?}", i);
        }
        self.execute(i)
    }

    fn resolve_param(&mut self, arg: &Arg) -> i64 {
        //if DBG >= 2 {
        //    println!("resolve {:?}", arg);
        //}

        match arg.mode {
            Mode::Normal => self.read(arg.value),
            Mode::Imm => arg.value,
            Mode::Base => self.read(self.base + arg.value)
        }
    }

    fn resolve_param_w(&mut self, arg: &Arg) -> i64 {
        //if DBG >= 2 {
        //    println!("resolve {:?}", arg);
        //}

        match arg.mode {
            Mode::Normal => arg.value,
            Mode::Imm => panic!("writing immediate?"),
            Mode::Base => self.base + arg.value
        }
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
    let mut vm = VM::new(ints.to_vec(), 1);
    vm.run();
    //println!("part 1: {:?}", vm.outputs);
    let paints: Vec<&[i64]> = vm.outputs.chunks(3).collect();
    println!("{:?}",paints);
    let x_min = paints.iter().map(|triple|triple[0]).min().expect("no x_min") as usize;
    let x_max = paints.iter().map(|triple|triple[0]).max().expect("no x_max") as usize;
    let y_min = paints.iter().map(|triple|triple[1]).min().expect("no y_min") as usize;
    let y_max = paints.iter().map(|triple|triple[1]).max().expect("no y_max") as usize;
//    println!("({}-{},{}-{})", x_min, x_max, y_min, y_max);
    if x_min != 0 {panic!("x_min non-zero: {}", x_min)}
    if y_min != 0 {panic!("y_min non-zero: {}", y_min)}
    let mut display = vec![vec![' '; x_max+1]; y_max+1];

    for paint in paints {
        let x = paint[0] as usize;
        let y = paint[1] as usize;
        {
            display[y][x] = match paint[2] {
                0 => ' ',
                1 => 'X',
                2 => '#',
                3 => '_',
                4 => 'o',
                x => panic!("Unexpected paint value {}", x)
            };
        }

//        let mut printout = String::from("\n\n\n");
//        for row in &display {
//            for c in row {
//                printout.push(*c);
//            }
//            printout.push('\n');
//        }
//        println!("{}", printout);
    }

    let block_count = display.iter().flat_map(|row| row.iter()).filter(|c| **c == '#').count();

    println!("part 1: {}", block_count);
}

fn part2(ints: &Vec<i64>) {
    let mut vm = VM::new(ints.to_vec(), 2);
    vm.write(0, 2);
    vm.run();
}
