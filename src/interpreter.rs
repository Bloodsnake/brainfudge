use std::{usize, io::Read};

use crate::wrapping_integer::WrappingInteger;

pub struct Interpreter {
    memory: Vec<WrappingInteger<u8>>,
    memory_pointer: WrappingInteger<usize>,

    instructions: Vec<char>,
    instruction_pointer: usize,

    jump_stack: Vec<usize>,
}

impl Interpreter {
    pub fn new(memory_size: usize) -> Interpreter {
        Interpreter {
            memory: vec![WrappingInteger::new(u8::MAX, 0, 0); memory_size], 
            instructions: Vec::new(), 
            memory_pointer: WrappingInteger::new(memory_size - 1, 0, 0), 
            instruction_pointer: 0,
            jump_stack: Vec::new(),
        }
    }

    pub fn run(&mut self, tape: String) {
        self.instructions = tape.chars().collect();

        loop {
            let current_instruction = self.instructions.get(self.instruction_pointer);
            match current_instruction {
                Some(current_instruction) => self.interpret_instruction(*current_instruction),
                None => break,
            }
        }
    }

    fn interpret_instruction(&mut self, instruction: char) {
        match instruction {
            '>' => self.memory_pointer.add(1),
            '<' => self.memory_pointer.subtract(1),
            '+' => {
                let value = self.memory.get_mut(self.memory_pointer.get_value()).expect("Memory out of bound");
                value.add(1);
            },
            '-' => {
                let value = self.memory.get_mut(self.memory_pointer.get_value()).expect("Memory out of bound");
                value.subtract(1);
            },
            '[' => self.jump_stack.push(self.instruction_pointer),
            ']' => {
                let value = self.memory.get(self.memory_pointer.get_value()).expect("Memory out of bound");
                // Jump to last [ when value is not 0
                if value.get_value() != 0 {
                    let msg = format!("Missing '[' at position: {0}", self.instruction_pointer);
                    self.instruction_pointer = *self.jump_stack.last().expect(msg.as_str())
                }
                else {
                    self.jump_stack.pop();
                }
            },
            '.' => print!("{}", self.memory.get(
                    self.memory_pointer.get_value())
                    .expect("Memory out of bound")
                    .get_value() as char),
            ',' => {
                let input: Option<u8> = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);

                let cell = self.memory.get_mut(self.memory_pointer.get_value()).expect("Memory out of bound");
                cell.set_value(input.unwrap());
            },
            _ => (),
        }

        self.instruction_pointer += 1;
    }
}