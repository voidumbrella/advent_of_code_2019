/*
 * Advent of Code 2019 Day 9
 *
 * Sensor Boost
 *
 * Remarks:
 *  Another IntCode problem??
 *  At least it made me refactor some stuff.
 */

use std::collections::VecDeque;

#[derive(Clone)]
struct IntCode {
    ip: usize,
    relative_base: i64,
    mem: Vec<i64>,
    input_queue: VecDeque<i64>
}

enum IntCodeStatus {
    Output(i64),
    Halt,
    WaitingInput,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> IntCode {
    let mut i = IntCode {
        ip: 0,
        relative_base: 0,
        mem: input
            .split(',')
            .map(|n| { n.parse().unwrap() })
            .collect(),
        input_queue: VecDeque::new(),
    };
    i.mem.resize(5_000, 0);
    i
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
            4 => 1, // output {1}
            5 => 2, // jump to {2} if {1} != 0
            6 => 2, // jump to {2} if {1} == 0
            7 => 3, // {3} = ({1} < {2})
            8 => 3, // {3} = ({1} == {2})
            9 => 1, // relative_base = {1}
            99 => 0, // halt
            _ => panic!("Invalid opcode {}", opcode),
        };

        let mut args: Vec<usize> = Vec::new();
        for (i, mode) in modes.iter().enumerate().take(num_params) {
            let x = self.mem[self.ip + 1 + i];
            args.push(match mode {
                0 => x as usize, // Address mode
                1 => self.ip + 1 + i, // Immediate mode
                2 => (self.relative_base + x) as usize, // Relative mode
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
                9 => self.relative_base += self.mem[args[0]],
                99 => return IntCodeStatus::Halt,
                _ => panic!("Invalid opcode {}", opcode),
            }
            self.ip = new_ip;
        }
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &IntCode) -> String {
    /*
     * Return a Str representing a vector, because if our machine is bugged
     * it outputs multiple values and we need to know that.
     */
    let mut program = input.clone();

    let mut outputs: Vec<i64> = Vec::new();
    program.input_queue.push_back(1);
    loop {
        match program.execute() {
            IntCodeStatus::Output(n) => outputs.push(n),
            IntCodeStatus::Halt => break,
            IntCodeStatus::WaitingInput => panic!("This should not be waiting for input"),
        }
    }
    format!("{:?}", outputs)
}

#[aoc(day9, part2)]
fn solve_part2(input: &IntCode) -> i64 {
    let mut program = input.clone();

    program.input_queue.push_back(2);
    match program.execute() {
        IntCodeStatus::Output(n) => n,
        IntCodeStatus::Halt => panic!("Premature halt"),
        IntCodeStatus::WaitingInput => panic!("This should not be waiting for input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let s = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
            let mut program = parse(s);
            let expected: Vec<i64> = s
                .split(',')
                .map(|n| { n.parse().unwrap() })
                .collect();

            let mut i = 0;
            loop {
                if let IntCodeStatus::Output(n) = program.execute() {
                    assert_eq!(expected[i], n);
                    i += 1;
                }
                else { break } 
            }
        }
        {
            let mut program = parse("104,1125899906842624,99");
            if let IntCodeStatus::Output(s) = program.execute() {
                assert_eq!(s, 1125899906842624);
            }
            else { panic!("Expected to receive output!") } 
        }
        {
            let mut program = parse("1102,34915192,34915192,7,4,7,99,0");
            if let IntCodeStatus::Output(s) = program.execute() {
                assert_eq!(s.to_string().len(), 16);
            }
            else { panic!("Expected to receive output!") } 
        }
    }
}
