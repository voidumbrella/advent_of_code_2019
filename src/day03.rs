/*
 * Advent of Code 2019 Day 3
 *
 * Crossed Wires
 *
 * Remarks:
 *  For whatever reason, I decided to store each line segments
 *  for my first iteration, which resulted in a horrible mess why did i do that
 *
 *  I saw a cool solution that uses a custom iterator to represent traversing the wire,
 *  maybe I'll try to do something like that in the future if I can figure it out?
 */

use regex::Regex;
use std::cmp;
use std::collections::HashSet;

enum TurnDirection {
    Right,
    Left,
    Up,
    Down,
}

struct Turn {
    direction: TurnDirection,
    distance: i32,
}

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
struct Point (i32, i32);

fn get_dxdy(direction: &TurnDirection) -> Point {
    match direction {
        TurnDirection::Right => { Point (1, 0) }
        TurnDirection::Left => { Point (-1, 0) }
        TurnDirection::Up => { Point (0, 1) }
        TurnDirection::Down => { Point (0, -1) }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<Turn>> {
    let re = Regex::new(r"([RUDL])([0-9]{1,})").unwrap();

    let mut wires: Vec<Vec<Turn>> = Vec::new();

    for line in input.lines().take(2) {
        wires.push(
            line
                .split(',')
                .map(|line| {
                    let caps = re.captures(line).unwrap();
                    Turn {
                        direction: match &caps[1] {
                            "R" => TurnDirection::Right,
                            "U" => TurnDirection::Up,
                            "D" => TurnDirection::Down,
                            "L" => TurnDirection::Left,
                            _ => panic!("Unknown direction"),
                        },
                        distance: caps[2].parse().unwrap()
                    }
                }).collect()
        )
    }

    wires
}

fn distance(p: &Point) -> i32 {
    (p.0).abs() + (p.1).abs()
}

fn find_collisions(a: &[Turn], b: &[Turn]) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();

    let mut points: HashSet<Point> = HashSet::new();
    {
        let mut cur = Point (0, 0);
        for turn in a.iter() {
            let dxdy = get_dxdy(&turn.direction);
            for _i in 0..turn.distance {
                cur.0 += dxdy.0;
                cur.1 += dxdy.1;
                points.insert(cur.clone());
            }
        }
    }

    {
        let mut cur = Point (0, 0);
        for turn in b.iter() {
            let dxdy = get_dxdy(&turn.direction);
            for _i in 0..turn.distance {
                cur.0 += dxdy.0;
                cur.1 += dxdy.1;
                if points.contains(&cur) {
                    intersections.push(cur.clone());
                }
            }
        }
    }

    intersections

}

#[aoc(day3, part1)]
fn solve_part1(wires: &[Vec<Turn>]) -> i32 {
    find_collisions(&wires[0], &wires[1])
        .iter()
        .map(|p| distance(&p))
        .min()
        .unwrap()
}

fn get_distance(wire: &[Turn], dest: &Point) -> i32 {
    let mut cur = Point (0, 0);
    let mut distance = 0;
    for turn in wire.iter() {
        let dxdy = get_dxdy(&turn.direction);
        for _i in 0..turn.distance {
            if cur == *dest {
                return distance;
            }
            cur.0 += dxdy.0;
            cur.1 += dxdy.1;
            distance += 1;
        }
    }
    panic!("Could not reach given point!");
}

#[aoc(day3, part2)]
fn solve_part2(wires: &[Vec<Turn>]) -> i32 {
    let mut min_distance = core::i32::MAX;

    let intersections = find_collisions(&wires[0], &wires[1]);

    for intersection in intersections.iter() {
        let first_distance = get_distance(&wires[0], intersection);
        let second_distance = get_distance(&wires[1], intersection);

        min_distance = cmp::min(min_distance, first_distance + second_distance);
    }

    min_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input =
            "R8,U5,L5,D3
            U7,R6,D4,L4";
            assert_eq!(solve_part1(&parse(input)), 6);
        }
        {
            let input =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
            U62,R66,U55,R34,D71,R55,D58,R83";
            assert_eq!(solve_part1(&parse(input)), 159);
        }
        {
            let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
            assert_eq!(solve_part1(&parse(input)), 135);
        }
    }

    #[test]
    fn part2() {
        {
            let input =
            "R8,U5,L5,D3
            U7,R6,D4,L4";
            assert_eq!(solve_part2(&parse(input)), 30);
        }
        {
            let input =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
            U62,R66,U55,R34,D71,R55,D58,R83";
            assert_eq!(solve_part2(&parse(input)), 610);
        }
        {
            let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
            assert_eq!(solve_part2(&parse(input)), 410);
        }
    }
}
