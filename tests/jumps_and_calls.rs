mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

// * JSR TESTS

#[test]
fn jsr_absolute_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::JsrABS.into());
    cpu.write(0xFFFD, 0x80);
    cpu.write(0xFFFE, 0x80);
    cpu.write(0x8080, Opcode::LdaIMM.into());
    cpu.write(0x8081, 0x2A);
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * RTS TESTS
#[test]
fn jsr_rts_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::JsrABS.into());
    cpu.write(0xFFFD, 0x80);
    cpu.write(0xFFFE, 0x80);
    cpu.write(0xFFFF, Opcode::LdaIMM.into());
    cpu.write(0x0000, 0x2A);
    cpu.write(0x8080, Opcode::RtsIMP.into()); // return
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    // another 6 cycle to execute RTS instruction
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    // check if the RTS returned successfully
    assert_eq!(cpu.pc, 0xFFFF);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * JMP TESTS
#[test]
fn jmp_abs() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::JmpABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x42);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.pc, 0x4242);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::JmpIND.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4242, 0x37);
    cpu.write(0x4243, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind_to_lda() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::JmpIND.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4242, 0x37);
    cpu.write(0x4243, 0x2A);
    cpu.write(0x2A37, Opcode::LdaIMM.into());
    cpu.write(0x2A38, 0x0);
    // Jump to ins
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
    // Execute ins after jump
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}
