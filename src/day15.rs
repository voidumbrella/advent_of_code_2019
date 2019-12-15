/*
 * Advent of Code 2019 Day 15
 *
 * Oxygen System
 *
 * Remarks:
 *  I'm using DFS, I'm using BFS, I'm using the combination of DFS and BFS.
 */

extern crate num;

use crate::intcode;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use num::complex::Complex;

#[derive (Clone)]
struct Drone {
    code: intcode::IntCode,
    pos: Complex<i64>,
}

#[derive (PartialEq, Eq, Debug, Clone)]
enum Tile {
    Wall,
    Space,
    Oxygen,
}

type Maze = HashMap<Complex<i64>, Tile>;

#[aoc_generator(day15)]
fn parse(input: &str) -> Maze {
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
    let mut drone = Drone {
        code: i.clone(),
        pos: Complex::new(0, 0),
    };
    let mut map: Maze = HashMap::new();
    map.insert(drone.pos, Tile::Space);
    scan(&mut drone, &map)

}

fn scan(drone: &mut Drone, map: &Maze) -> Maze {
    let mut map = map.clone();
    for input in 1..5 {
        let mut drone = drone.clone();
        let direction = match input {
            1 => Complex::new(0, 1),
            2 => Complex::new(0, -1),
            3 => Complex::new(-1, 0),
            4 => Complex::new(1, 0),
            _ => panic!(),
        };
        let new_pos = drone.pos + direction;

        // Check if tile is explored
        if let None = map.get(&new_pos) {
            // Move drone and check result
            drone.code.input_queue.push_back(input);
            if let intcode::Status::Output(n) = drone.code.execute() {
                let new_tile = match n {
                    0 => Tile::Wall,
                    1 => Tile::Space,
                    2 => Tile::Oxygen,
                    _ => panic!("Unknown response from drone"),
                };
                // If not a wall, update drone position
                if new_tile != Tile::Wall { drone.pos = new_pos; }
                map.insert(new_pos, new_tile);
            } else { panic!("Expecting response from drone") }

            // From our new position, recursively scan the entire maze.
            map.extend(scan(&mut drone, &map));
        }
    }
    map
}

#[aoc(day15, part1)]
fn solve_part1(map: &Maze) -> i64 {
    // For keeping track of tiles that were visited
    let mut visited: HashSet<Complex<i64>> = HashSet::new();
    // Queue of (pos, distance) for each tile to visit
    let mut to_visit: VecDeque<(Complex<i64>, i64)> = VecDeque::new();
    to_visit.push_back((Complex::new(0, 0), 0));

    let directions = [Complex::new(0, 1), Complex::new(0, -1), Complex::new(-1, 0), Complex::new(1, 0)];
    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();
        visited.insert(current.0);
        if map.get(&current.0).unwrap() == &Tile::Oxygen { return current.1; }

        for direction in directions.iter() {
            let new = current.0 + direction;
            if visited.contains(&new) { continue; }
            if let Some(tile) = map.get(&new) {
                if tile != &Tile::Wall {
                    to_visit.push_back((new, current.1 + 1));
                }
            }
        }
    }
    panic!("Could not find oxygen");
}

#[aoc(day15, part2)]
fn solve_part2(map: &Maze) -> i64 {
    // For keeping track of tiles that were visited
    let mut visited: HashSet<Complex<i64>> = HashSet::new();
    // Queue of (pos, distance) for each tile to visit
    let mut to_visit: VecDeque<(Complex<i64>, i64)> = VecDeque::new();

    for (pos, tile) in map {
        if tile == &Tile::Oxygen {
            to_visit.push_back((pos.clone(), 0));
            break;
        }
    }

    let directions = [Complex::new(0, 1), Complex::new(0, -1), Complex::new(-1, 0), Complex::new(1, 0)];
    let mut t = 0;
    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();
        visited.insert(current.0);
        if current.1 > t { t = current.1; }

        for direction in directions.iter() {
            let new = current.0 + direction;
            if visited.contains(&new) { continue; }
            if let Some(tile) = map.get(&new) {
                if tile != &Tile::Wall {
                    to_visit.push_back((new, current.1 + 1));
                }
            }
        }
    }
    t
} 
