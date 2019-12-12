/*
 * Advent of Code 2019 Day 12
 *
 * The N-Body Program
 *
 * Remarks:
 *  This is probably when the puzzles stop being "do this"
 *  and become "do this without taking 50 years to do it".
 */

use std::collections::HashSet;
use regex::Regex;

#[derive (PartialEq, Eq, Clone)]
struct Moon {
    x: i64, y: i64, z: i64,
    dx: i64, dy: i64, dz: i64,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Moon>  {
    let re = Regex::new(r"^<x=(-?[0-9]{1,}), y=(-?[0-9]{1,}), z=(-?[0-9]{1,})>$").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Moon {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
                dx: 0,
                dy: 0,
                dz: 0,
            }
        }).collect()
}

fn simulate(moons: &mut[Moon]) {
    // Why won't it just let me mutate things inside the nested loop
    // this is much more complicated than it needs to be
    let mut velocities: Vec<(i64, i64, i64)> = Vec::new();
    velocities.resize(moons.len(), (0, 0, 0));
    for (i, moon) in moons.iter().enumerate() {
        let v = &mut velocities[i];
        v.0 = moon.dx;
        v.1 = moon.dy;
        v.2 = moon.dz;
        for other in moons.iter() {
            v.0 += if other.x > moon.x { 1 } else if other.x < moon.x { -1 } else { 0 };
            v.1 += if other.y > moon.y { 1 } else if other.y < moon.y { -1 } else { 0 };
            v.2 += if other.z > moon.z { 1 } else if other.z < moon.z { -1 } else { 0 };
        }
    }
    for (i, moon) in moons.iter_mut().enumerate() {
        let v = &velocities[i];
        moon.dx = v.0;
        moon.dy = v.1;
        moon.dz = v.2;
        moon.x += moon.dx;
        moon.y += moon.dy;
        moon.z += moon.dz;
    }
}

fn calculate_energy(moons: &[Moon]) -> i64 {
    let mut total_energy = 0;
    for moon in moons.iter() {
        let potential_energy = moon.x.abs() + moon.y.abs() + moon.z.abs();
        let kinetic_energy = moon.dx.abs() + moon.dy.abs() + moon.dz.abs();
        total_energy += potential_energy * kinetic_energy;
    }
    total_energy
}

#[aoc(day12, part1)]
fn solve_part1(moons: &[Moon]) -> i64 {
    let mut moons = moons.to_vec();
    for _i in 0..1000 {
        simulate(&mut moons);
    }
    calculate_energy(&moons)
}

#[aoc(day12, part2)]
fn solve_part2(moons: &[Moon]) -> i64 {
    let moons = moons.to_vec();

    let mut xs_cycle = -1;
    let mut ys_cycle = -1;
    let mut zs_cycle = -1;
    {
        let mut moons = moons.clone();
        let mut xs: HashSet<Vec<(i64, i64)>> = HashSet::new();
        let mut ys: HashSet<Vec<(i64, i64)>> = HashSet::new();
        let mut zs: HashSet<Vec<(i64, i64)>> = HashSet::new();
        let mut i = 0;
        while xs_cycle < 0 || ys_cycle < 0 || zs_cycle < 0 {
            let x: Vec<(i64, i64)> = moons.iter().map(|a| (a.x, a.dx)).collect();
            let y: Vec<(i64, i64)> = moons.iter().map(|a| (a.y, a.dy)).collect();
            let z: Vec<(i64, i64)> = moons.iter().map(|a| (a.z, a.dz)).collect();
            // insert returns false if value was present
            if !xs.insert(x) && xs_cycle < 0 { xs_cycle = i; }
            if !ys.insert(y) && ys_cycle < 0 { ys_cycle = i; }
            if !zs.insert(z) && zs_cycle < 0 { zs_cycle = i; }
            simulate(&mut moons);
            i += 1;
        }
    }
    
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a }
        else { gcd(b, a % b) }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / gcd(a, b) * b
    }

    lcm(lcm(xs_cycle, ys_cycle), zs_cycle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = "<x=-1, y=0, z=2>\n\
            <x=2, y=-10, z=-7>\n\
            <x=4, y=-8, z=8>\n\
            <x=3, y=5, z=-1>";
            let mut moons = parse(input);
            for _i in 0..10 {
                simulate(&mut moons);
            }
            assert_eq!(calculate_energy(&moons), 179);
        }
        {
            let input = "<x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>";
            let mut moons = parse(input);
            for _i in 0..100 {
                simulate(&mut moons);
            }
            assert_eq!(calculate_energy(&moons), 1940);
        }
    }

    #[test]
    fn part2() {
        {
            let input = "<x=-1, y=0, z=2>\n\
            <x=2, y=-10, z=-7>\n\
            <x=4, y=-8, z=8>\n\
            <x=3, y=5, z=-1>";
            assert_eq!(solve_part2(&parse(input)), 2772);
        }
        {
            let input = "<x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>";
            assert_eq!(solve_part2(&parse(input)), 4_686_774_924);
        }
    }
}
