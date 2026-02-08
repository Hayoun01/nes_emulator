mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::Instruction};

// * JSR TESTS

#[test]
fn jsr_absolute_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JsrABS.into();
    bus[0xFFFD] = 0x80;
    bus[0xFFFE] = 0x80;
    bus[0x8080] = Instruction::LdaIMM.into();
    bus[0x8081] = 0x2A;
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * RTS TESTS
#[test]
fn jsr_rts_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JsrABS.into();
    bus[0xFFFD] = 0x80;
    bus[0xFFFE] = 0x80;
    bus[0xFFFF] = Instruction::LdaIMM.into();
    bus[0x0000] = 0x2A;
    bus[0x8080] = Instruction::RtsIMP.into(); // return
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // another 6 cycle to execute RTS instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // check if the RTS returned successfully
    assert_eq!(cpu.pc, 0xFFFF);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * JMP TESTS
#[test]
fn jmp_abs() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.pc, 0x4242);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpIND.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    bus[0x4242] = 0x37;
    bus[0x4243] = 0x2A;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind_to_lda() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpIND.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    bus[0x4242] = 0x37;
    bus[0x4243] = 0x2A;
    bus[0x2A37] = Instruction::LdaIMM.into();
    bus[0x2A38] = 0x0;
    // Jump to ins
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
    // Execute ins after jump
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}
