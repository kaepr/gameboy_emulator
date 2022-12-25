pub struct Bus {
    memory: [u8; 0xFFFF],
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        todo!();
    }

    pub fn write16(&mut self, addr: u16) {}
}
