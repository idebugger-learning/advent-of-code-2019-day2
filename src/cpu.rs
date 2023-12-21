pub struct CPU {
    memory: Vec<isize>,
    ip: usize,
    halted: bool,
}

impl CPU {
    pub fn new(program: Vec<isize>) -> Self {
        CPU {
            memory: program,
            ip: 0,
            halted: false,
        }
    }

    pub fn run(&mut self, param1: isize, param2: isize) -> isize {
        self.memory[1] = param1;
        self.memory[2] = param2;

        while !self.halted {
            self.step();
        }

        self.memory[0]
    }

    fn step(&mut self) {
        match self.memory[self.ip] {
            1 => self.step_add(),
            2 => self.step_mul(),
            99 => self.step_halt(),
            opcode => panic!("Unknown opcode {}", opcode),
        }
    }

    fn step_add(&mut self) {
        let operand1 = self.memory[self.memory[self.ip + 1] as usize];
        let operand2 = self.memory[self.memory[self.ip + 2] as usize];

        let target_position = self.memory[self.ip + 3] as usize;
        self.memory[target_position] = operand1 + operand2;

        self.ip += 4;
    }

    fn step_mul(&mut self) {
        let operand1 = self.memory[self.memory[self.ip + 1] as usize];
        let operand2 = self.memory[self.memory[self.ip + 2] as usize];

        let target_position = self.memory[self.ip + 3] as usize;
        self.memory[target_position] = operand1 * operand2;

        self.ip += 4;
    }

    fn step_halt(&mut self) {
        self.halted = true;
    }
}
