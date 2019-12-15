/*
 * Advent of Code 2019 Day 14
 *
 * Space Stoichiometry
 *
 * Remarks:
 *  ???
 */

use regex::Regex;
use std::collections::HashMap;

type Reactions = HashMap<String, (i32, Vec<(String, i32)>)>;

#[aoc_generator(day14)]
fn parse(input: &str) -> Reactions {
    let re = Regex::new(r"([0-9]{1,}) ([A-Z]{1,})").unwrap();

    let mut reactions: Reactions = HashMap::new();
    for line in input.lines() {
        let v: Vec<&str> = line.split("=>").collect();

        let caps = re.captures(v[1]).unwrap();
        let product = caps[2].parse().unwrap();
        let num_produced = caps[1].parse().unwrap();

        let mut reagents: Vec<(String, i32)> = Vec::new();
        for reagent in v[0].split(",") {
            let caps = re.captures(reagent).unwrap();
            reagents.push((caps[2].parse().unwrap(), caps[1].parse().unwrap()));
        }
        reactions.insert(product, (num_produced, reagents));
    }
    reactions
}

#[aoc(day14, part1)]
fn solve_part1(input: &Reactions) -> i32 {
    // Stores chemicals that must be produced
    let mut goal: HashMap<String, i32> = HashMap::new();
    // Stores chemicals that were produced as excess
    let mut pool: HashMap<String, i32> = HashMap::new();
    goal.insert("FUEL".to_string(), 1);
    let mut total_ore = 0;

    while !goal.is_empty() {
        let mut new_goal: HashMap<String, i32> = HashMap::new();
        
        for (product, &needed) in goal.iter() {
            let mut needed = needed;
            if let Some(&left) = pool.get(product) {
                needed -= left;
                pool.insert(product.to_string(), std::cmp::max(0, left - needed));
            }
            if needed <= 0 { continue; }

            let (product_coef, reagents) = input.get(product).unwrap();
            let num_reactions = (needed + product_coef - 1) / product_coef; // ceiling div
            let num_produced = num_reactions * product_coef;

            let excess = num_produced - needed;
            if excess > 0 {
                // Add any excess products to pool
                *pool.entry(product.to_string()).or_insert(0) += excess;
            }

            for (reagent, coef) in reagents {
                let num_reagent = coef * num_reactions;
                if reagent == "ORE" {
                    total_ore += num_reagent;
                    continue;
                } 
                let left = match pool.get(reagent) {
                    None => 0,
                    Some(&x) => x,
                };
                *new_goal.entry(reagent.to_string()).or_insert(0) += num_reagent - left;
                pool.insert(reagent.to_string(), std::cmp::max(0, left - num_reagent));
            }
        }
        goal = new_goal;
    }
    total_ore
}

// #[aoc(day14, part2)]
// fn solve_part2(input: &i32) -> i32 {
    // input
        // .iter()
        // .map(|| {
        // })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = "10 ORE => 10 A\n\
            1 ORE => 1 B\n\
            7 A, 1 B => 1 C\n\
            7 A, 1 C => 1 D\n\
            7 A, 1 D => 1 E\n\
            7 A, 1 E => 1 FUEL";
            assert_eq!(solve_part1(&parse(input)), 31);
        }
        {
            let input = "9 ORE => 2 A\n\
            8 ORE => 3 B\n\
            7 ORE => 5 C\n\
            3 A, 4 B => 1 AB\n\
            5 B, 7 C => 1 BC\n\
            4 C, 1 A => 1 CA\n\
            2 AB, 3 BC, 4 CA => 1 FUEL";
            assert_eq!(solve_part1(&parse(input)), 165);
        }
        {
            let input = "157 ORE => 5 NZVS\n\
            165 ORE => 6 DCFZ\n\
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
            179 ORE => 7 PSHF\n\
            177 ORE => 5 HKGWZ\n\
            7 DCFZ, 7 PSHF => 2 XJWVT\n\
            165 ORE => 2 GPVTF\n\
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
            assert_eq!(solve_part1(&parse(input)), 13312);
        }
        {
            let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
            17 NVRVD, 3 JNWZP => 8 VPVL\n\
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
            22 VJHF, 37 MNCFX => 5 FWMGM\n\
            139 ORE => 4 NVRVD\n\
            144 ORE => 7 JNWZP\n\
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
            145 ORE => 6 MNCFX\n\
            1 NVRVD => 8 CXFTF\n\
            1 VJHF, 6 MNCFX => 4 RFSQX\n\
            176 ORE => 6 VJHF";
            assert_eq!(solve_part1(&parse(input)), 180697);
        }
        {
            let input = "171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX";
            assert_eq!(solve_part1(&parse(input)), 2210736);
        }
    }

    #[test]
    fn part2() {
        {
        }
    }
}
