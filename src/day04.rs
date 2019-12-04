/*
 * Advent of Code 2019 Day 4
 *
 * Secure Container
 *
 * Remarks:
 *  Much easier than Day 3.
 *  Not very optimized however (takes around 20~35ms to run on my machine).
 */

use regex::Regex;
use std::ops::Range;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Range<i32> {
    let re = Regex::new(r"^([0-9]{6})-([0-9]{6})$").expect("regex is invalid");

    let caps = re.captures(input).unwrap();
    caps[1].parse().unwrap()..caps[2].parse().unwrap()
}

fn is_sorted<T>(data: &[T]) -> bool
    where T: Ord, {
    data.windows(2).all(|w| w[0] <= w[1])
}

fn count_reps(s :&[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let mut count = 1;
    for n in s.windows(2) {
        if n[0] == n[1] {
            count += 1;
        }
        else {
            v.push(count);
            count = 1;
        }
    }
    v.push(count);
    v 
}

fn part1_check(n: i32) -> bool {
    let s = n.to_string();
    let sb = s.as_bytes();
    if !is_sorted(&sb) {
        return false;
    }
    let v = count_reps(&sb);

    v != [1,1,1,1,1,1]
}

fn part2_check(n: i32) -> bool {
    let s = n.to_string();
    let sb = s.as_bytes();
    if !is_sorted(&sb) {
        return false;
    }
    let v = count_reps(&sb);

    v.contains(&2)
}

#[aoc(day4, part1)]
fn solve_part1(range: &Range<i32>) -> i32 {
    let mut count = 0;
    for i in range.clone() {
        if part1_check(i) {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
fn solve_part2(range: &Range<i32>) -> i32 {
    let mut count = 0;
    for i in range.clone() {
        if part2_check(i) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            assert!(!part1_check(723236), "723236 should be false");
            assert!(!part1_check(123456), "123456 should be false");
            assert!(part1_check(113456), "113456 should be true");
            assert!(part1_check(112333), "112333 should be true");
            assert!(part1_check(666666), "666666 should be true");
        }
    }

    #[test]
    fn part2() {
        {
            assert!(!part2_check(723236), "723236 should be false");
            assert!(!part2_check(123456), "123456 should be false");
            assert!(part2_check(113456), "113456 should be true");
            assert!(!part2_check(111333), "111333 should be false");
            assert!(part2_check(111566), "111566 should be true");
            assert!(!part2_check(122223), "122223 should be false");
            assert!(!part2_check(111123), "111123 should be false");
            assert!(!part2_check(442211), "442211 should be false");
        }
    }
}
