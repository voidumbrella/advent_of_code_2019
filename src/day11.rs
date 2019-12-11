/*
 * Advent of Code 2019 Day 11
 *
 * Space Police
 *
 * Remarks:
 *  I'm overall happy with how my IntCode machine turned out,
 *  but I think it would've been a much better idea to have an output queue
 *  instead of halting the machine after every output.
 *  Might refactor that part if I feel like in the future (I won't)
 *
 *  The problem itself was pretty straightforward!
 *  I think my solution is good-ish except for the ugly `receiving_color` flag
 *  which could be gone if there was an output queue.
 */

extern crate num;

use num::complex::Complex;
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::intcode;

#[aoc_generator(day11)]
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

#[derive (PartialEq, Eq, Hash, Clone)]
struct Coord { x: i64, y: i64 }
#[derive (PartialEq, Eq)]
enum Color { 
    Black,
    White,
}

#[aoc(day11, part1)]
fn solve_part1(input: &intcode::IntCode) -> i64 {
    let mut grid: HashMap<Complex<i64>, Color> = HashMap::new();

    let mut robot = input.clone();
    let mut robot_position = Complex::new(0, 0);
    let mut robot_direction = Complex::new(0, 1);

    grid.insert(robot_position.clone(), Color::Black);
    let mut receiving_color = true;
    let turn = Complex::new(0, 1);
    loop {
        match robot.execute() {
            intcode::Status::Output(n) => {
                if receiving_color {
                    let color = if n == 0 { Color::Black } else { Color::White };
                    grid.insert(robot_position.clone(), color);
                    receiving_color = false;
                } else {
                    if n == 0 { robot_direction *= turn; } // left turn
                    else { robot_direction /= turn; } // right turn
                    robot_position += robot_direction;
                    receiving_color = true;
                }
            }
            intcode::Status::WaitingInput => {
                let current_color = match grid.get(&robot_position) {
                    None => &Color::Black,
                    Some(c) => c,
                };
                let input = if current_color == &Color::Black { 0 } else { 1 };
                robot.input_queue.push_back(input);
            }
            intcode::Status::Halt => break,
        }
    }

    grid.len() as i64
}

#[aoc(day11, part2)]
fn solve_part2(input: &intcode::IntCode) -> &'static str {
    let mut grid: HashMap<Complex<i64>, Color> = HashMap::new();

    let mut robot = input.clone();
    let mut robot_position = Complex::new(0, 0);
    let mut robot_direction = Complex::new(0, 1);

    grid.insert(robot_position.clone(), Color::White);
    let mut receiving_color = true;
    let turn = Complex::new(0, 1);
    loop {
        match robot.execute() {
            intcode::Status::Output(n) => {
                if receiving_color {
                    let color = if n == 0 { Color::Black } else { Color::White };
                    grid.insert(robot_position.clone(), color);
                    receiving_color = false;
                } else {
                    if n == 0 { robot_direction *= turn; } // left turn
                    else { robot_direction /= turn; } // right turn
                    robot_position += robot_direction;
                    receiving_color = true;
                }
            }
            intcode::Status::WaitingInput => {
                let current_color = match grid.get(&robot_position) {
                    None => &Color::Black,
                    Some(c) => c,
                };
                let input = if current_color == &Color::Black { 0 } else { 1 };
                robot.input_queue.push_back(input);
            }
            intcode::Status::Halt => break,
        }
    }

    let min_x = grid.keys().min_by_key(|a| a.re).unwrap().re;
    let max_x = grid.keys().max_by_key(|a| a.re).unwrap().re;
    let min_y = grid.keys().min_by_key(|a| a.im).unwrap().im;
    let max_y = grid.keys().max_by_key(|a| a.im).unwrap().im;

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut points = vec![vec![false; 100]; 100]; // Magic number that's probably good enough

    for (point, color) in &grid {
        if color == &Color::White {
            let x = point.re - min_x;
            let y = max_y - point.im;
            points[y as usize][x as usize] = true;
        }
    }
    for line in points.iter().take(height as usize) {
        for is_white in line.iter().take(width as usize) {
            if *is_white { print!("⬜") }
            else { print!("⬛") }
        }
        println!();
    }

    "Output above"
} 
