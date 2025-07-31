pub struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        // the index of an array must be of type usize
        self.memory[addr as usize]
    }
}