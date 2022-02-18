const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct Memory {
    memory: [u16; MEMORY_SIZE]
}
impl Memory {
    pub fn new () -> Memory {
        Memory {
            memory: [0;MEMORY_SIZE]
        }
    }
    
    pub fn read(&self, address: u16) -> u16 {
        return self.memory[address as usize];
    }

    pub fn update(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }
}