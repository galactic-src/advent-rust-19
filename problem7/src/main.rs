use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use permutohedron::heap_recursive;

type Storage = Vec<i64>;

struct OS {
    files: HashMap<u64, Vec<i64>>
}

impl OS {
    fn new() -> OS {
        OS { files: HashMap::new() }
    }
    fn new_file(&mut self) -> u64 {
        let handle = self.files.len() as u64;
        self.files.insert(handle, vec!());
        handle
    }
    fn write(&mut self, file: u64, value: i64) {
        println!("write {} -> {}: {:?}", value, file, self.files[&file]);
        self.files.entry(file).and_modify(|v|v.insert(0, value));
    }
    fn read(&mut self, file: u64) -> i64 {
        println!("read {}: {:?}", file, self.files[&file]);
        let file_content = &self.files[&file];
        let result = file_content[file_content.len()-1];
        self.files.entry(file).and_modify(|v| {v.pop().expect("out of input");});
        result
    }
    fn log_file(&self, file: u64) {
        println!("{}: {:?}", file, self.files[&file]);
    }
}

pub struct VM {
    ip: i64,
    storage: Storage,
    input_file: u64,
    output_file: u64
}

impl VM {
    fn read(&self, address: i64) -> i64 {
        println!("read {}", address);
        self.storage[address as usize]
    }

    fn read_ptr(&self, address: i64) -> i64 {
        let ptr = self.read(address);
        self.read(ptr)
    }

    fn write(&mut self, address: i64, value: i64) {
        println!("write {} to {}", value, address);
        self.storage[address as usize] = value;
    }

    fn write_ptr(&mut self, address: i64, value: i64) {
        let ptr = self.read(address);
        self.write(ptr, value);
    }

    fn run(&mut self, os: &mut OS) {
        while self.execute_step(os) {}
    }

    fn execute_step(&mut self, os: &mut OS) -> bool {
        let next = self.read(self.ip);
        println!("\tnext@{}: {}", self.ip, next);

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
                if os.files[&self.input_file].len() == 0 {
                    return false;
                }
                let input = os.read(self.input_file);
                self.write(param1, input);
                self.ip += argc as i64 + 1;
            }
            4 => {
                let param1 = self.resolve_param(args[0], self.ip+1);
                os.write(self.output_file, param1);
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

    fn resolve_param(&self, imm: bool, address: i64) -> i64 {
        //println!("resolve {} ({})", address, imm);
        if imm {self.read(address)} else {self.read_ptr(address)}
    }

    fn stopped(&self) -> bool {
        self.storage[self.ip as usize] == 99
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
    let vm_count : u64 = 5;

    let mut phases = vec!();
    for i in 0..(vm_count) {
        phases.push(i as i64);
    }

    let mut max_result = 0;
    heap_recursive(&mut phases,
        |phases| {
            //println!("trying phases: {:?}", phases);
            let mut os = OS::new();
            let first_input = os.new_file();
            let mut next_input = first_input;

            let mut vms: Vec<VM> = (0..vm_count).map(|_i| VM {
                ip: 0,
                storage: ints.to_vec(),
                input_file: 0,
                output_file: os.new_file()
            }).collect();

            for (vm, phase) in vms.iter_mut().zip(phases) {
                os.write(next_input, *phase);
                vm.input_file = next_input;
                next_input = vm.output_file;
            }

            os.write(vms[0].input_file, 0);

            for vm in &mut vms {
                //println!("next vm");
                vm.run(&mut os);
                //println!("vm complete - logging output");
                os.log_file(vm.output_file);
            }

            let last_output_file = vms[vms.len()-1].output_file;
            let last_output = os.read(last_output_file);

            if last_output > max_result {
                max_result = last_output;
            }
        });

    println!("part 1: {:?}", max_result);
}

fn part2(ints: &Vec<i64>) {
    let vm_count : u64 = 5;

    let mut phases_template = vec!();
    for i in 5..(5 + vm_count) {
        phases_template.push(i as i64);
    }

    let mut max_result = 0;
    heap_recursive(&mut phases_template,
       |phases| {
           println!("phases: {:?}", phases);
           let mut os = OS::new();
           let first_input = os.new_file();

           let mut vms: Vec<VM> = (0..vm_count).map(|_i| VM {
               ip: 0,
               storage: ints.to_vec(),
               input_file: 0,
               output_file: os.new_file()
           }).collect();

           let mut next_input = vms[(vm_count-1) as usize].output_file;

           for (vm, phase) in vms.iter_mut().zip(phases) {
               vm.input_file = next_input;
               os.write(vm.input_file, *phase);
               next_input = vm.output_file;
           }

           os.write(vms[0].input_file, 0);

           loop {
               for vm in &mut vms {
                   println!("next vm");
                   vm.run(&mut os);
                   println!("vm complete - logging output");
                   os.log_file(vm.output_file);
               }

               if vms.iter().all(|vm|vm.stopped()) {
                   break;
               }
           }

           let last_output_file = vms[vms.len()-1].output_file;
           let last_output = os.read(last_output_file);

           if last_output > max_result {
               max_result = last_output;
           }
       });

    println!("part 2: {:?}", max_result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut vm = VM {ip: 0, storage: vec!(1002,4,3,4,33), inputs: vec!(1), outputs: vec!()};
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