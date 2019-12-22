/*
 * Advent of Code 2019 Day 22
 *
 * Slam Shuffle
 *
 * Remarks:
 *  I can't math.
 *  I learned some basic modular arithmetic from this though so yay
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
            Operation::NewStack => index = (-index - 1) % deck_size,
            Operation::Cut(n) => index = (index - n) % deck_size,
            Operation::Deal(n) => index = (index * n) % deck_size,
        }
    }
    index
}

fn modulo(x: i128, m: i128) -> i128 {
    ((x % m) + m) % m
}

// Returns (g, x, y) such that a * x + b * y = g = gcd(a, b)
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1)
    } else {
        let (g, x, y) = egcd(modulo(b, a), a);
        return (g, y - (b / a) * x, x)
    }
}

// Returns x such that ax = 1 (mod m)
fn mod_inv(a: i128, m: i128) -> i128 {
    let (g, x, _) = egcd(a, m);
    if g == 1 {
        return modulo(x, m);
    } else {
        panic!("modular inverse of {} and {} not found", a, m);
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

#[aoc(day22, part2)]
fn solve_part2(input: &Vec<Operation>) -> i128 {

    let m = 119315717514047; // deck size
    let n = 101741582076661; // loops

    // Find a and b such that ay + b (mod m) = x,
    // where y is the position after a round of shuffling and x is the original position
    let mut a = 1;
    let mut b = 0;
    for operation in input.iter().rev().cloned() {
        match operation {
            Operation::NewStack => {
                a = -a;
                b = -b - 1;
            }
            Operation::Cut(n) => {
                b += n;
            }
            Operation::Deal(n) => {
                let temp = mod_inv(n, m);
                a *= temp;
                b *= temp;
            }
        }
        a = modulo(a, m);
        b = modulo(b, m);
    }

    // ax + b
    // a^2x + ab + b
    // a^3x + a^2b + ab + b
    // In general,
    // a^nx + b * (a^n - 1) / (a - 1)
    //
    // Taking mod m,
    // y = (a^n * x mod m) + b * ((a^n - 1) / (a - 1) mod m)
    //                           |------------------|
    //                             must be moduloed
    //                           to prevent overflow!
    let p = mod_exp(a, n, m) * 2020 + b * modulo((mod_exp(a, n, m) - 1) * mod_inv(a - 1, m), m);
    modulo(p, m)
}
