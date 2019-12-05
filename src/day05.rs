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
    mem: Vec<i32>
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
    fn get_args(&self) -> Vec<i32> {
        let instruction = self.mem[self.ip];
        let opcode = instruction % 100;
        let mut modes = [(instruction / 100) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10];

        /*
         * Hard coded hack
         *
         * The problem says the parameter for the instruction writes to will never be in immediate mode.
         * But honestly, it makes everything simpler if they were immediate mode,
         * so I flip the mode manually.
         *
         * Rationale:
         *  Let's say we have a sample program `1101,100,-1,4,0` from the problem.
         *  The `01` instruction is writing to address `4`, which is value given immediately!!
         *  If it was in position mode (11101), then we'd be writing to address `0` because
         *  that is the value at position `4`.
         */
        match opcode {
            1 => modes[2] = 1,
            2 => modes[2] = 1,
            3 => modes[0] = 1,
            7 => modes[2] = 1,
            8 => modes[2] = 1,
            _ => (),
        }

        let num_params = match opcode {
            1 => 3, // {1} + {2} = {3}
            2 => 3, // {1} * {2} = {3}
            3 => 1, // {1} = input
            4 => 1, // output {4}
            5 => 2, // jump to {2} if {1} != 0
            6 => 2, // jump to {2} if {1} == 0
            7 => 3, // {1} < {2} = {3}
            8 => 3, // {1} == {2} = {3}
            99 => 0, // halt
            _ => panic!("Invalid opcode {}", opcode),
        };

        let mut args: Vec<i32> = Vec::new();
        for i in 0..num_params {
            let x = self.mem[self.ip + 1 + i];
            args.push(if modes[i] == 0 {self.mem[x as usize]} else { x });
        }
        args
    }

    pub fn execute(&mut self, input: i32) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();
        loop {
            let args = self.get_args();
            let instruction = self.mem[self.ip];
            let opcode = instruction % 100;

            self.ip += args.len() + 1;
            match opcode {
                1 => self.mem[args[2] as usize] = args[0] + args[1],
                2 => self.mem[args[2] as usize] = args[0] * args[1],
                3 => self.mem[args[0] as usize] = input,
                4 => outputs.push(args[0]),
                5 => if args[0] != 0 { self.ip = args[1] as usize },
                6 => if args[0] == 0 { self.ip = args[1] as usize },
                7 => self.mem[args[2] as usize] = (args[0] < args[1]) as i32,
                8 => self.mem[args[2] as usize] = (args[0] == args[1]) as i32,
                99 => break,
                _ => panic!("Invalid opcode {}", opcode),
            }
        }
        outputs
    }
}

#[aoc(day5, part1)]
fn solve_part1(input: &IntCode) -> i32 {
    let mut program = input.clone();
    let outputs = program.execute(1);
    println!("program outputs: {:?}", outputs);
    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &IntCode) -> i32 {
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
