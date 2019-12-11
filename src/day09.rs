/*
 * Advent of Code 2019 Day 9
 *
 * Sensor Boost
 *
 * Remarks:
 *  Another intcode::IntCode problem??
 *  At least it made me refactor some stuff.
 */

use std::collections::VecDeque;

use crate::intcode;

#[aoc_generator(day9)]
fn parse(input: &str) -> intcode::IntCode {
    let mut i = intcode::IntCode {
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

#[aoc(day9, part1)]
fn solve_part1(input: &intcode::IntCode) -> String {
    /*
     * Return a Str representing a vector, because if our machine is bugged
     * it outputs multiple values and we need to know that.
     */
    let mut program = input.clone();

    let mut outputs: Vec<i64> = Vec::new();
    program.input_queue.push_back(1);
    loop {
        match program.execute() {
            intcode::Status::Output(n) => outputs.push(n),
            intcode::Status::Halt => break,
            intcode::Status::WaitingInput => panic!("This should not be waiting for input"),
        }
    }
    format!("{:?}", outputs)
}

#[aoc(day9, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    let mut program = input.clone();

    program.input_queue.push_back(2);
    match program.execute() {
        intcode::Status::Output(n) => n,
        intcode::Status::Halt => panic!("Premature halt"),
        intcode::Status::WaitingInput => panic!("This should not be waiting for input"),
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
                if let intcode::Status::Output(n) = program.execute() {
                    assert_eq!(expected[i], n);
                    i += 1;
                }
                else { break } 
            }
        }
        {
            let mut program = parse("104,1125899906842624,99");
            if let intcode::Status::Output(s) = program.execute() {
                assert_eq!(s, 1125899906842624);
            }
            else { panic!("Expected to receive output!") } 
        }
        {
            let mut program = parse("1102,34915192,34915192,7,4,7,99,0");
            if let intcode::Status::Output(s) = program.execute() {
                assert_eq!(s.to_string().len(), 16);
            }
            else { panic!("Expected to receive output!") } 
        }
    }
}
