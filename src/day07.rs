/*
 * Advent of Code 2019 Day 7
 *
 * Amplification Circuit
 *
 * Remarks:
 *  First, I got lazy and downloaded a crate to handle the generation of permutations for me.
 *  Second, what the fuck was Part 2?
 *
 *  I handled the issue of inputs of amplifiers depending on outputs of others by
 *  adding an input queue into each machine, and having the input opcode return an enum
 *  indicating if IntCode machine is waiting for an input if the queue is empty.
 *  This is differentiated from other return values of regular outputs
 *  and an enum indicating the machine halted.
 *
 *  There are probably more elegant methods.
 *  Why didn't I just store these states in the struct for a simple finite state machine?
 *
 *  It'd be cool to make these machines run concurrently.
 *
 *  Also the fact that Rust allows for enums with arbitrary values in it is really cool!
 */

use permutohedron;
use std::collections::VecDeque;

#[derive(Clone)]
struct IntCode {
    ip: usize,
    mem: Vec<i32>,
    input_queue: VecDeque<i32>
}

enum IntCodeStatus {
    Output(i32),
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
    fn get_args(&self) -> Vec<i32> {
        let instruction = self.mem[self.ip];
        let opcode = instruction % 100;
        let mut modes = [(instruction / 100) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10];

        /*
         * Hard coded hack
         *
         * The problem says the parameter for the instruction writes to will never be in immediate mode.
         * But honestly, it makes everything simpler if they were immediate mode,
         * so I flip the mode manually.
         *
         * Rationale:
         *  Let's say we have a sample program `1101,100,-1,4,0` from the problem.
         *  The `01` instruction is writing to address `4`, which is value given immediately!!
         *  If it was in position mode (11101), then we'd be writing to address `0` because
         *  that is the value at position `4`.
         */
        match opcode {
            1 => modes[2] = 1,
            2 => modes[2] = 1,
            3 => modes[0] = 1,
            7 => modes[2] = 1,
            8 => modes[2] = 1,
            _ => (),
        }

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

        let mut args: Vec<i32> = Vec::new();
        for i in 0..num_params {
            let x = self.mem[self.ip + 1 + i];
            args.push(if modes[i] == 0 {self.mem[x as usize]} else { x });
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
                1 => self.mem[args[2] as usize] = args[0] + args[1],
                2 => self.mem[args[2] as usize] = args[0] * args[1],
                3 => {
                    match self.input_queue.pop_front() {
                        Some(input) => self.mem[args[0] as usize] = input,
                        None => return IntCodeStatus::WaitingInput,
                    }
                }
                4 => {
                    self.ip = new_ip;
                    return IntCodeStatus::Output(args[0]);
                }
                5 => if args[0] != 0 { new_ip = args[1] as usize },
                6 => if args[0] == 0 { new_ip = args[1] as usize },
                7 => self.mem[args[2] as usize] = (args[0] < args[1]) as i32,
                8 => self.mem[args[2] as usize] = (args[0] == args[1]) as i32,
                99 => return IntCodeStatus::Halt,
                _ => panic!("Invalid opcode {}", opcode),
            }
            self.ip = new_ip;
        }
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &IntCode) -> i32 {
    let program = input.clone();
    let mut maximum_signal = 0;
    let phases = &mut vec![0, 1, 2, 3, 4];
    let heap = permutohedron::Heap::new(phases);

    for phase_set in heap {
        let mut signal = 0;

        let mut a = program.clone();
        a.input_queue.push_back(phase_set[0]);
        let mut b = program.clone();
        b.input_queue.push_back(phase_set[1]);
        let mut c = program.clone();
        c.input_queue.push_back(phase_set[2]);
        let mut d = program.clone();
        d.input_queue.push_back(phase_set[3]);
        let mut e = program.clone();
        e.input_queue.push_back(phase_set[4]);

        a.input_queue.push_back(signal);
        if let IntCodeStatus::Output(s) = a.execute() { signal = s; }
        else { panic!("Expected to receive output, but did not"); }

        b.input_queue.push_back(signal);
        if let IntCodeStatus::Output(s) = b.execute() { signal = s; }
        else { panic!("Expected to receive output, but did not"); }

        c.input_queue.push_back(signal);
        if let IntCodeStatus::Output(s) = c.execute() { signal = s; }
        else { panic!("Expected to receive output, but did not"); }

        d.input_queue.push_back(signal);
        if let IntCodeStatus::Output(s) = d.execute() { signal = s; }
        else { panic!("Expected to receive output, but did not"); }

        e.input_queue.push_back(signal);
        if let IntCodeStatus::Output(s) = e.execute() { signal = s; }
        else { panic!("Expected to receive output, but did not"); }

        if maximum_signal < signal {
            maximum_signal = signal
        }
    }
    maximum_signal
}

#[aoc(day7, part2)]
fn solve_part2(input: &IntCode) -> i32 {
    let program = input.clone();
    let mut maximum_signal = 0;
    let phases = &mut vec![5, 6, 7, 8, 9];
    let heap = permutohedron::Heap::new(phases);

    for phase_set in heap {
        let mut signal = 0;

        let mut a = program.clone();
        a.input_queue.push_back(phase_set[0]);
        let mut b = program.clone();
        b.input_queue.push_back(phase_set[1]);
        let mut c = program.clone();
        c.input_queue.push_back(phase_set[2]);
        let mut d = program.clone();
        d.input_queue.push_back(phase_set[3]);
        let mut e = program.clone();
        e.input_queue.push_back(phase_set[4]);

        'main: loop {
            a.input_queue.push_back(signal);
            loop {
                if let IntCodeStatus::Output(s) = a.execute() { signal = s }
                else { break; }
            }

            b.input_queue.push_back(signal);
            loop {
                if let IntCodeStatus::Output(s) = b.execute() { signal = s }
                else { break; }
            }

            c.input_queue.push_back(signal);
            loop {
                if let IntCodeStatus::Output(s) = c.execute() { signal = s }
                else { break; }
            }

            d.input_queue.push_back(signal);
            loop {
                if let IntCodeStatus::Output(s) = d.execute() { signal = s }
                else { break; }
            }

            e.input_queue.push_back(signal);
            loop {
                match e.execute() {
                    IntCodeStatus::Output(s) => signal = s,
                    IntCodeStatus::WaitingInput => break,
                    IntCodeStatus::Halt => break 'main, // All done if E halts
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
