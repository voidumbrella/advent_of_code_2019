/*
 * Advent of Code 2019 Day 17
 *
 * Set and Forget
 *
 * Remarks:
 *  I am also writing and forgetting this because my Part 2 solution is horrible
 *
 *  Also, what's with the unspecified behaviors of the program?
 *  Spent too much time figuring out why it was returning suspiciously small numbers.
 */

extern crate num;

use crate::intcode;
use std::collections::VecDeque;
use std::collections::HashSet;
use num::complex::Complex;

#[aoc_generator(day17)]
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

fn is_intersection(visited: &HashSet<Complex<i32>>, current: &Complex<i32>) -> bool {
    visited.contains(&(current + Complex::new(1, 0))) &&
    visited.contains(&(current + Complex::new(-1, 0))) &&
    visited.contains(&(current + Complex::new(0, 1))) &&
    visited.contains(&(current + Complex::new(0, -1)))
}

fn scan(robot: &mut intcode::IntCode) -> (HashSet<Complex<i32>>, Complex<i32>, Complex<i32>) {
    let mut map: HashSet<Complex<i32>> = HashSet::new();
    let mut current_position = Complex::new(0, 0);

    let mut robot_position = Complex::new(0, 0);
    let mut robot_direction = Complex::new(0, 0);

    loop {
        match robot.execute() {
            intcode::Status::Output(chr) => {
                match chr {
                    35 => {
                        map.insert(current_position);
                        current_position.re += 1;
                    },
                    94 | 118 | 60 | 62 => {
                        map.insert(current_position);
                        robot_position = current_position;
                        match chr {
                            94 => robot_direction.im = -1,
                            118 => robot_direction.im = 1,
                            60 => robot_direction.re = -1,
                            62 => robot_direction.re = 1,
                            _ => panic!(),
                        };
                        current_position.re += 1;
                    },
                    46 => current_position.re += 1,
                    10 => {
                        current_position.re = 0;
                        current_position.im += 1;
                    },
                    _ => (),
                }
            },
            intcode::Status::WaitingInput => break,
            intcode::Status::Halt => break,
        }
    }
    (map, robot_position, robot_direction)
}

#[aoc(day17, part1)]
fn solve_part1(input: &intcode::IntCode) -> i32 {
    let (visited, _, _) = scan(&mut input.clone());
    visited.iter()
        .filter(|coord| is_intersection(&visited, coord))
        .fold(0, |sum, a| sum + a.re * a.im)
}

// This is crap
fn compress(movements: &str, a_length: usize, b_length: usize, c_length: usize) -> Option<String> {
    let mut patterns = String::new();
    let mut i = 0;
    let end = movements.len();

    let a = &movements[..a_length];
    while &movements[i..i + a_length] == a {
        patterns += "A,";
        i += a_length;
    }

    let b = &movements[i..i + b_length];
    loop {
        if &movements[i..i + a_length] == a {
            patterns += "A,";
            i += a_length;
        } else if &movements[i..i + b_length] == b {
            patterns += "B,";
            i += b_length;
        } else {
            break;
        }
    }

    let c = &movements[i..i + c_length];
    loop {
        if i == end {
            // Strip trailing commas
            patterns.pop();
            let a = &a[..a_length - 1];
            let b = &b[..b_length - 1];
            let c = &c[..c_length - 1];
            return Some(format!("{}\n{}\n{}\n{}\n", patterns, a, b, c));
        } else if i <= end - a_length && &movements[i..i + a_length] == a {
            patterns += "A,";
            i += a_length;
        } else if i <= end - b_length && &movements[i..i + b_length] == b {
            patterns += "B,";
            i += b_length;
        } else if i <= end - c_length && &movements[i..i + c_length] == c {
            patterns += "C,";
            i += c_length;
        } else {
            break;
        }
    }
    None
}

#[aoc(day17, part2)]
fn solve_part2(input: &intcode::IntCode) -> i64 {
    let mut robot = input.clone();
    robot.mem[0] = 2;
    let (map, mut robot_position, mut robot_direction) = scan(&mut robot);

    let mut movements = String::new();
    let right_turn = Complex::new(0, 1);
    let left_turn = Complex::new(0, -1);

    loop {
        if map.contains(&(robot_position + robot_direction)) {
            let mut distance = 1;
            while map.contains(&(robot_position + (distance + 1) * robot_direction)) {
                distance += 1;
            }
            robot_position += distance * robot_direction;
            movements += &distance.to_string();
            movements += ",";
        } else {
            if map.contains(&(robot_position + robot_direction * left_turn)) {
                robot_direction *= left_turn;
                movements += "L,";
            } else if map.contains(&(robot_position + robot_direction * right_turn)) {
                robot_direction *= right_turn;
                movements += "R,";
            } else { // Dead end
                break;
            }
        }
    }
    // println!("{:?}", movements);

    for a in 1..21 {
        for b in 1..21 {
            for c in 1..21 {
                if let Some(s) = compress(&movements, a, b, c) {
                    // println!("{:?}", s);
                    robot.input_queue.extend(s.bytes().map(|x| x as i64));
                }
            }
        }
    }
    robot.input_queue.extend("n\n".bytes().map(|x| x as i64));
    loop {
        match robot.execute() {
            intcode::Status::Output(dust) => {
                // print!("{}", dust as u8 as char);
                if dust > 256 { return dust; } // not ASCII
            }
            intcode::Status::WaitingInput => panic!("Unexpected to wait for input"),
            intcode::Status::Halt => panic!("Halting due to error"),
        }
    }
}
