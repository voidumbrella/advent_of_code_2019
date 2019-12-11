/*
 * Advent of Code 2019 Day 10 
 *
 * Monitoring Station
 *
 * Remarks:
 *  I wanted to use only integers and my code is a mess
 */

use std::cmp::Ordering;
use std::collections::HashSet;

#[derive (Debug, Hash, PartialEq, Eq, Clone)]
struct Vec2D { x: i32, y: i32 }

impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Vec2D) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec2D {
    /*
     * Compare two vectors, starting from 12'o clock and moving clockwise
     */
    fn cmp(&self, other: &Vec2D) -> Ordering {
        // If one is to the left and another is to the right of the origin,
        // it's obvious which one comes first.
        if self.x >= 0 && other.x < 0 { return Ordering::Less }
        else if self.x < 0 && other.x >= 0 { return Ordering::Greater }
        
        // Calculate the determinant of the matrix [self, other].
        // If positive, self lies on the right of other..
        let det = self.x * other.y - other.x * self.y;
        if det < 0 { Ordering::Greater }
        else if det > 0 { Ordering::Less }
        else {
            // Collinear, use distance to compare
            let ra = self.x.pow(2) + self.y.pow(2);
            let rb = other.x.pow(2) + other.y.pow(2);
            if ra == rb { Ordering::Equal }
            else if ra < rb { Ordering::Less }
            else { Ordering::Greater }
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec2D>  {
    let mut asteroids: Vec<Vec2D> = Vec::new();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Vec2D {x: i as i32, y: j as i32});
            }
        }
    }
    asteroids

}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

fn get_station(asteroids: &[Vec2D]) -> Vec2D {
    let mut maximum: (Vec2D, i32) = (Vec2D {x: 0, y: 0}, 0);

    // Good old O(n^2) solution (fortunately the input is small enough)
    for station in asteroids {
        let mut seen_slopes: HashSet<Vec2D> = HashSet::new();
        for asteroid in asteroids {
            if asteroid == station { continue; }
            seen_slopes.insert(slope(asteroid, station));
        }
        let num_seen = seen_slopes.len() as i32;
        if num_seen > maximum.1 { maximum = (station.clone(), num_seen); }
    }
    maximum.0
}

// Vec2D from a to b
fn slope(a: &Vec2D, b: &Vec2D) -> Vec2D {
    let y = a.y - b.y;
    let x = a.x - b.x;
    let gcd = gcd(x.abs(), y.abs());
    Vec2D { y: y / gcd, x: x / gcd } // simplify, so collinear points have same slope
}

#[aoc(day10, part1)]
fn solve_part1(input: &[Vec2D]) -> i32 {
    let station = get_station(input);

    /*
     * Why are we counting this again when `get_station` alreay counts this?
     * There's no way to pass information from part 1 to part 2 
     * (maybe I shouldn't have gotten lazy by using someone else's library to solve puzzles...)
     * so "find the station" part is separated into another function,
     * and I guess it could also return the number of asteroids seen but
     * that's kind of weird?
     */
    let mut seen_slopes: HashSet<Vec2D> = HashSet::new();
    for asteroid in input {
        if asteroid == &station { continue; }
        seen_slopes.insert(slope(asteroid, &station));
    }
    seen_slopes.len() as i32
}

#[aoc(day10, part2)]
fn solve_part2(input: &[Vec2D]) -> i32 {
    let station = get_station(input);

    /*
     * Vector of 3-tuple containing:
     *  Position, Vector to station, and boolean indicating if destroyed
     */
    let mut asteroids: Vec<(Vec2D, Vec2D, bool)> = Vec::new();
    for asteroid in input {
        if asteroid == &station { continue; }
        asteroids.push((asteroid.clone(), slope(asteroid, &station), false));
    }
    // Use the vector from asteroid to station to sort the asteroids
    // clockwise from 12'o clock from the station.
    asteroids.sort_by(|a, b| (a.1).cmp(&b.1));

    let mut destroyed_count = 0;
    let mut index = 0;
    loop {
        asteroids[index].2 = true;
        destroyed_count += 1;
        let last_destroyed = &asteroids[index].0;
        let last_slope = &asteroids[index].1;
        if destroyed_count == 200 { return last_destroyed.x * 100 + last_destroyed.y; }
        // Skip asteroids that are either destroyed or
        // collinear to the asteroid that we just destroyed
        // (Interestingly the 200th asteroid is destroyed before the first rotation though)
        while asteroids[index].2 || &asteroids[index].1 == last_slope {
            index = (index + 1) % asteroids.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = ".#..#\n\
                .....\n\
                #####\n\
                ....#\n\
                ...##";
            
            assert_eq!(solve_part1(&parse(input)), 8);
        }
        {
            let input = "......#.#.\n\
                #..#.#....\n\
                ..#######.\n\
                .#.#.###..\n\
                .#..#.....\n\
                ..#....#.#\n\
                #..#....#.\n\
                .##.#..###\n\
                ##...#..#.\n\
                .#....####";
            assert_eq!(solve_part1(&parse(input)), 33);
        }
        {
            let input = ".#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##";
            assert_eq!(solve_part1(&parse(input)), 210);
        }
    }

    #[test]
    fn part2() {
        {
            let input = ".#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##";
            assert_eq!(solve_part2(&parse(input)), 802);
        }
    }
}
