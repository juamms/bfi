use std::collections::HashMap;
use std::io::{self, Read};

#[derive(PartialEq, Debug)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    LoopStart,
    LoopEnd,
    Read,
    Write,
}

pub struct Machine {
    program: Vec<Instruction>,
    data: Vec<u8>,
    jump_table: HashMap<usize, usize>,
    instruction_pointer: usize,
    data_pointer: usize,
}

/// Machine setup
impl Machine {
    pub fn new(size: usize) -> Machine {
        Machine {
            program: Vec::new(),
            data: vec![0; size],
            jump_table: HashMap::new(),
            instruction_pointer: 0,
            data_pointer: 0,
        }
    }

    pub fn load_program(&mut self, raw_program: Vec<char>, optimise: bool) {
        if optimise {
            self.load_optimised(raw_program);
        } else {
            self.load_unoptimised(raw_program);
        }

        self.load_jump_table();
    }

    pub fn dump(&self) {
        println!("{:?}", self.data[0..32].to_vec());
        println!("DP: {}", self.data_pointer);
        println!("IP: {}", self.instruction_pointer);
    }

    fn load_jump_table(&mut self) {
        for (idx, ins) in self.program.iter().enumerate() {
            if *ins == Instruction::LoopStart {
                let start = idx;
                let end = self.find_loop_end(start);

                self.jump_table.insert(start, end + 1);
                self.jump_table.insert(end, start + 1);
            }
        }
    }

    fn find_loop_end(&self, start: usize) -> usize {
        let mut end = start + 1;
        let mut skips = 0;

        while end < self.program.len() {
            if self.program[end] == Instruction::LoopStart {
                skips += 1;
            } else if self.program[end] == Instruction::LoopEnd {
                if skips == 0 {
                    return end;
                }

                skips -= 1;
            }

            end += 1;
        }

        panic!("Unbalanced loops detected")
    }

    fn load_optimised(&mut self, raw_program: Vec<char>) {
        let mut idx = 0;
        let mut instruction: Instruction;

        while idx < raw_program.len() {
            let token = raw_program[idx];

            if ['>', '<', '+', '-'].contains(&token) {
                let mut amount = 1;

                while idx + 1 < raw_program.len() && raw_program[idx + 1] == token {
                    idx += 1;
                    amount += 1;
                }

                instruction = match token {
                    '>' => Instruction::MoveRight(amount),
                    '<' => Instruction::MoveLeft(amount),
                    '+' => Instruction::Increment(amount as u8),
                    '-' => Instruction::Decrement(amount as u8),
                    _ => panic!(format!("Unknown token '{}'", token)),
                }
            } else {
                instruction = match token {
                    '[' => Instruction::LoopStart,
                    ']' => Instruction::LoopEnd,
                    ',' => Instruction::Read,
                    '.' => Instruction::Write,
                    _ => panic!(format!("Unknown token '{}'", token)),
                };
            }

            self.program.push(instruction);
            idx += 1;
        }
    }

    fn load_unoptimised(&mut self, raw_program: Vec<char>) {
        self.program = raw_program
            .into_iter()
            .map(|instruction| match instruction {
                '>' => Instruction::MoveRight(1),
                '<' => Instruction::MoveLeft(1),
                '+' => Instruction::Increment(1),
                '-' => Instruction::Decrement(1),
                '[' => Instruction::LoopStart,
                ']' => Instruction::LoopEnd,
                ',' => Instruction::Read,
                '.' => Instruction::Write,
                _ => panic!(format!("Unknown instruction '{}'", instruction)),
            })
            .collect()
    }
}

/// Execution
impl Machine {
    pub fn has_program_ended(&self) -> bool {
        self.instruction_pointer >= self.program.len()
    }

    pub fn run(&mut self) {
        while !self.has_program_ended() {
            self.step()
        }
    }

    pub fn step(&mut self) {
        match self.program[self.instruction_pointer] {
            Instruction::MoveRight(amount) => self.move_right(amount),
            Instruction::MoveLeft(amount) => self.move_left(amount),
            Instruction::Increment(amount) => self.increment(amount),
            Instruction::Decrement(amount) => self.decrement(amount),
            Instruction::LoopStart => self.loop_start(),
            Instruction::LoopEnd => self.loop_end(),
            Instruction::Read => self.read_input(),
            Instruction::Write => self.write_output(),
        }
    }

    fn current_data(&mut self) -> &mut u8 {
        &mut self.data[self.data_pointer]
    }

    fn move_right(&mut self, amount: usize) {
        self.data_pointer = self.data_pointer.wrapping_add(amount);
        self.instruction_pointer += 1;
    }

    fn move_left(&mut self, amount: usize) {
        self.data_pointer = self.data_pointer.wrapping_sub(amount);
        self.instruction_pointer += 1;
    }

    fn increment(&mut self, amount: u8) {
        *self.current_data() = self.current_data().wrapping_add(amount);
        self.instruction_pointer += 1;
    }

    fn decrement(&mut self, amount: u8) {
        *self.current_data() = self.current_data().wrapping_sub(amount);
        self.instruction_pointer += 1;
    }

    fn find_jump(&self, instruction_pointer: &usize) -> usize {
        match self.jump_table.get(instruction_pointer) {
            Some(&jump) => jump,
            _ => panic!(format!(
                "Could not find jump table entry for instruction pointer '{}'",
                self.instruction_pointer
            )),
        }
    }

    fn loop_start(&mut self) {
        if *self.current_data() == 0 {
            self.instruction_pointer = self.find_jump(&self.instruction_pointer);
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn loop_end(&mut self) {
        if *self.current_data() != 0 {
            self.instruction_pointer = self.find_jump(&self.instruction_pointer);
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn get_char_from_input(&self) -> u8 {
        let mut buffer: [u8; 1] = [0];
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer).expect("Invalid input");

        buffer[0]
    }

    fn read_input(&mut self) {
        *self.current_data() = self.get_char_from_input();
        self.instruction_pointer += 1;
    }

    fn write_output(&mut self) {
        print!("{}", *self.current_data() as char);
        self.instruction_pointer += 1;
    }
}
