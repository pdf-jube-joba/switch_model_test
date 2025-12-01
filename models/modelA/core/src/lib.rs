// simple turing machine
// states, symbols are strings
// blank symbol is "", not underscore
// direction is -1 or 0 or 1

pub fn blank() -> String {
    "".to_string()
}

// infinite tape to the left and right
// => use push and pop to simulate infinite tape

pub struct Tape {
    cells: Vec<String>,
    head: usize,
}

impl Tape {
    pub fn left(&mut self) {
        if self.head > 0 {
            self.head -= 1;
        }
        // if head is at 0, add a blank cell at the beginning
        else {
            self.cells.insert(0, blank());
        }
    }
    pub fn right(&mut self) {
        self.head += 1;
        // if head is at the end, add a blank cell at the end
        if self.head >= self.cells.len() {
            self.cells.push(blank());
        }
    }
    pub fn at_head(&self) -> String {
        self.cells[self.head].clone()
    }

    pub fn write(&mut self, symbol: String) {
        self.cells[self.head] = symbol;
    }
}

pub struct Code {
    // (current_state, current_symbol, new_symbol, new_state, direction)
    instructions: Vec<(String, String, String, String, i32)>,
}

impl Code {
    pub fn find_instruction(
        &self,
        state: &String,
        symbol: &String,
    ) -> Option<&(String, String, String, String, i32)> {
        self.instructions
            .iter()
            .find(|&instr| &instr.0 == state && &instr.1 == symbol)
            .map(|v| v as _)
    }
}

pub struct TuringMachine {
    tape: Tape,
    code: Code,
    state: String,
}

impl TuringMachine {
    pub fn step(&mut self) -> bool {
        let current_symbol = self.tape.at_head();
        let state = self.state.clone();
        if let Some((_, _, new_symbol, new_state, direction)) =
            self.code.find_instruction(&state, &current_symbol).cloned()
        {
            self.tape.write(new_symbol);
            self.state = new_state;

            match direction {
                -1 => self.tape.left(),
                1 => self.tape.right(),
                _ => {}
            }

            true
        } else {
            false
        }
    }
}
