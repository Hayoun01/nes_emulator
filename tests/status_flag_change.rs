mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

#[test]
fn clc_can_clear_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::all());
    cpu.write(0xFFFC, Opcode::ClcIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert!(!cpu.flag.contains(Flag::CARRY));
}

#[test]
fn sec_can_set_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::SecIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.flag.bits(), Flag::CARRY.bits());
}

#[test]
fn cld_can_clear_decimal_mode_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::all());
    cpu.write(0xFFFC, Opcode::CldIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert!(!cpu.flag.contains(Flag::DECIMAL_MODE));
}

#[test]
fn sed_can_set_decimal_mode_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::SedIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.flag.bits(), Flag::DECIMAL_MODE.bits());
}

#[test]
fn cli_can_clear_interrupt_disable_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::all());
    cpu.write(0xFFFC, Opcode::CliIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert!(!cpu.flag.contains(Flag::INTERRUPT_DISABLE));
}

#[test]
fn sei_can_set_interrupt_disable_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::SeiIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.flag.bits(), Flag::INTERRUPT_DISABLE.bits());
}

#[test]
fn clv_can_clear_overflow_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::all());
    cpu.write(0xFFFC, Opcode::ClvIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert!(!cpu.flag.contains(Flag::OVERFLOW));
}
