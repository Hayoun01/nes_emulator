mod common;
use common::setup_cpu_bus;
use cpu_6502::bus::{Bus, simple_bus::MEMORY_SIZE};

#[test]
fn cpu_initialized_properly() {
    let (cpu, bus) = setup_cpu_bus();
    assert_eq!(cpu.pc, 0xFFFC);
    assert_eq!(cpu.sp, 0xFD);
    assert!(cpu.flag.is_empty());
    assert_eq!(bus.ram, [0x0; MEMORY_SIZE]);
}

#[test]
#[should_panic(expected = "unknown CPU instruction")]
fn invalid_cpu_instruction() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus.write(0xFFFC, 0x0);
    cpu.execute(1, &mut bus);
}
