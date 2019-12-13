/*
 * Advent of Code 2019 Day 13
 *
 * Care Package
 *
 * Remarks:
 *  Thank god the game isn't obfuscated,
 *  because then I would've had to actually play the game!
 *  
 *  Also the interface for my Intcode machine is really ugly jeez
 */

extern crate num;

use std::collections::VecDeque;
use crate::intcode;

#[aoc_generator(day13)]
fn parse(input: &str) -> intcode::IntCode  {
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

#[aoc(day13, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
    let mut game = input.clone();
    let mut num_blocks = 0;
    loop {
        match game.execute() {
            intcode::Status::Output(_) => {
                if let intcode::Status::Output(_) = game.execute() {}
                else { panic!("Was expecting y-coordinate") }
                if let intcode::Status::Output(n) = game.execute() {
                    if n == 2 { num_blocks += 1; }
                }
                else { panic!("Was expecting tile type") }
            }
            intcode::Status::WaitingInput => panic!("Was expecting x-coordinate"),
            intcode::Status::Halt => break,
        }
    }
    num_blocks
}

#[aoc(day13, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    let mut game = input.clone();
    game.mem[0] = 2;
    for i in 1543..1584 {
        game.mem[i] = 3; // extend paddle to cover the entire screen
    }
    let mut score = 0;
    loop {
        match game.execute() {
            intcode::Status::Output(n) => {
                let x = n;
                let y: i64;
                if let intcode::Status::Output(n) = game.execute() {
                    y = n;
                }
                else { panic!("Was expecting y-coordinate") }
                if let intcode::Status::Output(n) = game.execute() {
                    if x == -1 && y == 0 { score = n }
                }
                else { panic!("Was expecting tile type or score") }
            }
            intcode::Status::WaitingInput => {
                game.input_queue.push_back(0);
            }
            intcode::Status::Halt => break,
        }
    }
    score
} 
