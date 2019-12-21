/*
 * Advent of Code 2019 Day 19
 *
 * Tractor Beam
 *
 * Remarks:
 *  I thought a bit if I was expected to pull out trigonometry, but it turns out that
 *  just walking through the beam and checking for the corners was a good enough solution.
 *
 *  Also, this is the first (and probably the only) time I actually managed to get in the
 *  global leaderboard! Rank 253 for Part 1 and Rank 24 for Part 2!!!
 *
 *  I can die happy now.
 */

extern crate num;

use crate::intcode;
use std::collections::VecDeque;

#[aoc_generator(day19)]
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

#[aoc(day19, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut drone = input.clone();
            drone.input_queue.push_back(x);
            drone.input_queue.push_back(y);
            match drone.execute() {
                intcode::Status::Output(a) => {
                    if a == 1 { count += 1; }
                },
                _ => (),
            }
        }
    }
    count
}

#[aoc(day19, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    // ....####...
    // ......####.
    // ^^^^
    // keep track of x to skip checking tiles that aren't obviously in the beam
    let mut x = 0;

    // the first few rows don't have any attracted tiles in my input for some reason?
    for y in 10..std::i64::MAX {
        let edge;

        // Get the lower left corner of the box
        loop {
            let mut drone = input.clone();
            drone.input_queue.extend(vec![x, y]);
            match drone.execute() {
                intcode::Status::Output(n) => {
                    if n == 1 { edge = x; break; }
                },
                _ => (),
            }
            x += 1;
        }

        // Check upper-right corner; if both corners are in the beam
        // the entire beam is guaranteed to be in it
        let mut drone = input.clone();
        drone.input_queue.extend(vec![edge + 99, y - 99]);
        match drone.execute() {
            intcode::Status::Output(n) => {
                // Make sure to return based on upper-left corner
                if n == 1 { return edge * 10000 + (y - 99); }
            },
            _ => (),
        }
    }
    panic!("Could not find suitable position (This is a bug)")
}
