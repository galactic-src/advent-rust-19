use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const DBG: u8 = 0;

type Storage = Vec<i64>;

#[derive(Debug)]
enum TractorState {
    FindStart, FindEnd
}

#[derive(Debug)]
enum SearchDirection {
    UNKNOWN, RIGHT, LEFT
}

#[derive(Debug)]
enum NextCoord {
    X,Y
}

struct Tractor {
    state: TractorState,
    direction: SearchDirection,
    next: NextCoord,
    current_y: usize,
    current_x: usize,
    row_start: usize,
    beam_ranges: Vec<(usize,usize)>,
    beam_grid:Vec<Vec<i64>>
}

impl Tractor {
    fn new() -> Tractor {
        Tractor {
            state: TractorState::FindStart,
            direction: SearchDirection::UNKNOWN,
            next: NextCoord::X,
            current_y: 0,
            current_x: 0,
            row_start: 0,
            beam_ranges: vec!(),
            beam_grid: vec!(vec!())
        }
    }

    fn store_line(&mut self, row_end: usize) {
        println!("storing line ({}-{})", self.row_start, row_end);
        self.beam_ranges.push((self.row_start, row_end));
    }

    fn next_line(&mut self) {
        self.current_x = self.beam_ranges[self.beam_ranges.len()-1].0;
        self.current_y += 1;
        self.state = TractorState::FindStart;
        self.direction = SearchDirection::UNKNOWN;
    }

    fn found_row_start(&mut self, row_start: usize) {
        //println!("found row start");
        self.row_start = row_start;
        self.state = TractorState::FindEnd;
        self.direction = SearchDirection::UNKNOWN;
        self.current_x = if self.current_y == 0 { 0 } else { self.beam_ranges[self.beam_ranges.len() - 1].1 };
    }

    fn read(&mut self) -> i64 {
        match self.next {
            NextCoord::X => {
                self.next = NextCoord::Y;
                //println!("read x {}", self.current_x);
                self.current_x as i64
            },
            NextCoord::Y => {
                self.next = NextCoord::X;
//                println!("read y {}", self.current_y);
                self.current_y as i64
            }
        }
    }

    fn write(&mut self, output: i64) {
        use TractorState::*;
        use SearchDirection::*;

//        println!("output {}", output);

        let in_beam =
            match output {
                0 => false,
                a if a > 0 => true,
                _ => panic!("negative object in the beam area: {}", output)
            };

        println!("I'm at {},{} in_beam={}", self.current_x, self.current_y, in_beam);
        println!("Looking for {:?} to the {:?} ", self.state, self.direction);

        match self.state {
            FindStart =>
                match self.direction {
                    UNKNOWN => {
                        if in_beam {
                            if self.current_x == 0 {
                                self.found_row_start(0);
                            } else {
                                self.direction = LEFT;
                                self.current_x -= 1;
                            }
                        } else {
                            self.direction = RIGHT;
                            self.current_x += 1;
                        }
                    },
                    LEFT =>
                        if self.current_x == 0 {
                            self.found_row_start(0);
                        } else if in_beam {
                            self.current_x -= 1;
                        } else {
                            self.found_row_start(self.current_x + 1);
                        },
                    RIGHT =>
                        if in_beam {
                            self.found_row_start(self.current_x);
                        } else if self.current_x == 49 {
                            self.next_line();
                        } else {
                            self.current_x += 1;
                        }
                },
            FindEnd =>
                match self.direction {
                    UNKNOWN => {
                        if self.current_x == 49 {
                            self.store_line(self.current_x);
                            self.next_line();
                        } else if in_beam {
                            self.direction = RIGHT;
                            self.current_x += 1;
                        } else {
                            self.direction = LEFT;
                            self.current_x -= 1;
                        }
                    },
                    LEFT => if in_beam {
                        self.store_line(self.current_x);
                        self.next_line();
                    } else {
                        self.current_x -= 1;
                    },
                    RIGHT => if in_beam {
                        if self.current_x == 49 {
                            self.store_line(self.current_x);
                            self.next_line();
                        } else {
                            self.current_x += 1;
                        }
                    } else {
                        self.store_line(self.current_x - 1);
                        self.next_line();
                    }
                }
        }
    }

    fn write2(&mut self, output: i64) {
        self.beam_grid[self.current_y].push(output);
        self.current_x += 1;

        if self.current_x == 50 {
            self.current_y += 1;
            self.current_x = 0;
            self.beam_grid.push(vec!());
        }
    }
}

struct VM<'a> {
    ip: i64,
    storage: Storage,
    base: i64,
    more_storage: HashMap<i64, i64>,
    tractor: &'a mut Tractor
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

impl <'a> VM<'a> {

    fn new(storage: Storage, tractor: &'a mut Tractor) -> VM<'a> {
        VM {
            ip: 0,
            storage,
            base: 0,
            more_storage: HashMap::new(),
            tractor
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
//                println!("op in");
                let address = self.resolve_param_w(&dest);
                let input = self.tractor.read();
//                println!("tractor outputted {}", input);
                self.write(address, input);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Out { data } => {
//                println!("op out");
                let value = self.resolve_param(&data);
                self.tractor.write2(value);
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
    //part2(&input);
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
//    let mut vm = VM::new(ints.to_vec());
//    loop {
////        println!("kicking off the VM");
//        vm.ip = 0;
//        vm.storage = ints.to_vec();
//        vm.run();
//
//        if vm.tractor.beam_ranges.len() == 50 {
//            break;
//        }
//    }
//    for range in &vm.tractor.beam_ranges {
//        println!("{:?}", range);
//    }
//    println!("part 1: {:?}", vm.tractor.beam_ranges.iter()
//        .map(|(first, last)| last + 1 - first).sum::<usize>());

    let mut tractor = Tractor::new();

    for _ in 0..50 {
        for _ in 0..50 {
            let mut vm = VM::new(ints.to_vec(), &mut tractor);
            vm.run();
        }
    }

    for line in &tractor.beam_grid {
        println!("{}", line.iter().map(|i|if *i == 0 {'.'} else {'@'}).collect::<String>());
    }

    println!("{} x {}", tractor.beam_grid.len(), tractor.beam_grid[0].len());

    let total = &tractor.beam_grid.iter().map(|row| row.iter().filter(|n|**n > 0).count()).sum::<usize>();
    println!("part 1: {}", total);
}

//fn part2(ints: &Vec<i64>) {
//    let mut vm = VM::new(ints.to_vec(), 2);
//    vm.run();
//    println!("part 2: {:?}", );
//}