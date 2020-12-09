use itertools::Itertools;

#[derive(Debug)]
pub enum ParseError {
    ImmediateParseError(std::num::ParseIntError),
    UnknownInstruction,
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub memory: Vec<Instruction>,
    pub pc: usize,
    pub acc: isize,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ins_len = s.find(' ').unwrap_or(s.len());
        match &s[..ins_len] {
            "nop" => Ok(Instruction::Nop(
                s[ins_len + 1..]
                    .parse()
                    .map_err(ParseError::ImmediateParseError)?,
            )),
            "acc" => Ok(Instruction::Acc(
                s[ins_len + 1..]
                    .parse()
                    .map_err(ParseError::ImmediateParseError)?,
            )),
            "jmp" => Ok(Instruction::Jmp(
                s[ins_len + 1..]
                    .parse()
                    .map_err(ParseError::ImmediateParseError)?,
            )),
            _ => Err(ParseError::UnknownInstruction),
        }
    }
}

impl std::str::FromStr for Computer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Vec<Instruction> = s.split_terminator('\n').map(|l| l.parse()).try_collect()?;
        Ok(Computer {
            memory,
            pc: 0,
            acc: 0,
        })
    }
}

impl Computer {
    pub fn step(&mut self) {
        if self.pc >= self.memory.len() {
            panic!("pc out of range");
        }

        match self.memory[self.pc] {
            Instruction::Nop(_) => self.pc += 1,
            Instruction::Acc(n) => {
                self.pc += 1;
                self.acc += n;
            }
            Instruction::Jmp(n) => {
                self.add_to_pc(n);
            }
        }
    }

    pub fn run_until_loop(&mut self) {
        let mut visited = vec![false; self.len()];
        while self.pc < self.len() && !visited[self.pc] {
            visited[self.pc] = true;
            self.step();
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn acc(&self) -> isize {
        self.acc
    }

    pub fn memory(&self) -> &Vec<Instruction> {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.memory
    }

    fn add_to_pc(&mut self, n: isize) {
        if n < 0 {
            self.pc -= n.wrapping_neg() as usize
        } else {
            self.pc += n as usize
        }
    }
}
