/*
 * Advent of Code 2019 Day n
 *
 *
 */

use regex::Regex;

#[aoc_generator(dayn)]
fn input_generator(input: &str) -> OUTPUTTYPE  {
    let re = Regex::new(r"^([0-9]{1,})x([0-9]{1,})x([0-9]{1,})$").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse().unwrap(),
            )
        }).collect()
}

#[aoc(dayn, part1)]
fn solve_part1(input: OUTPUTTYPE) -> i32 {
    input
        .iter()
        .map(|| {
        })
}

#[aoc(dayn, part2)]
fn solve_part2(input: OUTPUTTYPE) -> i32 {
    input
        .iter()
        .map(|| {
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
        }
    }

    #[test]
    fn part2() {
        {
        }
    }
}
