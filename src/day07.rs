/*
 * Advent of Code 2019 Day 7
 *
 * Amplification Circuit
 *
 * Remarks:
 *  First, I got lazy and downloaded a crate to handle the generation of permutations for me.
 *  Second, what the heck was Part 2?
 *
 *  I handled the issue of inputs of amplifiers depending on outputs of others by
 *  having execute() return the state of the IntCode machine or its output.
 *  There are probably more elegant methods.
 *
 *  It'd be nice to make these machines run concurrently?
 *  Also the fact that Rust allows for enums with arbitrary values in it is really cool!
 */

use permutohedron;
use std::collections::VecDeque;

#[derive(Clone)]
struct IntCode {
    ip: usize,
    mem: Vec<i64>,
    input_queue: VecDeque<i64>
}

enum IntCodeStatus {
    Output(i64),
    Halt,
    WaitingInput,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> IntCode {
    IntCode {
        ip: 0,
        mem: input
            .split(',')
            .map(|n| { n.parse().unwrap() })
            .collect(),
        input_queue: VecDeque::new(),
    }
}

impl IntCode {

    /*
     * Returns a Vector containing indicies for the arguments of the current opcode.
     */
    fn get_args(&self) -> Vec<usize> {
        let instruction = self.mem[self.ip];
        let opcode = instruction % 100;
        let modes = [(instruction / 100) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10];

        let num_params = match opcode {
            1 => 3, // {3} = {1} + {2}
            2 => 3, // {3} = {1} * {2}
            3 => 1, // {1} = input
            4 => 1, // output {4}
            5 => 2, // jump to {2} if {1} != 0
            6 => 2, // jump to {2} if {1} == 0
            7 => 3, // {3} = ({1} < {2})
            8 => 3, // {3} = ({1} == {2})
            99 => 0, // halt
            _ => panic!("Invalid opcode {}", opcode),
        };

        let mut args: Vec<usize> = Vec::new();
        for i in 0..num_params {
            let x = self.mem[self.ip + 1 + i];
            args.push(match modes[i] {
                0 => x as usize, // Address mode
                1 => self.ip + 1 + i, // Immediate mode
                x => panic!("Unknown parameter mode {}", x),
            });
        }
        args
    }

    pub fn execute(&mut self) -> IntCodeStatus {
        loop {
            let args = self.get_args();
            let instruction = self.mem[self.ip];
            let opcode = instruction % 100;

            let mut new_ip = self.ip + args.len() + 1;
            match opcode {
                1 => self.mem[args[2]] = self.mem[args[0]] + self.mem[args[1]],
                2 => self.mem[args[2]] = self.mem[args[0]] * self.mem[args[1]],
                3 => {
                    match self.input_queue.pop_front() {
                        Some(input) => self.mem[args[0]] = input,
                        None => return IntCodeStatus::WaitingInput,
                    }
                }
                4 => {
                    self.ip = new_ip;
                    return IntCodeStatus::Output(self.mem[args[0]]);
                }
                5 => if self.mem[args[0]] != 0 { new_ip = self.mem[args[1]] as usize },
                6 => if self.mem[args[0]] == 0 { new_ip = self.mem[args[1]] as usize },
                7 => self.mem[args[2]] = (self.mem[args[0]] < self.mem[args[1]]) as i64,
                8 => self.mem[args[2]] = (self.mem[args[0]] == self.mem[args[1]]) as i64,
                99 => return IntCodeStatus::Halt,
                _ => panic!("Invalid opcode {}", opcode),
            }
            self.ip = new_ip;
        }
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &IntCode) -> i64 {
    let program = input.clone();
    let mut maximum_signal = 0;
    let phases = &mut vec![0, 1, 2, 3, 4];
    let heap = permutohedron::Heap::new(phases);

    for phase_set in heap {
        let mut signal = 0;

        let mut programs = [
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
        ];

        for i in 0..5 {
            programs[i].input_queue.push_back(phase_set[i]);
        }

        for i in 0..5 {
            programs[i].input_queue.push_back(signal);
            if let IntCodeStatus::Output(s) = programs[i].execute() { signal = s; }
            else { panic!("Expected to receive output, but did not"); }
        }

        if maximum_signal < signal {
            maximum_signal = signal
        }
    }
    maximum_signal
}

#[aoc(day7, part2)]
fn solve_part2(input: &IntCode) -> i64 {
    let program = input.clone();
    let mut maximum_signal = 0;
    let phases = &mut vec![5, 6, 7, 8, 9];
    let heap = permutohedron::Heap::new(phases);

    for phase_set in heap {
        let mut signal = 0;

        let mut programs = [
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
        ];

        for i in 0..5 {
            programs[i].input_queue.push_back(phase_set[i]);
        }

        /*
         * Each machine outputs a number and immediately passes control to the next.
         * This ensures no machine will be stuck waiting for an input.
         */
        'main: loop {
            for i in 0..5 {
                programs[i].input_queue.push_back(signal);
                match programs[i].execute() {
                    IntCodeStatus::Output(s) => { signal = s; }
                    IntCodeStatus::WaitingInput => panic!("Deadlock reached"),
                    IntCodeStatus::Halt => {
                        // Feedback loop is over if the last machine halts
                        if i == 4 { break 'main; }
                    } 
                }
            }
        }

        if maximum_signal < signal {
            maximum_signal = signal
        }
    }

    maximum_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let program = parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
            assert_eq!(solve_part1(&program), 43210);
        }
        {
            let program = parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
            assert_eq!(solve_part1(&program), 54321);
        }
        {
            let program = parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
            assert_eq!(solve_part1(&program), 65210);
        }
    }

    #[test]
    fn part2() {
        {
            let program = parse("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
            assert_eq!(solve_part2(&program), 18216);
        }
        {
            let program = parse("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
            assert_eq!(solve_part2(&program), 139629729);
        }
    }
}
