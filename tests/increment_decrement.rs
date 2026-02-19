mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::Opcode};

// * [INC] TESTS

#[test]
fn inc_zero_page_can_incr_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::IncZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x29);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn inc_zero_page_x_can_incr_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::IncZPX.into());
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0xFF);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn inc_abs_can_incr_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::IncABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn inc_abs_x_can_incr_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::IncABX.into());
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x7F);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn inx_can_incr_x_register_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InxIMP.into());
    cpu.x = 0xFF;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn inx_can_incr_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InxIMP.into());
    cpu.x = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn inx_can_incr_x_register_negative() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InxIMP.into());
    cpu.x = 0x7F;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn iny_can_incr_y_register_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InyIMP.into());
    cpu.y = 0xFF;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn iny_can_incr_y_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InyIMP.into());
    cpu.y = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn iny_can_incr_y_register_negative() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::InyIMP.into());
    cpu.y = 0x7F;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * [DEC] TESTS

#[test]
fn dec_zero_page_can_decrement_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DecZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x2B);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn dec_zero_page_x_can_decrement_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DecZPX.into());
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0x01);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn dec_abs_can_decrement_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DecABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x2);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn dec_abs_x_can_decrement_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DecABX.into());
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0xFF);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn dex_can_decrement_x_register_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DexIMP.into());
    cpu.x = 0x1;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn dex_can_decrement_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DexIMP.into());
    cpu.x = 0x2;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn dex_can_decrement_x_register_negative() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DexIMP.into());
    cpu.x = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0xFF);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn dey_can_decrement_y_register_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DeyIMP.into());
    cpu.y = 0x1;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn dey_can_decrement_y_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DeyIMP.into());
    cpu.y = 0x2;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x1);
    assert!(cpu.flag.is_empty());
}

#[test]
fn dey_can_decrement_y_register_negative() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::DeyIMP.into());
    cpu.y = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0xFF);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}
