mod common;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::Byte,
    cpu::{Flag, instructions::opcode::Opcode},
};

#[test]
fn beq_can_branch_forward_when_zero_flag_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.flag.insert(Flag::ZERO);
    cpu.x = 0x36;
    cpu.y = 0x29;
    cpu.write(0xFF00, Opcode::BeqREL.into());
    cpu.write(0xFF01, 0x28);
    cpu.write(0xFF02, Opcode::InyIMP.into());
    cpu.write(0xFF2A, Opcode::InxIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.x, 0x37);
    assert_eq!(cpu.y, 0x29);
}

#[test]
fn beq_can_branch_backward_when_zero_flag_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.flag.insert(Flag::ZERO);
    cpu.x = 0x36;
    cpu.y = 0x29;
    cpu.write(0xFEFF, Opcode::InxIMP.into());
    cpu.write(0xFF00, Opcode::BeqREL.into());
    cpu.write(0xFF01, -3i8 as Byte); // offset by -3 (0xFD)
    cpu.write(0xFF02, Opcode::InyIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.x, 0x37);
    assert_eq!(cpu.y, 0x29);
}

#[test]
fn beq_does_not_branch_when_zero_flag_is_not_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.x = 0x36;
    cpu.y = 0x29;
    cpu.write(0xFF00, Opcode::BeqREL.into());
    cpu.write(0xFF01, 0x28);
    cpu.write(0xFF02, Opcode::InyIMP.into());
    cpu.write(0xFF2A, Opcode::InxIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert!(cpu.flag.is_empty());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.x, 0x36);
    assert_eq!(cpu.y, 0x2A);
}

#[test]
fn beq_can_branch_when_zero_flag_set_and_page_crossed() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFDE0;
    cpu.flag.insert(Flag::ZERO);
    cpu.x = 0x36;
    cpu.y = 0x29;
    cpu.write(0xFDE0, Opcode::BeqREL.into());
    cpu.write(0xFDE1, 0x28);
    cpu.write(0xFDE2, Opcode::InyIMP.into());
    cpu.write(0xFE0A, Opcode::InxIMP.into());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.x, 0x37);
    assert_eq!(cpu.y, 0x29);
}

#[test]
fn bne_can_branch_forward_when_zero_flag_not_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BneREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert!(cpu.flag.is_empty());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bcc_can_branch_forward_when_carry_flag_not_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BccREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert!(cpu.flag.is_empty());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bcs_can_branch_forward_when_carry_flag_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFF00, Opcode::BcsREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert_eq!(cpu.flag.bits(), Flag::CARRY.bits());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bmi_can_branch_forward_when_negative_flag_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.flag.insert(Flag::NEGATIVE);
    cpu.write(0xFF00, Opcode::BmiREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bpl_can_branch_forward_when_negative_flag_not_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BplREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert!(cpu.flag.is_empty());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bvs_can_branch_forward_when_overflow_flag_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.flag.insert(Flag::OVERFLOW);
    cpu.write(0xFF00, Opcode::BvsREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert_eq!(cpu.flag.bits(), Flag::OVERFLOW.bits());
    assert_eq!(cpu.pc, 0xFF2A);
}

#[test]
fn bvc_can_branch_forward_when_overflow_flag_not_set() {
    let mut cpu = setup_cpu_bus();
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BvcREL.into());
    cpu.write(0xFF01, 0x28);
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert!(cpu.flag.is_empty());
    assert_eq!(cpu.pc, 0xFF2A);
}
