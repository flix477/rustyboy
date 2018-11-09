const MEMORY_MAP_LENGTH: usize = 65536;

pub struct Memory {
    pub map: [u8; MEMORY_MAP_LENGTH]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            map: [0; MEMORY_MAP_LENGTH]
        }
    }

    pub fn reset(&mut self) {
        self.map = [0; MEMORY_MAP_LENGTH];
    }

    pub fn set(&mut self, address: usize, value: u8) {
        self.map[address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let memory = Memory::new();
        assert!(memory.map.iter().all(|x| { *x == 0 }));
    }

    #[test]
    fn resets() {
        let mut memory = Memory::new();
        memory.map[1] = 3;
        memory.reset();
        assert!(memory.map.iter().all(|x| { *x == 0 }));
    }
}