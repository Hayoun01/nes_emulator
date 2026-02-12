mod common;
use cpu_6502::bus::{Byte, Word};

use crate::common::setup_cpu_bus;

#[test]
fn load_simple_program() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let program: [Byte; _] = [
        0x00, 0x10, 0xa9, 0xff, 0x85, 0x90, 0x8d, 0x00, 0x80, 0x49, 0xcc, 0x4c, 0x02, 0x10,
    ];
    cpu.load_program(&program, &mut bus);
    let mut addr = 0x1000 as Word;
    cpu.pc = addr;
    for &v in program.iter().skip(2) {
        assert_eq!(bus[addr], v);
        addr += 1;
    }

    for _ in 0..1000 {
        cpu.execute(1, &mut bus);
        println!("A: {}, X: {}, Y: {}", cpu.a, cpu.x, cpu.y);
        println!("PC: {}, SP: {}", cpu.pc, cpu.sp);
    }
}
