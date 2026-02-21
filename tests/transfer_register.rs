mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

#[test]
fn tax_can_transfer_non_zero_value_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TaxIMP.into());
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn tax_can_transfer_zero_value_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TaxIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn tax_can_transfer_negative_value_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TaxIMP.into());
    cpu.a = 0xFC;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0xFC);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn tay_can_transfer_non_zero_value_to_y_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TayIMP.into());
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn tay_can_transfer_zero_value_to_y_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TayIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn tay_can_transfer_negative_value_to_y_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TayIMP.into());
    cpu.a = 0xFC;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0xFC);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn txa_can_transfer_non_zero_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TxaIMP.into());
    cpu.x = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn txa_can_transfer_zero_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TxaIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn txa_can_transfer_negative_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TxaIMP.into());
    cpu.x = 0xFC;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0xFC);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn tya_can_transfer_non_zero_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TyaIMP.into());
    cpu.y = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn tya_can_transfer_zero_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TyaIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn tya_can_transfer_negative_value_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::TyaIMP.into());
    cpu.y = 0xFC;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0xFC);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}
