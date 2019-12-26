/*
 * Advent of Code 2019 Day 25
 *
 * Cryostasis
 *
 * Remarks:
 *  If I was cool I would've written a program that parses the output,
 *  travels around and pick up items, and at the pressure sensor
 *  try all different combinations of picked up items.
 *  I'm not a cool person.
 *
 *  A new command `solve` was hacked in, which if
 *      1) all the items are collected,
 *      2) droid is located at Security Checkpoint, just before Pressure-Sensitive Floor, 
 *      3) the `bruteforce` function is modified appropriately,
 *  then will enter commands into the droid that try each combination of the items
 *  and attempt to pass the pressure sensor.
 */

extern crate num;

use crate::intcode;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::io;

#[aoc_generator(day25)]
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
    i.mem.resize(15_000, 0);
    i
}

// Copied from https://gist.github.com/synecdoche/9ade913c891dda6fcf1cdac823e7d524
fn powerset<T: Clone>(slice: &[T]) -> Vec<Vec<T>> {
    let mut v: Vec<Vec<T>> = Vec::new();

    for mask in 0..(1 << slice.len()) {
        let mut ss: Vec<T> = vec![];
        let mut bitset = mask;
        while bitset > 0 {
            // isolate the rightmost bit to select one item
            let rightmost: u64 = bitset & !(bitset - 1);
            // turn the isolated bit into an array index
            let idx = rightmost.trailing_zeros();
            let item = (*slice.get(idx as usize).unwrap()).clone();
            ss.push(item);
            // zero the trailing bit
            bitset &= bitset - 1;
        }
        v.push(ss);
    }
    v
}

/*
 * Returns a sequence of commands that try out every single combination of the items.
 * Modify items and directions based on input.
 */
fn bruteforce() -> String {
    // hardcoded list of all items
    let items = ["prime number", "candy cane", "asterisk", "food ration", "boulder", "mutex", "mug"];
    // hardcoded direction to pressure sensor from security checkpoint
    let direction = "north\n";

	let mut commands = String::new();
    let mut inv = HashSet::new();
    for set in powerset(&items) {
        // Drop any items that aren't included
        let mut new_inv = inv.clone();
        for carrying in &inv {
            if !set.contains(carrying) {
                commands.push_str(&format!("drop {}\n", carrying));
                new_inv.remove(carrying);
            }
        }
        inv = new_inv;

        // Pick up items that we don't have
        for item in set {
            if !inv.contains(item) {
                commands.push_str(&format!("take {}\n", item));
                inv.insert(item);
            }
        }
        commands.push_str(direction);
    }
	commands
}

#[aoc(day25, part1)]
fn solve_part1(input: &intcode::IntCode) -> i32 {
    let mut droid = input.clone();

    loop {
        match droid.execute() {
            intcode::Status::Halt => break,
            intcode::Status::WaitingInput => {
				// TODO: what if carriage returns??
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    eprintln!("Error reading from stdin: {}", e);
                }
                if input == "solve\n" {
                    droid.input_queue.extend(bruteforce().bytes().map(|x| x as i64));
                } else {
                    droid.input_queue.extend(input.bytes().map(|x| x as i64));
                }
            }
            intcode::Status::Output(c) => print!("{}", c as u8 as char),
        }
    }
	0
}
