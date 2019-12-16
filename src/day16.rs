/*
 * Advent of Code 2019 Day 16
 *
 * Flawed Frequency Transmission
 *
 * Remarks:
 *  A bit rough.
 */

#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|n| n.to_digit(10).unwrap()).collect()
}

#[aoc(day16, part1)]
fn solve_part1(input: &Vec<u32>) -> u32 {

    fn fft(input: &mut Vec<u32>) {
        let pattern = [0, 1, 0, -1];
        let mut output: Vec<u32> = Vec::with_capacity(input.len());
        output.resize(input.len(), 0);
        output[input.len() - 1] = input[input.len() - 1];

        let half = input.len() / 2;
        for i in 0..half {
            let mut result = 0;

            for (pos, &digit) in input.iter().enumerate() {
                let p = (pos + 1) / (i + 1);
                result += digit as i32 * pattern[p % 4];
            }

            output[i] = (result.abs() % 10) as u32;
        }
        for i in (half..input.len() - 1).rev() {
            let result = input[i] + output[i + 1];
            output[i] = result % 10;
        }
        *input = output;
    }

    let mut signal = input.clone();
    for _ in 0..100 {
        fft(&mut signal);
    }

    let result = signal[..8].iter().fold(0, |n, &d| 10 * n + d);
    result
}

#[aoc(day16, part2)]
fn solve_part2(input: &Vec<u32>) -> u32 {
    let offset = input[..7].iter().fold(0, |n, &d| 10 * n + d) as usize;
    let end_length = input.len() * 10000 - offset;

    let mut end: Vec<u32> = input.iter()
        .copied()
        .cycle()
        .skip(offset)
        .take(end_length)
        .collect();

    for _ in 0..100 {
        /*
         * Why are we just summing the digits?
         * Because latter half of the signal, the pattern looks like this:
         * [0 0 0 0 0 0 0 0 0  0 0 ....  1 1 1 1 1 1 1 1 ... 1 1 ]
         *                               ^             
         *                               offset
         *
         * And the offset happens to be in the latter half.
         */
        for i in (0..end_length - 1).rev() {
            let result = end[i] + end[i + 1];
            end[i] = result % 10;
        }
    }

    let result = end[..8].iter().fold(0, |n, &d| 10 * n + d);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            let input = "80871224585914546619083218645595";
            assert_eq!(solve_part1(&parse(input)), 24176176);
        }
        {
            let input = "19617804207202209144916044189917";
            assert_eq!(solve_part1(&parse(input)), 73745418);
        }
        {
            let input = "69317163492948606335995924319873";
            assert_eq!(solve_part1(&parse(input)), 52432133);
        }
    }

    #[test]
    fn part2() {
        {
            let input = "03036732577212944063491565474664";
            assert_eq!(solve_part2(&parse(input)), 84462026);
        }
        {
            let input = "02935109699940807407585447034323";
            assert_eq!(solve_part2(&parse(input)), 78725270);
        }
        {
            let input = "03081770884921959731165446850517";
            assert_eq!(solve_part2(&parse(input)), 53553731);
        }
    }
}
