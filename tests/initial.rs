mod common;
use common::setup_cpu_bus;

#[test]
fn cpu_initialized_properly() {
    let cpu = setup_cpu_bus();
    assert_eq!(cpu.pc, 0xFFFC);
    assert_eq!(cpu.sp, 0xFD);
    assert!(cpu.flag.is_empty());
}

#[test]
#[should_panic(expected = "Illegal instruction!")]
fn invalid_cpu_instruction() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, 0x0);
    cpu.execute();
}
