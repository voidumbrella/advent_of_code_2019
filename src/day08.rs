/*
 * Advent of Code 2019 Day 8
 *
 * Space Image Format
 *
 * Remarks:
 *  Eh.
 */

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<String>  {
    input.as_bytes()
        .chunks(WIDTH * HEIGHT)
        .map(|s| { String::from_utf8(s.to_vec()).unwrap() })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(layers: &Vec<String>) -> i32 {
    let mut min = (std::usize::MAX, 0, 0);
    for layer in layers {
        let mut count = (0, 0, 0);
        for c in layer.chars() {
            match c {
                '0' => count.0 += 1,
                '1' => count.1 += 1,
                '2' => count.2 += 1,
                _ => panic!("layer contains invalid character"),
            };
        }
        if count.0 < min.0 { min = count; }
    }
    (min.1 * min.2) as i32
}

#[aoc(day8, part2)]
fn solve_part2(layers: &Vec<String>) -> &'static str {
    let mut image: [i32; WIDTH * HEIGHT] = [2; WIDTH * HEIGHT];

    for layer in layers {
        for (i, c) in layer.chars().enumerate() {
            match c {
                '0' => if image[i] == 2 { image[i] = 0; },
                '1' => if image[i] == 2 { image[i] = 1; },
                '2' => (),
                _ => panic!("layer contains invalid character"),
            };
        }
    }

    for (i, n) in image.iter().enumerate() {
        match n {
            0 => print!(" "),
            1 => print!("*"),
            _ => panic!("image contains invalid character"),
        }
        if (i + 1) % WIDTH == 0 {println!("");}
    }
    "Check output"
}
