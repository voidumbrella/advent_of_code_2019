/*
 * Advent of Code 2019 Day 24
 *
 * Planet of Discord
 *
 * Remarks:
 *  My Part 2 solution runs in ~200ms which is pretty meh,
 *  but the problem was essentially implementing the automata according to the spec
 *  which wasn't that bad.
 *
 *  If I wanted to optimize it I'd probably start with flattening that vector of vectors,
 *  maybe in the future if I have nothing better to do?
 */

use std::collections::HashSet;
use std::collections::HashMap;

type Bugs = Vec<Vec<bool>>;

#[aoc_generator(day24)]
fn parse(input: &str) -> Bugs {
    let mut bugs = vec![vec![false; 5]; 5];
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' { bugs[y][x] = true; }
            x += 1;
        }
        y += 1;
    }
    bugs
}

#[aoc(day24, part1)]
fn solve_part1(input: &Bugs) -> i64 {
    fn count_neighbors(bugs: &Bugs, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let mut adjacent_tiles = Vec::new();
        if x > 0 { adjacent_tiles.push(bugs[y][x - 1]); }
        if x < 4 { adjacent_tiles.push(bugs[y][x + 1]); }
        if y > 0 { adjacent_tiles.push(bugs[y - 1][x]); }
        if y < 4 { adjacent_tiles.push(bugs[y + 1][x]); }
        for &tile in adjacent_tiles.iter() {
            if tile { count += 1; }
        }
        count
    }

    fn update(bugs: &mut Bugs) {
        let mut new_bugs = vec![vec![false; 5]; 5];
        for y in 0..5 {
            for x in 0..5 {
                let neighbors = count_neighbors(bugs, x, y);
                if bugs[y][x] {
                    new_bugs[y][x] = if neighbors == 1 {true} else {false};
                } else {
                    new_bugs[y][x] = if neighbors == 1 || neighbors == 2 {true} else {false};
                }
            }
        }
        *bugs = new_bugs
    }

    fn biodiversity_rating(bugs: &Bugs) -> i64 {
        let mut point = 1;
        let mut total_rating = 0;
        for y in 0..5 {
            for x in 0..5 {
                if bugs[y][x] {
                    total_rating += point;
                }
                point *= 2;
            }
        }
        total_rating
    }

    let mut ratings = HashSet::new();
    let mut bugs = input.clone();
    loop {
        let rating = biodiversity_rating(&bugs);
        // insert() returns false if the value was present in the set
        if !ratings.insert(rating) { return rating; }
        update(&mut bugs);
    }
}

#[aoc(day24, part2)]
fn solve_part2(input: &Bugs) -> i32 {
    fn count_neighbors(dimensions: &HashMap<isize, Bugs>, i: isize, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let cur = dimensions.get(&i).unwrap();
        let upper_maybe = dimensions.get(&(i + 1));
        let lower_maybe = dimensions.get(&(i - 1));
        let mut adjacent_tiles = Vec::new();

        // Add adjacent tiles on the current dimension.
        // If on the edge, then add the tiles from the upper dimension instead.
        if x > 0 { adjacent_tiles.push(cur[y][x - 1]); }
        else if let Some(upper) = upper_maybe { adjacent_tiles.push(upper[2][1]); }

        if x < 4 { adjacent_tiles.push(cur[y][x + 1]); }
        else if let Some(upper) = upper_maybe { adjacent_tiles.push(upper[2][3]); }

        if y > 0 { adjacent_tiles.push(cur[y - 1][x]); }
        else if let Some(upper) = upper_maybe { adjacent_tiles.push(upper[1][2]); }

        if y < 4 { adjacent_tiles.push(cur[y + 1][x]); }
        else if let Some(upper) = upper_maybe { adjacent_tiles.push(upper[3][2]); }

        // If adjacent to the hole, then count the tiles from the lower dimension as well.
        if let Some(lower) = lower_maybe {
            if x == 2 && y == 1 {
                for x in 0..5 { adjacent_tiles.push(lower[0][x]); }
            } else if x == 2 && y == 3 {
                for x in 0..5 { adjacent_tiles.push(lower[4][x]); }
            } else if x == 1 && y == 2 {
                for y in 0..5 { adjacent_tiles.push(lower[y][0]); }
            } else if x == 3 && y == 2 {
                for y in 0..5 { adjacent_tiles.push(lower[y][4]); }
            }
        }

        for &tile in adjacent_tiles.iter() {
            if tile { count += 1; }
        }
        count
    }

    fn update(dimensions: &mut HashMap<isize, Bugs>) {
        let max_dimension = (dimensions.len() / 2) as isize;
        dimensions.insert(max_dimension + 1, vec![vec![false; 5]; 5]);
        dimensions.insert(-(max_dimension + 1) , vec![vec![false; 5]; 5]);

        let mut new_dimensions = dimensions.clone();
        for (&n, cur) in dimensions.iter() {
            let mut new_bugs = vec![vec![false; 5]; 5];
            for y in 0..5 {
                for x in 0..5 {
                    // Skip the center hole to another dimension
                    if x == 2 && y == 2 { continue; }
                    let neighbors = count_neighbors(dimensions, n, x, y);
                    if cur[y][x] {
                        new_bugs[y][x] = if neighbors == 1 {true} else {false};
                    } else {
                        new_bugs[y][x] = if neighbors == 1 || neighbors == 2 {true} else {false};
                    }
                }
            }
            new_dimensions.insert(n, new_bugs);
        }
        *dimensions = new_dimensions;
    }

    let mut bugs = HashMap::new();
    bugs.insert(0, input.clone());
    for _ in 0..200 { update(&mut bugs); }

    let mut num_bugs = 0;
    for dimension in bugs.values() {
        for &bug in dimension.iter().flatten() {
            if bug { num_bugs += 1; }
        }
    }
    num_bugs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = "....#\n\
            #..#.\n\
            #..##\n\
            ..#..\n\
            #....";
            assert_eq!(solve_part1(&parse(input)), 2129920);
        }
    }
}
