/*
 * Advent of Code 2019 Day 5
 *
 * Sunny with a Chance of Asteroids
 *
 * Remarks:
 *  Surprisingly simple?
 *  Dealing with the immediate/position mode shenanigan was tricky though.
 */

#[derive(Clone)]
struct IntCode {
    ip: usize,
    mem: Vec<i64>
}

#[aoc_generator(day5)]
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

    /*
     * Returns a Vector containing indicies for the arguments of the current opcode.
     */
    fn get_args(&self) -> Vec<usize> {
        let instruction = self.mem[self.ip];
        let opcode = instruction % 100;
        let modes = [(instruction / 100) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10];

        let num_params = match opcode {
            1 => 3, // {3} = {1} + {2}
            2 => 3, // {3} = {1} * {2}
            3 => 1, // {1} = input
            4 => 1, // output {4}
            5 => 2, // jump to {2} if {1} != 0
            6 => 2, // jump to {2} if {1} == 0
            7 => 3, // {3} = ({1} < {2})
            8 => 3, // {3} = ({1} == {2})
            99 => 0, // halt
            _ => panic!("Invalid opcode {}", opcode),
        };

        let mut args: Vec<usize> = Vec::new();
        for i in 0..num_params {
            let x = self.mem[self.ip + 1 + i];
            args.push(match modes[i] {
                0 => x as usize, // Address mode
                1 => self.ip + 1 + i, // Immediate mode
                x => panic!("Unknown parameter mode {}", x),
            });
        }
        args
    }

    pub fn execute(&mut self, input: i64) -> Vec<i64> {
        let mut outputs: Vec<i64> = Vec::new();
        loop {
            let args = self.get_args();
            let instruction = self.mem[self.ip];
            let opcode = instruction % 100;

            self.ip += args.len() + 1;
            match opcode {
                1 => self.mem[args[2]] = self.mem[args[0]] + self.mem[args[1]],
                2 => self.mem[args[2]] = self.mem[args[0]] * self.mem[args[1]],
                3 => self.mem[args[0]] = input,
                4 => outputs.push(self.mem[args[0]]),
                5 => if self.mem[args[0]] != 0 { self.ip = self.mem[args[1]] as usize },
                6 => if self.mem[args[0]] == 0 { self.ip = self.mem[args[1]] as usize },
                7 => self.mem[args[2]] = (self.mem[args[0]] < self.mem[args[1]]) as i64,
                8 => self.mem[args[2]] = (self.mem[args[0]] == self.mem[args[1]]) as i64,
                99 => break,
                _ => panic!("Invalid opcode {}", opcode),
            }
        }
        outputs
    }
}

#[aoc(day5, part1)]
fn solve_part1(input: &IntCode) -> i64 {
    let mut program = input.clone();
    let outputs = program.execute(1);
    println!("program outputs: {:?}", outputs);
    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &IntCode) -> i64 {
    let mut program = input.clone();
    let outputs = program.execute(5);
    println!("program outputs: {:?}", outputs);
    *outputs.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut program = parse("1002,4,3,4,33");
        program.execute(0);
        assert_eq!(program.mem[4], 99);
    }

    #[test]
    fn part2() {
        {
            let program = parse("3,9,8,9,10,9,4,9,99,-1,8");
            assert_eq!(program.clone().execute(2), [0]);
            assert_eq!(program.clone().execute(8), [1]);
        }
        {
            let program = parse("3,9,7,9,10,9,4,9,99,-1,8");
            assert_eq!(program.clone().execute(2), [1]);
            assert_eq!(program.clone().execute(8), [0]);
            assert_eq!(program.clone().execute(9), [0]);
        }
        {
            let program = parse("3,3,1108,-1,8,3,4,3,99");
            assert_eq!(program.clone().execute(2), [0]);
            assert_eq!(program.clone().execute(8), [1]);
        }
        {
            let program = parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
            assert_eq!(program.clone().execute(2), [999]);
            assert_eq!(program.clone().execute(8), [1000]);
            assert_eq!(program.clone().execute(800), [1001]);
        }
    }
}
