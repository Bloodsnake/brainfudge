mod interpreter;
mod wrapping_integer;

use std::fs;
use interpreter::Interpreter;

fn main() {
    let memory_lenght = 30000;

    let program = fs::read_to_string("program.b").expect("Failed reading file");

    let mut interpreter = Interpreter::new(memory_lenght);
    interpreter.run(program);
}
