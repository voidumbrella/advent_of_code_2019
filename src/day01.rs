/*
 * Advent of Code 2019 Day 1
 *
 * The Tyranny of the Rocket Equation
 *
 * Remarks:
 *  Nothing noteworthy, except I spent too much time debugging Part 2
 *  because I read the question wrong :/
 */

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            line.parse().unwrap()
        }).collect()
} 

fn calc_fuel(x: i32) -> i32 {
    if x < 6 { 0 }
    else { x / 3 - 2 }
}

#[aoc(day1, part1)]
fn solve_part1(input: &Vec<i32>) -> i32 {
    input.iter().map(|n| calc_fuel(*n)).sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &Vec<i32>) -> i32 {
    input.iter()
        .map(|n| {
            let mut total_fuel = 0;
            let mut fuel = *n;
            while fuel > 0 {
                fuel = calc_fuel(fuel);
                total_fuel += fuel;
            }
            total_fuel
        }).sum()
}
