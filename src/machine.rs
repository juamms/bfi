use std::io::{self, Read, Write};

#[derive(PartialEq, Debug)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    Clear,
    LoopStart(usize),
    LoopEnd(usize),
    Read,
    Write,
}

pub struct Machine {
    program: Vec<Instruction>,
    data: Vec<u8>,
    instruction_pointer: usize,
    data_pointer: usize,
}

/// Machine setup
impl Machine {
    pub fn new(size: usize) -> Machine {
        Machine {
            program: Vec::new(),
            data: vec![0; size],
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

        self.process_loops();
    }

    pub fn dump(&self) {
        println!("{:?}", &self.data[0..32]);
        println!("DP: {}", self.data_pointer);
        println!("IP: {}", self.instruction_pointer);
    }

    pub fn current_program(&self) -> &[Instruction] {
        &self.program
    }

    pub fn intermediate_representation(&self) -> String {
        self.program
            .iter()
            .map(|ins| format!("{:?}", ins))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn process_loops(&mut self) {
        for idx in 0..self.program.len() {
            if self.program[idx] == Instruction::LoopStart(0) {
                let start = idx;
                let end = self.find_loop_end(start);

                self.program[start] = Instruction::LoopStart(end + 1);
                self.program[end] = Instruction::LoopEnd(start + 1);
            }
        }
    }

    fn find_loop_end(&self, start: usize) -> usize {
        let mut end = start + 1;
        let mut skips = 0;

        while end < self.program.len() {
            if self.program[end] == Instruction::LoopStart(0) {
                skips += 1;
            } else if self.program[end] == Instruction::LoopEnd(0) {
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
                    _ => unreachable!(),
                }
            } else {
                instruction = match token {
                    '[' => {
                        if idx + 2 < raw_program.len()
                            && raw_program[idx + 1] == '-'
                            && raw_program[idx + 2] == ']'
                        {
                            idx += 2;
                            Instruction::Clear
                        } else {
                            Instruction::LoopStart(0)
                        }
                    }
                    ']' => Instruction::LoopEnd(0),
                    ',' => Instruction::Read,
                    '.' => Instruction::Write,
                    _ => unreachable!(),
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
                '[' => Instruction::LoopStart(0),
                ']' => Instruction::LoopEnd(0),
                ',' => Instruction::Read,
                '.' => Instruction::Write,
                _ => unreachable!(),
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
            Instruction::Clear => self.clear(),
            Instruction::LoopStart(pointer) => self.loop_start(pointer),
            Instruction::LoopEnd(pointer) => self.loop_end(pointer),
            Instruction::Read => self.read_input(),
            Instruction::Write => self.write_output(),
        }
    }

    fn current_data(&mut self) -> &mut u8 {
        &mut self.data[self.data_pointer]
    }

    fn next_instruction(&mut self) {
        self.instruction_pointer += 1;
    }

    fn move_right(&mut self, amount: usize) {
        self.data_pointer = self.data_pointer.wrapping_add(amount);
        self.next_instruction()
    }

    fn move_left(&mut self, amount: usize) {
        self.data_pointer = self.data_pointer.wrapping_sub(amount);
        self.next_instruction()
    }

    fn increment(&mut self, amount: u8) {
        *self.current_data() = self.current_data().wrapping_add(amount);
        self.next_instruction()
    }

    fn decrement(&mut self, amount: u8) {
        *self.current_data() = self.current_data().wrapping_sub(amount);
        self.next_instruction()
    }

    fn clear(&mut self) {
        *self.current_data() = 0;
        self.next_instruction()
    }

    fn loop_start(&mut self, pointer: usize) {
        if *self.current_data() == 0 {
            self.instruction_pointer = pointer;
        } else {
            self.next_instruction()
        }
    }

    fn loop_end(&mut self, pointer: usize) {
        if *self.current_data() != 0 {
            self.instruction_pointer = pointer;
        } else {
            self.next_instruction()
        }
    }

    fn get_char_from_input(&self) -> u8 {
        let stdin = io::stdin();

        // This fixes an input issue with the Lost Kingdom game
        let _ = io::stdout().flush();
        stdin.bytes().next().expect("Invalid input").unwrap()
    }

    fn read_input(&mut self) {
        *self.current_data() = self.get_char_from_input();
        self.next_instruction()
    }

    fn write_output(&mut self) {
        print!("{}", *self.current_data() as char);
        self.next_instruction()
    }
}
