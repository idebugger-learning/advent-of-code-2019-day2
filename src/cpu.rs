pub struct CPU {
    program: Vec<isize>,
    ip: usize,
    halted: bool,
}

impl CPU {
    pub fn new(program: Vec<isize>) -> Self {
        CPU {
            program,
            ip: 0,
            halted: false,
        }
    }

    pub fn get_first_value(&self) -> isize {
        self.program[0]
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.program[self.ip] {
            1 => self.step_add(),
            2 => self.step_mul(),
            99 => self.step_halt(),
            opcode => panic!("Unknown opcode {}", opcode),
        }
    }

    fn step_add(&mut self) {
        let operand1 = self.program[self.program[self.ip + 1] as usize];
        let operand2 = self.program[self.program[self.ip + 2] as usize];

        let target_position = self.program[self.ip + 3] as usize;
        self.program[target_position] = operand1 + operand2;

        self.ip += 4;
    }

    fn step_mul(&mut self) {
        let operand1 = self.program[self.program[self.ip + 1] as usize];
        let operand2 = self.program[self.program[self.ip + 2] as usize];

        let target_position = self.program[self.ip + 3] as usize;
        self.program[target_position] = operand1 * operand2;

        self.ip += 4;
    }

    fn step_halt(&mut self) {
        self.halted = true;
    }
}
