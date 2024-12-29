enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl Opcode {
    fn from_i32(value: i32) -> Result<Opcode, String> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            99 => Ok(Opcode::Halt),
            _ => Err("Unknown opcode".into()),
        }
    }
}

pub struct IntCode {
    pub program: Vec<i32>,
}

impl IntCode {
    pub fn new(program: Vec<i32>) -> IntCode {
        IntCode { program }
    }

    pub fn from_str(input: &str) -> IntCode {
        let program = input.split(',').map(|x| x.parse().unwrap()).collect();
        IntCode::new(program)
    }

    pub fn run(&mut self) -> Result<i32, String> {
        let mut index = 0;
        loop {
            let operation = Opcode::from_i32(self.program[index])?;
            match operation {
                Opcode::Add => {
                    self.add(index);
                }
                Opcode::Multiply => {
                    self.multiply(index);
                }
                Opcode::Halt => break,
            }
            index += 4;
        }

        Ok(self.program[0])
    }

    fn add(&mut self, index: usize) {
        let i1 = self.program[index + 1] as usize;
        let i2 = self.program[index + 2] as usize;
        let i3 = self.program[index + 3] as usize;
        self.program[i3] = self.program[i1] + self.program[i2];
    }

    fn multiply(&mut self, index: usize) {
        let i1 = self.program[index + 1] as usize;
        let i2 = self.program[index + 2] as usize;
        let i3 = self.program[index + 3] as usize;
        self.program[i3] = self.program[i1] * self.program[i2];
    }
}
