use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCode {
    pub ip: usize,
    pub relative_base: i64,
    pub mem: Vec<i64>,
    pub input_queue: VecDeque<i64>
}

pub enum Status {
    Output(i64),
    Halt,
    WaitingInput,
}

impl IntCode {

    /*
     * Returns a Vector containinStatus indicies for the arStatusuments of the current opcode.
     */
    fn get_args(&self) -> Vec<usize> {
        let instruction = self.mem[self.ip];
        let opcode = instruction % 100;
        let modes = [(instruction / 100) % 10, (instruction / 1000) % 10, (instruction / 10000) % 10];

        let num_params = match opcode {
            1 => 3, // {3} = {1} + {2}
            2 => 3, // {3} = {1} * {2}
            3 => 1, // {1} = input
            4 => 1, // output {1}
            5 => 2, // jump to {2} if {1} != 0
            6 => 2, // jump to {2} if {1} == 0
            7 => 3, // {3} = ({1} < {2})
            8 => 3, // {3} = ({1} == {2})
            9 => 1, // relative_base = {1}
            99 => 0, // halt
            _ => panic!("Invalid opcode {}", opcode),
        };

        let mut args: Vec<usize> = Vec::new();
        for (i, mode) in modes.iter().enumerate().take(num_params) {
            let x = self.mem[self.ip + 1 + i];
            args.push(match mode {
                0 => x as usize, // Address mode
                1 => self.ip + 1 + i, // Immediate mode
                2 => (self.relative_base + x) as usize, // Relative mode
                x => panic!("Unknown parameter mode {}", x),
            });
        }
        args
    }

    pub fn execute(&mut self) -> Status {
        loop {
            let args = self.get_args();
            let instruction = self.mem[self.ip];
            let opcode = instruction % 100;

            let mut new_ip = self.ip + args.len() + 1;
            match opcode {
                1 => self.mem[args[2]] = self.mem[args[0]] + self.mem[args[1]],
                2 => self.mem[args[2]] = self.mem[args[0]] * self.mem[args[1]],
                3 => {
                    match self.input_queue.pop_front() {
                        Some(input) => self.mem[args[0]] = input,
                        None => return Status::WaitingInput,
                    }
                }
                4 => {
                    self.ip = new_ip;
                    return Status::Output(self.mem[args[0]]);
                }
                5 => if self.mem[args[0]] != 0 { new_ip = self.mem[args[1]] as usize },
                6 => if self.mem[args[0]] == 0 { new_ip = self.mem[args[1]] as usize },
                7 => self.mem[args[2]] = (self.mem[args[0]] < self.mem[args[1]]) as i64,
                8 => self.mem[args[2]] = (self.mem[args[0]] == self.mem[args[1]]) as i64,
                9 => self.relative_base += self.mem[args[0]],
                99 => return Status::Halt,
                _ => panic!("Invalid opcode {}", opcode),
            }
            self.ip = new_ip;
        }
    }
}
