use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i64>) {
    let mut painter = Painter::new(input.to_vec());
    painter.run();

    println!("{}", painter.painted.len());
}

fn part2(input: &Vec<i64>) {
    let mut painter = Painter::new(input.to_vec());
    let (start_x, start_y) = loc_to_array((0, 0));
    painter.paint_array[start_x][start_y] = 1;
    painter.run();
    for row in painter.paint_array.iter() {
        println!("{}",row.iter().map(|byte|(*byte + b'0') as char).collect::<String>());
    }
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

const DBG: u8 = 0;

type Storage = Vec<i64>;

const PAINT_SIZE: usize = 1000;
const HALF_PAINT_SIZE: usize = PAINT_SIZE/2;

struct Painter {
    ip: i64,
    storage: Storage,
    base: i64,
    more_storage: HashMap<i64, i64>,
    location: (i64, i64),
    direction: Direction,
    paint_array: [[u8; PAINT_SIZE]; PAINT_SIZE],
    next_out_paint: bool,
    painted: HashSet<(i64, i64)>
}

fn loc_to_array(loc: (i64, i64)) -> (usize, usize) {
    let array_x = loc.0 + HALF_PAINT_SIZE as i64;
    let array_y = loc.1 + HALF_PAINT_SIZE as i64;
    return (array_x as usize, array_y as usize);
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

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn turn_left(direction: &Direction) -> Direction {
    use Direction::*;
    match direction {
        UP => LEFT,
        RIGHT => UP,
        DOWN => RIGHT,
        LEFT => DOWN
    }
}

fn turn_right(direction: &Direction) -> Direction {
    use Direction::*;
    match direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP
    }
}

impl Painter {

    fn new(storage: Storage) -> Painter {
        Painter {
            ip: 0,
            storage,
            base: 0,
            more_storage: HashMap::new(),
            location: (0i64, 0i64),
            direction: Direction::UP,
            paint_array: [[0; 1000]; 1000],
            next_out_paint: true,
            painted: HashSet::new()
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
                let (x, y) = loc_to_array(self.location);
                let input = match self.paint_array[x][y] {
                    0 => 0,
                    1 => 1,
                    n => panic!("found paint colour {}", n)
                };
                self.write(address, input);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Out { data } => {
                let value = self.resolve_param(&data);

                if self.next_out_paint {
                    let (x, y) = loc_to_array(self.location);
                    self.paint_array[x][y] = match value {
                        0 => 0,
                        1 => 1,
                        _ => panic!("painting {}", value)
                    };
                    self.painted.insert(self.location);
                    self.next_out_paint = false;
                    println!("paint {:?} = ({},{})", self.location, x, y);
                } else {
                    self.direction = match value {
                        0 => turn_left(&self.direction),
                        1 => turn_right(&self.direction),
                        d => panic!("direction input {}", d)
                    };
                    use Direction::*;
                    match self.direction {
                        UP => self.location.1 += 1,
                        RIGHT => self.location.0 += 1,
                        DOWN => self.location.1 -= 1,
                        LEFT => self.location.0 -= 1
                    }
                    self.next_out_paint = true;
                    println!("turned to {:?}", self.direction);
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
