/*
 * Advent of Code 2019 Day 6
 *
 * Universal Orbit Map
 */

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

#[aoc_generator(day6)]
fn parse(input: &str) -> HashMap<String, Vec<String>>  {
    let re = Regex::new(r"^([A-Z0-9]{0,})\)([A-Z0-9]{0,})$").unwrap();

    let orbits: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
            )
        }).collect();

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for orbit in orbits {
        match tree.get_mut(&orbit.0) {
            None => { tree.insert(orbit.0, vec![orbit.1]); }
            Some (o) => o.push(orbit.1),
        }
    }
    tree
}

fn count_orbits(tree: &HashMap<String, Vec<String>>, s: &str, orbits: i32) -> i32 {
    let mut orbits = orbits;
    match tree.get(s) {
        Some(suborbits) => {
            let mut subtree_orbits_sum = 0;
            for sub in suborbits {
                subtree_orbits_sum += count_orbits(tree, sub, orbits + 1);
            }
            orbits += subtree_orbits_sum;
        }
        None => (),
    }
    orbits
}

#[aoc(day6, part1)]
fn solve_part1(tree: &HashMap<String, Vec<String>>) -> i32 {
    count_orbits(tree, "COM", 0)
}

/*
 * Returns the name of the least common ancestor of a and b, with root as the root node.
 * Returns None if there is no such common ancestor.
 */
fn find_lca(tree: &HashMap<String, Vec<String>>, root: &str, a: &str, b: &str) -> Option<String> {
    if root == a || root == b {
        return Some(root.into());
    }

    // If the root has no children and the root is not a or b (checked above), then there is no LCA.
    let children = match tree.get(root) {
        None => return None,
        Some(a) => a,
    };

    let mut lca_candidates: HashSet<Option<String>> = HashSet::new();
    for child in children {
        lca_candidates.insert(find_lca(tree, child, &a, &b));
    }

    if lca_candidates.contains(&Some(a.into())) &&
       lca_candidates.contains(&Some(b.into())) {
        return Some(root.into());
    }

    for candidate in lca_candidates {
        if candidate != None {
            return candidate;
        }
    }
    None
}

fn get_distance(tree: &HashMap<String, Vec<String>>, root: &str, needle: &str) -> i32 {
    let mut v: VecDeque<(String, i32)> = VecDeque::new();
    v.push_back((root.into(), 0));

    while !v.is_empty() {
        let current = v.pop_front().unwrap();
        if current.0 == needle {
            return current.1;
        }
        else {
            if let Some(sub) = tree.get(&current.0) {
                for node in sub { v.push_back((node.into(), current.1 + 1)); }
            }
        }
    }
    panic!("Could not find {}", needle);
}

#[aoc(day6, part2)]
fn solve_part2(tree: &HashMap<String, Vec<String>>) -> i32 {
    let lca = find_lca(tree, "COM", "SAN", "YOU").unwrap();

    let santa_distance = get_distance(tree, &lca, "SAN");
    let you_distance = get_distance(tree, &lca, "YOU");

    // Subtract 2 because we don't count the direct orbits from YOU and SAN
    santa_distance + you_distance - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = parse("COM)A\nCOM)B");
            assert_eq!(solve_part1(&input), 2);
        }
        {
            let input = parse("COM)A\nA)B\nB)C");
            assert_eq!(solve_part1(&input), 6);
        }
        {
            let input = parse("COM)A\nA)B\nB)C\nB)D");
            assert_eq!(solve_part1(&input), 9);
        }
        {
            let input = parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
            assert_eq!(solve_part1(&input), 42);
        }
    }

    #[test]
    fn part2() {
        {
            let input = parse("COM)SAN\nCOM)YOU");
            assert_eq!(solve_part2(&input), 0);
        }
        {
            let input = parse("COM)A\nCOM)B\nA)SAN\nB)YOU");
            assert_eq!(solve_part2(&input), 2);
        }
        {
            let input = parse("COM)A\nCOM)B\nA)YOU\nB)C\nB)D\nD)E\nE)SAN\nE)F");
            assert_eq!(solve_part2(&input), 4);
        }
        {
            let input = parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
            assert_eq!(solve_part2(&input), 4);
        }
    }
}
