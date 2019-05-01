pub struct Log {
    limit: usize,
    data: Vec<String>,
    pub index: usize
}

impl Log {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            data: vec![String::new()],
            index: 0
        }
    }

    pub fn push(&mut self, value: String) {
        self.data.insert(1, value);
        self.index = 0;
        if self.data.len() > self.limit {
            self.data.pop();
        }
    }

    pub fn get(&self) -> Option<&String> {
        self.data.get(self.index)
    }

    pub fn back(&mut self) {
        if self.index + 1 < self.data.len() {
            self.index += 1;
        }
    }

    pub fn forward(&mut self) {
        self.index = self.index.saturating_sub(1);
    }
}