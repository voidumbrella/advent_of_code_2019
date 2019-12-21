/*
 * Advent of Code 2021 Day 21
 *
 * Springdroid Adventure
 *
 * Remarks:
 *  VM inside a VM!
 *
 *  Some simple Boolean algebra; took a bit to figure out the logic for part 2
 */

extern crate num;

use crate::intcode;
use std::collections::VecDeque;

#[aoc_generator(day21)]
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

fn input_string(droid: &mut intcode::IntCode, s: &str) {
    droid.input_queue.extend(s.bytes().map(|x| x as i64));
}

fn not(droid: &mut intcode::IntCode, x: char, y: char) {
    input_string(droid, &format!("NOT {} {}\n", x, y));
}

fn or(droid: &mut intcode::IntCode, x: char, y: char) {
    input_string(droid, &format!("OR {} {}\n", x, y));
}

fn and(droid: &mut intcode::IntCode, x: char, y: char) {
    input_string(droid, &format!("AND {} {}\n", x, y));
}

#[aoc(day21, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
    let mut droid = input.clone();

    /*
     * J = ~(A and B and C) and D
     */

    // Is there a hole in the first 3 tiles?
    // J = ~A or ~B or ~C = ~(A and B and C)
    or(&mut droid, 'A', 'J');
    and(&mut droid, 'B', 'J');
    and(&mut droid, 'C', 'J');
    not(&mut droid, 'J', 'J');

    // Can we safely land on the fourth tile?
    // J = J ^ D
    and(&mut droid, 'D', 'J');
    input_string(&mut droid, "WALK\n");
    loop {
        match droid.execute() {
            intcode::Status::Output(n) => {
                if n > 256 { return n; }
                else { print!("{}", n as u8 as char); }
            }
            _ => panic!("Unexpected halt")
        }
    }
}

#[aoc(day21, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    let mut droid = input.clone();

    /*
     * J = (~(A and B and C) and D) and (E or H)
     */

    // Is there a hole in the first 3 tiles?
    // J = ~A or ~B or ~C = ~(A and B and C)
    or(&mut droid, 'A', 'J');
    and(&mut droid, 'B', 'J');
    and(&mut droid, 'C', 'J');
    not(&mut droid, 'J', 'J');

    // Can we safely land on the fourth tile?
    // J = J ^ D
    and(&mut droid, 'D', 'J');

    // Is there a hole at the fifth tile?
    // If there is, the eighth tile must be a ground (because droid must jump right away)
    // Otherwise, doesn't matter and we can jump
    // T = E v (~E ^ H) = (E v ~E) ^ (E v H) = E v H
    or(&mut droid, 'E', 'T');
    or(&mut droid, 'H', 'T');

    // J = J ^ T
    and(&mut droid, 'T', 'J');

    input_string(&mut droid, "RUN\n");
    loop {
        match droid.execute() {
            intcode::Status::Output(n) => {
                if n > 256 { return n; }
                else { print!("{}", n as u8 as char); }
            }
            _ => panic!()
        }
    }
}
