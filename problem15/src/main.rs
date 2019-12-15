use std::fs::File;
use std::io::{BufReader, BufRead, stdin};
use std::collections::{HashMap, HashSet};
use std::{thread, time};

const DBG: u8 = 0;

type Storage = Vec<i64>;

const SIZE: usize = 50;

struct Game {
    grid: [[char; SIZE]; SIZE],
    location: (usize, usize),
    last_result: i64,
    direction: Direction,
    mode: GameMode
}

enum GameMode {
    Manual,
    FollowLeft
}

impl Game {
    fn new() -> Game {
        println!("creating game");
        let mut game = Game {
            grid: [[' '; SIZE]; SIZE],
            location: (SIZE/2,SIZE/2),
            last_result: -1,
            direction: Direction::Start,
            mode: GameMode::FollowLeft
        };
        game.grid[game.location.1][game.location.0] = '░';
        game
    }

    fn print_map(&self) {
        println!();
        for y in 0..SIZE {
            let mut s: String = self.grid[y].iter().cloned().collect();
            if self.location.0 == y {
                s = s.chars().enumerate()
                    .map(|(i, c)| if i == self.location.1 { '🤖' } else { c })
                    .collect()
            }
            println!("{}",s);
        }
        println!("\n>");
    }

    fn set(&mut self, x: usize, y: usize, result: i64) {
        //println!("setting ({}, {}) for result {}", x, y, result);
        self.grid[y][x] = match result {
            0 => '█',
            1 => '░',
            2 => 'o',
            _ => panic!("painting with char {}", self.grid[x][y])
        }
    }

    fn release_the_oxygen(&mut self) {
        let mut seconds = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut newly_added: HashSet<(usize, usize)> = HashSet::new();
        let mut last_round: HashSet<(usize, usize)> = HashSet::new();

        let oxygen = self.grid.iter().enumerate()
            .flat_map(|(row_ix, row)|
                          row.iter().enumerate()
                              .filter(|(_char_ix, c)| **c == 'o')
                              .map(move |(char_ix, _c)| (char_ix, row_ix)))
            .nth(0).expect("original oxygen missing");

        println!("oxygen location = {:?}", oxygen);

        newly_added.insert(oxygen);

        loop {

            let sleep_duration = time::Duration::from_millis(20);
            thread::sleep(sleep_duration);

            if newly_added.len() == 0 {
                println!("part 2: {}", seconds-1);
                panic!("DONE");
            }
            visited.extend(last_round);
            //println!("visited: {:?}", visited);
            last_round = newly_added;
            newly_added = HashSet::new();

            for location in &last_round {
                for possible in Game::adjacent(&location) {
                    let spread: bool = self.grid[possible.1][possible.0] == '░' && !visited.contains(&possible);
                    if spread {
                        newly_added.insert(possible);
                        self.grid[possible.1][possible.0] = 'o';
                    }
                }
            }

            seconds += 1;

            //println!("{} seconds, newly_added {:?}", seconds, newly_added);
            self.print_map();
        }

    }

    fn adjacent(location: &(usize, usize)) -> Vec<(usize, usize)> {
        vec!(
            (location.0+1, location.1),
            (location.0-1, location.1),
            (location.0, location.1+1),
            (location.0, location.1-1)
        )
    }

    fn read_input(&mut self) -> i64 {
        use Direction::*;

        if self.location == (SIZE/2, SIZE/2) && self.direction == Direction::Down {
            self.release_the_oxygen();
        }

        let attempted_move = (
            match self.direction {
                Left => self.location.1 - 1,
                Right => self.location.1 + 1,
                _ => self.location.1
            },
            match self.direction {
                Down => self.location.0 + 1,
                Up => self.location.0 - 1,
                _ => self.location.0
            }
        );

        println!("tried to move from ({},{}) to ({},{}), got {}",
                 self.location.1, self.location.0,
                 attempted_move.0, attempted_move.1, self.last_result);

        if self.last_result != 0 {
            self.location = (attempted_move.1, attempted_move.0);
        }
        println!("I am at ({}, {})", self.location.1, self.location.0);

        if self.direction != Start {
            self.set(
                attempted_move.0,
                attempted_move.1,
                self.last_result
            )
        }

        self.print_map();

        match self.mode {
            GameMode::Manual => self.manual_input(),
            GameMode::FollowLeft => self.follow_left_input()
        }
    }

    fn manual_input(&mut self) -> i64 {
        use Direction::*;

        loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("input fail");
            match &buffer as &str {
                "e\n" => {
                    self.direction = Up;
                    return 1;
                },
                "d\n" => {
                    self.direction = Down;
                    return 2;
                },
                "s\n" => {
                    self.direction = Left;
                    return 3;
                },
                "f\n" => {
                    self.direction = Right;
                    return 4;
                },
                s => println!("unrecognised {}", s)
            }
        }
    }

    fn follow_left_input(&mut self) -> i64 {
        use Direction::*;

        //let sleep_duration = time::Duration::from_millis(1);
        //thread::sleep(sleep_duration);

        if self.last_result == 0 {
            match self.direction {
                Start | Up => {
                    self.direction = Right;
                    return 4;
                },
                Right => {
                    self.direction = Down;
                    return 2;
                },
                Down => {
                    self.direction = Left;
                    return 3;
                },
                Left => {
                    self.direction = Up;
                    return 1;
                }
            }
        } else {
            match self.direction {
                Start | Up => {
                    self.direction = Left;
                    return 3;
                },
                Left => {
                    self.direction = Down;
                    return 2;
                },
                Down => {
                    self.direction = Right;
                    return 4;
                },
                Right => {
                    self.direction = Up;
                    return 1;
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,Down,Left,Right,Start
}

struct VM {
    ip: i64,
    storage: Storage,
    base: i64,
    more_storage: HashMap<i64, i64>,
    game: Game
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

impl VM {

    fn new(storage: Storage) -> VM {
        println!("creating VM");
        VM {
            ip: 0,
            storage,
            base: 0,
            more_storage: HashMap::new(),
            game: Game::new()
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
                let input = self.game.read_input();
                println!("writing {}", input);
                self.write(address, input);
                self.advance_ip(argc as i64 + 1);
            },
            Instruction::Out { data } => {
                let value = self.resolve_param(&data);
                self.game.last_result = value;
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
    println!("reading input");
    let input = read_input();
    println!("read input");
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
    let mut vm = VM::new(ints.to_vec());
    println!("created VM");
    vm.run();
}