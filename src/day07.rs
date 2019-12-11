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
 *  having execute() return the state of the intcode::IntCode machine or its output.
 *  There are probably more elegant methods.
 *
 *  It'd be nice to make these machines run concurrently?
 *  Also the fact that Rust allows for enums with arbitrary values in it is really cool!
 */

use permutohedron;
use std::collections::VecDeque;

use crate::intcode;

#[aoc_generator(day7)]
fn parse(input: &str) -> intcode::IntCode {
    intcode::IntCode {
        ip: 0,
        mem: input
            .split(',')
            .map(|n| { n.parse().unwrap() })
            .collect(),
        input_queue: VecDeque::new(),
        relative_base: 0,
    }
}

#[aoc(day7, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
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

        for (i, program) in programs.iter_mut().enumerate() {
            program.input_queue.push_back(phase_set[i]);
        }

        for program in &mut programs {
            program.input_queue.push_back(signal);
            if let intcode::Status::Output(s) = program.execute() { signal = s; }
            else { panic!("Expected to receive output, but did not"); }
        }

        if maximum_signal < signal {
            maximum_signal = signal
        }
    }
    maximum_signal
}

#[aoc(day7, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
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
            for (i, program) in programs.iter_mut().enumerate() {
                program.input_queue.push_back(signal);
                match program.execute() {
                    intcode::Status::Output(s) => { signal = s; }
                    intcode::Status::WaitingInput => panic!("Deadlock reached"),
                    intcode::Status::Halt => {
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
