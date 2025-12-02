// simple toggle
#[derive(Debug, Default, Clone)]
pub struct ModelB {
    state: bool,
}

impl ModelB {
    pub fn new(state: bool) -> Self {
        Self { state }
    }
    pub fn current(&self) -> bool {
        self.state
    }

    pub fn step(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }
}
