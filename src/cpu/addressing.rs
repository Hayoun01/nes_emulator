use crate::{
    bus::{Bus, Byte, Word},
    cpu::{Access, CPU},
};

// * Addressing Modes

impl CPU {
    /// ### Addressing Modes - Zero page
    pub fn addr_zero_page<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Byte {
        self.fetch_byte(cycles, bus)
    }
    /// ### Addressing Modes - Zero page with X offset
    pub fn addr_zero_page_x<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Byte {
        let addr = self.fetch_byte(cycles, bus).wrapping_add(self.x);
        *cycles -= 1;
        addr
    }
    /// ### Addressing Modes - Zero page with Y offset
    pub fn addr_zero_page_y<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Byte {
        let addr = self.fetch_byte(cycles, bus).wrapping_add(self.y);
        *cycles -= 1;
        addr
    }
    /// ### Addressing Modes - Absolute
    pub fn addr_absolute<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Word {
        self.fetch_word(cycles, bus)
    }
    /// ### Addressing Modes - Absolute with X offset
    pub fn addr_absolute_x<B: Bus>(
        &mut self,
        cycles: &mut i32,
        bus: &mut B,
        access: Access,
    ) -> Word {
        let mut addr = self.fetch_word(cycles, bus);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.x) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.x as Word);
        addr
    }
    /// ### Addressing Modes - Absolute with Y offset
    pub fn addr_absolute_y<B: Bus>(
        &mut self,
        cycles: &mut i32,
        bus: &mut B,
        access: Access,
    ) -> Word {
        let mut addr = self.fetch_word(cycles, bus);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.y) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.y as Word);
        addr
    }
    /// ### Addressing Modes - Indirect
    /// only used with **JMP** instruction
    pub fn addr_indirect<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Word {
        let addr = self.fetch_word(cycles, bus);
        self.read_word(cycles, addr, bus)
    }
    /// ### Addressing Modes - Indexed Indirect (X)
    pub fn addr_indirect_x<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Word {
        let mut addr = self.fetch_byte(cycles, bus);
        addr = addr.wrapping_add(self.x);
        *cycles -= 1;

        self.read_word_zp(cycles, addr, bus)
    }
    /// ### Addressing Modes - Indirect Indexed (Y)
    pub fn addr_indirect_y<B: Bus>(
        &mut self,
        cycles: &mut i32,
        bus: &mut B,
        access: Access,
    ) -> Word {
        let addr = self.fetch_byte(cycles, bus);
        let mut addr = self.read_word_zp(cycles, addr, bus);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.y) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.y as Word);
        addr
    }
}
