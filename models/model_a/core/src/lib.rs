// simple counter
#[derive(Debug, Default, Clone)]
pub struct ModelA {
    count: i32,
}

impl ModelA {
    pub fn new(count: i32) -> Self {
        Self { count }
    }
    pub fn current(&self) -> i32 {
        self.count
    }

    pub fn step(&mut self) -> i32 {
        self.count += 1;
        self.count
    }

    pub fn output_if_terminal(&self) -> Option<i32> {
        if self.count >= 10 {
            Some(self.count)
        } else {
            None
        }
    }
}
