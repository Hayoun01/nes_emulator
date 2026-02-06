use std::ops::{Index, IndexMut};

use crate::bus::{Bus, Byte, Word};

pub const MEMORY_SIZE: usize = 1024 * 64;

pub struct SimpleBus {
    /// ### Fake ram
    pub ram: [Byte; MEMORY_SIZE],
}

impl Default for SimpleBus {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleBus {
    pub fn new() -> Self {
        Self {
            ram: [0; MEMORY_SIZE],
        }
    }
}

impl Index<Word> for SimpleBus {
    type Output = Byte;

    fn index(&self, addr: Word) -> &Self::Output {
        &self.ram[addr as usize]
    }
}

impl IndexMut<Word> for SimpleBus {
    fn index_mut(&mut self, addr: Word) -> &mut Self::Output {
        &mut self.ram[addr as usize]
    }
}

impl Bus for SimpleBus {
    fn read(&mut self, addr: Word) -> Byte {
        self.ram[addr as usize]
    }

    fn write(&mut self, addr: Word, value: Byte) {
        self.ram[addr as usize] = value;
    }
}
