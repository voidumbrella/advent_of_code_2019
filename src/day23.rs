/*
 * Advent of Code 2019 Day 23
 *
 * Category Six
 *
 * Remarks:
 *  Straightforward problem, but I think this would be a good excuse to
 *  learn concurrency in Rust so maybe I will return to this?
 */

extern crate num;

use crate::intcode;
use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day23)]
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

#[aoc(day23, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
    let mut computers: Vec<intcode::IntCode> = Vec::new();
    computers.reserve(50);
    for i in 0..50 {
        computers.push(input.clone());
        computers[i].input_queue.push_back(i as i64);
    }
    loop {
        let mut packets: Vec<(usize, i64, i64)> = Vec::new();
        for computer in computers.iter_mut() {
            match computer.execute() {
                intcode::Status::Halt => panic!("A computer halted unexpectedly!"),
                intcode::Status::WaitingInput => computer.input_queue.push_back(-1),
                intcode::Status::Output(addr) => {
                    let x;
                    let y;

                    if let intcode::Status::Output(n) = computer.execute() { x = n; }
                    else { panic!("Expected x value of packet") }
                    if let intcode::Status::Output(n) = computer.execute() { y = n; }
                    else { panic!("Expected y value of packet") }

                    packets.push((addr as usize, x, y));
                }
            }
        }
        for (addr, x, y) in packets {
            if addr == 255 { return y; }
            computers[addr].input_queue.push_back(x);
            computers[addr].input_queue.push_back(y);
        }
    }
}

#[aoc(day23, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    let mut computers: Vec<intcode::IntCode> = Vec::new();
    computers.reserve(50);
    for i in 0..50 {
        computers.push(input.clone());
        computers[i].input_queue.push_back(i as i64);
    }
    let mut sent_y: HashSet<i64> = HashSet::new();
    let mut nat_x = 0;
    let mut nat_y = 0;
    loop {
        let mut packets: Vec<(usize, i64, i64)> = Vec::new();
        for computer in computers.iter_mut() {
            match computer.execute() {
                intcode::Status::Halt => panic!("A computer halted unexpectedly!"),
                intcode::Status::WaitingInput => computer.input_queue.push_back(-1),
                intcode::Status::Output(addr) => {
                    let x;
                    let y;

                    if let intcode::Status::Output(n) = computer.execute() { x = n; }
                    else { panic!("Expected x value of packet") }
                    if let intcode::Status::Output(n) = computer.execute() { y = n; }
                    else { panic!("Expected y value of packet") }

                    packets.push((addr as usize, x, y));
                }
            }
        }
        if packets.is_empty() {
            computers[0].input_queue.push_back(nat_x);
            computers[0].input_queue.push_back(nat_y);
            if !sent_y.insert(nat_y) {
                return nat_y;
            }
        } else {
            for (addr, x, y) in packets {
                if addr == 255 {
                    nat_x = x;
                    nat_y = y;
                } else {
                    computers[addr].input_queue.push_back(x);
                    computers[addr].input_queue.push_back(y);
                }
            }
        }
    }
}
