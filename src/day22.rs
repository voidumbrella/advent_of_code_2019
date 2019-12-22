/*
 * Advent of Code 2019 Day 22
 *
 * Slam Shuffle
 *
 * Remarks:
 *  I can't math
 */

use regex::Regex;

#[derive (Debug, Clone)]
enum Operation {
    Cut(i128),
    Deal(i128),
    NewStack
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Operation>  {
    let deal = Regex::new(r"^deal with increment ([0-9]{1,})$").unwrap();
    let cut = Regex::new(r"^cut (-?[0-9]{1,})$").unwrap();

    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Operation::NewStack
            } else if let Some(caps) = deal.captures(line) {
                Operation::Deal(caps[1].parse().unwrap())
            } else if let Some(caps) = cut.captures(line) {
                Operation::Cut(caps[1].parse().unwrap())
            } else {
                panic!("{}", line)
            }
        }).collect()
}

#[aoc(day22, part1)]
fn solve_part1(input: &Vec<Operation>) -> i128 {
    let deck_size = 10007;
    let mut index = 2019;

    for operation in input.iter().cloned() {
        match operation {
            Operation::NewStack => index = deck_size - 1 - index,
            Operation::Cut(n) => index = (index - n) % deck_size,
            Operation::Deal(n) => index = (index * n) % deck_size,
        }
    }
    index
}

fn mod_inv(mut a: i128, mut base: i128) -> i128 {
    if base == 1 {
        return 0;
    }

    let orig = base;

    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        x + orig
    } else {
        x
    }
}

fn mod_exp(b: i128, e: i128, m: i128) -> i128 {
    // Reading the first few chapters of SICP paid off
    if e == 0 {
        return 1;
    } else if e % 2 == 0 { 
        return mod_exp(b, e / 2, m).pow(2) % m;
    } else {
        return (b * mod_exp(b, e - 1, m)) % m;
    }
}

// Reverse shuffle
fn f(ops: &Vec<Operation>, x: i128, deck_size: i128) -> i128 {
    let mut index = x;

    for operation in ops.iter().rev().cloned() {
        match operation {
            Operation::NewStack => index = deck_size - 1 - index,
            Operation::Cut(n) => index = (index + n) % deck_size,
            Operation::Deal(n) => index = (index * mod_inv(n, deck_size)) % deck_size,
        }
    }
    index
}


#[aoc(day22, part2)]
fn solve_part2(input: &Vec<Operation>) -> i128 {
    assert_eq!(f(input, 7545, 10007), 2019);
    let m = 119315717514047;
    let n = 101741582076661;

    // Find A and B such that Ax + B = f(x) (mod m)
    let x_0 = 2020;
    let x_1 = f(input, x_0, m);
    let x_2 = f(input, x_1, m);

    // x_1 = a * x_0 + b (mod m)
    // x_2 = a * x_1 + b (mod m)
    // x_2 - x_1 = a * x_1 - x_0 (mod m)
    // (x_2 - x_1) / (x_1 - x_0) = a (mod m)
    let a = ((x_2 - x_1) * mod_inv(x_1 - x_0, m)) % m;

    // x_1 = a * x_0 + b (mod m)
    // x_1 - a * x_0 = b (mod m)
    let b = (x_1 - a * x_0) % m;
    assert_eq!((a * x_0 + b) % m, x_1);
    
    // ax + b
    // a^2x + ab + b
    // a^3x + a^2b + ab + b
    // In general,
    // a^nx + b * (a^n - 1) / (a - 1)
    //
    // Taking mod m,
    // y = (a^n * x mod m) + b * ((a^n - 1) / (a - 1) mod m)
    // 
    // Then take (y + m) % m because % is actually remainder and not modulo
    ((mod_exp(a, n, m) * x_0 + b * ((mod_exp(a, n, m) - 1) * mod_inv(a - 1, m) % m)) % m + m) % m
}
