// simple programming language (goto language)

pub enum Statement {
    Assign(String, i32),   // variable, value
    Add(String, i32),      // variable, value
    Subtract(String, i32), // variable, value
    IfZero(String, usize), // variable, line number
}

pub struct Program {
    code: Vec<Statement>,
    counter: usize,
    variables: std::collections::HashMap<String, i32>,
}

impl Program {
    pub fn new(code: Vec<Statement>) -> Self {
        Self {
            code,
            counter: 0,
            variables: std::collections::HashMap::new(),
        }
    }

    pub fn execute_statements(&mut self) {
        while self.counter < self.code.len() {
            match &self.code[self.counter] {
                Statement::Assign(var, value) => {
                    self.variables.insert(var.clone(), *value);
                    self.counter += 1;
                }
                Statement::Add(var, value) => {
                    let entry = self.variables.entry(var.clone()).or_insert(0);
                    *entry += *value;
                    self.counter += 1;
                }
                Statement::Subtract(var, value) => {
                    let entry = self.variables.entry(var.clone()).or_insert(0);
                    *entry -= *value;
                    self.counter += 1;
                }
                Statement::IfZero(var, line) => {
                    let value = *self.variables.get(var).unwrap_or(&0);
                    if value == 0 {
                        self.counter = *line;
                    } else {
                        self.counter += 1;
                    }
                }
            }
        }
    }
}
