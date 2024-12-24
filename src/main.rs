use std::io::Read;

///Every possible instruction in the brainfuck language
#[derive(PartialEq, Debug)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Replace,
    JumpToClose,
    JumpToOpen,
}
///object oriented kind of struct that represents the turing machine that runs the bf programs
struct TuringMachine {
    //the memory of the little turing machine, or "tape". 30000 cells in size. each cell being one
    //byte in size
    tape: [u8; 30000],
    //the pointer that indicates where the turing machines head is on the tape
    pointer: usize,
    //the program itself, represented as a long list of instructions
    program: Vec<Instruction>,
    //the program counter, that indicates which instruction in the program we are currently
    //executing
    program_counter: usize,
}

impl TuringMachine {
    ///create a new turing machine, sets every cell in memory to be 0 initially, sets the pointer
    ///and program pointer to 0, and turns bf programs into lists of instructions.
    fn new(program: &str) -> Self {
        TuringMachine {
            tape: [0; 30000],
            pointer: 0,
            program: TuringMachine::parse(program),
            program_counter: 0,
        }
    }
    ///executes the "MoveRight" instruction on the turing machine, so it just moves the head, or
    ///pointer, one value to the right on the tape
    fn move_right(&mut self) {
        self.pointer += 1;
        self.program_counter += 1;
    }
    ///executes the "MoveLeft" instruction on the turing machine, so it just moves the head, or
    ///pointer, one value to the left on the tape
    fn move_left(&mut self) {
        self.pointer -= 1;
        self.program_counter += 1;
    }
    ///executes the "Increment" instruction on the turing machine, does nothing more than Increment
    ///the value of the current cell being pointed at by the pointer or head
    fn increment(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
        self.program_counter += 1;
    }
    ///executes the "Decrement" instruction on the turing machine, does nothing more than Decrement
    ///the value of the current cell being pointed at by the pointer or head
    fn decrement(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
        self.program_counter += 1;
    }
    ///executes the "Write" instruction on the turing machine, prints the value of the current cell
    ///being pointed at by the pointer
    fn write(&mut self) {
        print!("{}", self.tape[self.pointer] as char);
        self.program_counter += 1;
    }

    fn replace(&mut self) {
        let mut input: [u8; 1] = [0; 1];
        std::io::stdin().read_exact(&mut input).unwrap();
        self.tape[self.pointer] = input[0];
        self.program_counter += 1;
    }
    ///gets the maching closing bracket for the opening bracket indicated by "bracket_to_match".
    fn get_matching_closing_bracket(&self, bracket_to_match: usize) -> usize {
        let mut stack: Vec<Instruction> = vec![];
        let mut return_token = 0;
        for token in (bracket_to_match)..self.program.len() {
            match self.program[token] {
                Instruction::JumpToOpen => {
                    if stack.is_empty() {
                        return_token = token;
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Instruction::JumpToClose => {
                    stack.push(Instruction::JumpToOpen);
                }
                _ => { /*ignoring*/ }
            }
        }
        return_token
    }
    ///gets the matching opening bracket for the closing bracket indicated by "bracket_to_match"
    fn get_matching_opening_bracket(&self, bracket_to_match: usize) -> usize {
        let mut stack: Vec<Instruction> = vec![];
        let mut return_token = 0;
        for token in (0..(bracket_to_match)).rev() {
            match self.program[token] {
                Instruction::JumpToOpen => {
                    stack.push(Instruction::JumpToOpen);
                }
                Instruction::JumpToClose => {
                    if stack.is_empty() {
                        return_token = token;
                        break;
                    } else {
                        stack.pop();
                    }
                }
                _ => { /*ignoring*/ }
            }
        }
        return_token
    }
    ///executes the "JumpToClose" instruction
    fn jump_if_zero(&mut self) {
        match self.tape[self.pointer] {
            0 => {
                let new_counter = self.get_matching_closing_bracket(self.program_counter);
                self.program_counter = new_counter;
            }
            _ => {
                self.program_counter += 1;
            }
        }
    }
    ///executes the "JumpToOpen" Instruction
    fn jump_unless_zero(&mut self) {
        match self.tape[self.pointer] {
            0 => {
                self.program_counter += 1;
            }
            _ => {
                let new_counter = self.get_matching_opening_bracket(self.program_counter);
                self.program_counter = new_counter;
            }
        }
    }

    ///turns a string representation of a brainfuck program into a list of instructions
    fn parse(program: &str) -> Vec<Instruction> {
        program
            .trim()
            .to_string()
            .chars()
            .map(|c| match c {
                '>' => Instruction::MoveRight,
                '<' => Instruction::MoveLeft,
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '.' => Instruction::Output,
                ',' => Instruction::Replace,
                '[' => Instruction::JumpToClose,
                ']' => Instruction::JumpToOpen,
                _ => panic!("unrecognized character: {}", c),
            })
            .collect::<Vec<_>>()
    }
    ///checks if the turing machine still has instructions left to exeute
    fn has_instructions_left(&self) -> bool {
        self.program_counter < self.program.len()
    }
    ///executes the current instruction pointed to by the program counter for our turing machine
    fn perform_next_instruction(&mut self) {
        match self.program.get(self.program_counter) {
            Some(Instruction::MoveRight) => {
                self.move_right();
            }
            Some(Instruction::MoveLeft) => {
                self.move_left();
            }
            Some(Instruction::Increment) => {
                self.increment();
            }
            Some(Instruction::Decrement) => {
                self.decrement();
            }
            Some(Instruction::Output) => {
                self.write();
            }
            Some(Instruction::Replace) => {
                self.replace();
            }
            Some(Instruction::JumpToClose) => {
                self.jump_if_zero();
            }
            Some(Instruction::JumpToOpen) => {
                self.jump_unless_zero();
            }
            None => {
                println!("not doing anything");
            }
        }
    }
    ///starts executing the program loaded into our turing machine.
    fn run(&mut self) {
        while self.has_instructions_left() {
            self.perform_next_instruction();
        }
    }
}

fn main() {
    let input = ",";
    //let input = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let mut tm = TuringMachine::new(input);
    tm.run();
}
