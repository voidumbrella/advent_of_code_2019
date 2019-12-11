/*
 * Advent of Code 2019 Day 2
 *
 * 1202 Program Alarm
 *
 * Remarks:
 * The output turns out to be 250702 + 243000 * noun + verb,
 * but analyzing that would've taken much longer than just brute forcing it. :3
 * 
 * The question hinted at adding more functionality to our IntCode machine,
 * so I ended up abstracting more things than strictly required.
 */

#[derive(Clone)]
struct IntCode {
    ip: usize,
    mem: Vec<i32>
}

#[aoc_generator(day2)]
fn parse(input: &str) -> IntCode {
    IntCode {
        ip: 0,
        mem: input
            .split(',')
            .map(|n| { n.parse().unwrap() })
            .collect()
    }
} 

impl IntCode {
    fn get_args(&self) -> Vec<i32> {
        let opcode = self.mem[self.ip];
        let num_params;

        match opcode {
            1 => { num_params = 3; }
            2 => { num_params = 3; }
            99 => { num_params = 0; }
            _ => panic!("Invalid opcode {}", opcode),
        }

        self.mem[self.ip + 1..self.ip + 1 + num_params].to_vec()
    }

    pub fn execute(&mut self) {
        loop {
            let args = self.get_args();
            let opcode = self.mem[self.ip];

            match opcode {
                1 => { self.mem[args[2] as usize] = self.mem[args[0] as usize] + self.mem[args[1] as usize]; },
                2 => { self.mem[args[2] as usize] = self.mem[args[0] as usize] * self.mem[args[1] as usize]; },
                99 => break,
                _ => panic!("Invalid opcode {}", opcode),
            }

            self.ip += args.len() + 1;
        }
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &IntCode) -> i32 {
    let mut program = input.clone();
    program.mem[0] = 12;
    program.mem[1] = 2;
    program.execute();
    program.mem[0]
}

#[aoc(day2, part2)]
fn solve_part2(input: &IntCode) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input.clone();
            program.mem[0] = noun;
            program.mem[1] = verb;
            program.execute();
            if program.mem[0] == 19_690_720 {
                return 100 * noun + verb
            }
        }
    }
    panic!("Could not find suitable inputs!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        {
            {
                let mut program = parse("1,0,0,0,99");
                program.execute();
                assert_eq!(program.mem[0], 2);
            }
            {
                let mut program = parse("2,3,0,3,99");
                program.execute();
                assert_eq!(program.mem[3], 6);
            }
            {
                let mut program = parse("1,1,1,4,99,5,6,0,99");
                program.execute();
                assert_eq!(program.mem[0], 30);
            }
            {
                let mut program = parse("1,9,10,3,2,3,11,0,99,30,40,50");
                program.execute();
                assert_eq!(program.mem[0], 3500);
            }
        }
    }

    #[test]
    fn part2() {
    }
}
