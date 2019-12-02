/*
 * Advent of Code 2019 Day 2
 *
 * 1202 Program Alarm
 */

use std::iter::Iterator;

#[derive(Clone)]
struct IntCode {
    ip: usize,
    mem: Vec<i32>
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> IntCode {
    IntCode {
        ip: 0,
        mem: input
            .split(',')
            .map(|n| { n.parse().unwrap() })
            .collect()
    }
} 

fn execute(p: &IntCode, noun: i32, verb: i32) -> i32 {
    let mut p = p.clone();
    p.mem[1] = noun;
    p.mem[2] = verb;
    loop {
        let num_params;
        let opcode = p.mem[p.ip];

        match opcode {
            1 => { num_params = 3; }
            2 => { num_params = 3; }
            99 => { num_params = 0; }
            _ => panic!("Invalid opcode {}", opcode),
        }

        let args = p.mem[p.ip + 1..p.ip + 1 + num_params].to_vec();

        match opcode {
            1 => {
                p.mem[args[2] as usize] =
                    p.mem[args[0] as usize] + p.mem[args[1] as usize];
            },
            2 => {
                p.mem[args[2] as usize] =
                    p.mem[args[0] as usize] * p.mem[args[1] as usize];
            },
            99 => break,
            _ => panic!("Invalid opcode {}", opcode),
        }

        p.ip += num_params + 1;
    }

    p.mem[0]
}

#[aoc(day2, part1)]
fn solve_part1(input: &IntCode) -> i32 {
    execute(&input, 12, 2)
}

#[aoc(day2, part2)]
fn solve_part2(input: &IntCode) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(&input, noun, verb) == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    panic!("Could not find suitable inputs!");
}
