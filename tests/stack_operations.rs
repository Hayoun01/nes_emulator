mod common;
use bitflags::Flags;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::{Byte, Word},
    cpu::{Flag, instructions::Instruction},
};

// * TSX TESTS

#[test]
fn tsx_can_transfer_sp_to_x_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::TsxIMP.into();
    cpu.sp = 0x2A;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn tsx_can_transfer_zero_sp_to_x_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::TsxIMP.into();
    cpu.sp = 0x0;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn tsx_can_transfer_negative_sp_to_x_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::TsxIMP.into();
    cpu.sp = 0b10000000;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0b10000000);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * TXS TESTS

#[test]
fn txs_can_transfer_x_register_to_sp() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::TxsIMP.into();
    cpu.x = 0x2A;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.sp, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * PHA TESTS

#[test]
fn pha_can_push_a_register_to_the_stack() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::PhaIMP.into();
    cpu.a = 0x2A;
    assert_eq!(cpu.sp, 0xFD);
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[cpu.stack_addr() + 1], 0x2A);
    assert_eq!(cpu.sp, 0xFC);
    assert!(cpu.flag.is_empty());
}

// * PHP TESTS

#[test]
fn php_can_push_cpu_status_to_the_stack() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let flags = Flag::CARRY | Flag::DECIMAL_MODE;
    let flags_as_byte = flags.bits();
    bus[0xFFFC] = Instruction::PhpIMP.into();
    cpu.flag.insert(flags);
    assert_eq!(cpu.sp, 0xFD);
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[cpu.stack_addr() + 1], flags_as_byte);
    assert_eq!(cpu.sp, 0xFC);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
}

// * PLA TESTS

#[test]
fn pla_can_pull_value_from_stack_to_a_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::PhaIMP.into();
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    bus[0xFFFD] = Instruction::PlaIMP.into();
    // reset a register
    cpu.a = 0x0;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn pla_can_pull_zero_value_from_stack_to_a_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::PhaIMP.into();
    cpu.a = 0x0;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    bus[0xFFFD] = Instruction::PlaIMP.into();
    // change a register
    cpu.a = 0xFF;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn pla_can_pull_negative_value_from_stack_to_a_register() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::PhaIMP.into();
    cpu.a = 0x80;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    bus[0xFFFD] = Instruction::PlaIMP.into();
    // reset a register
    cpu.a = 0x0;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * PLP TESTS

#[test]
fn plp_can_pull_cpu_status_from_the_stack() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let flags = Flag::CARRY | Flag::DECIMAL_MODE;
    let flags_as_byte = flags.bits();
    bus[0xFFFC] = Instruction::PhpIMP.into();
    cpu.flag.insert(flags);
    assert_eq!(cpu.sp, 0xFD);
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[cpu.stack_addr() + 1], flags_as_byte);
    assert_eq!(cpu.sp, 0xFC);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
    cpu.flag.clear();
    bus[0xFFFD] = Instruction::PlpIMP.into();
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
}

// * Stack Operations TESTS

#[test]
fn stack_can_push_and_pull_byte() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let mut cycles = 4; // push and pull must cost 4 cycles
    cpu.push_byte(&mut cycles, 0x2A, &mut bus);
    assert_eq!(bus[0x01FD], 0x2A); // check the data if stored in the correct place
    assert_eq!(cpu.sp, 0xFC); // check if the SP decremented by 1 after storing the data
    let data = cpu.pull_byte(&mut cycles, &mut bus);
    assert_eq!(data, 0x2A);
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_array_of_bytes() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let mut cycles = 40; // push and pull must cost 40 cycles
    for i in 1..=10 {
        cpu.push_byte(&mut cycles, i, &mut bus);
        let addr = 0x0100 | (cpu.sp + 1) as Word;
        assert_eq!(bus[addr], i); // check the data if stored in the correct place
    }
    assert_eq!(cpu.sp, 0xFD - 10); // check if the SP decremented by 10 after storing the data
    for i in (1..=10).rev() {
        let data = cpu.pull_byte(&mut cycles, &mut bus);
        assert_eq!(data, i);
    }
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_word() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let mut cycles = 8; // push and pull must cost 8 cycles
    cpu.push_word(&mut cycles, 0x2A37, &mut bus);
    assert_eq!(bus[0x01FD], 0x2A); // hi first
    assert_eq!(bus[0x01FC], 0x37); // lo
    assert_eq!(cpu.sp, 0xFB); // check if the SP decremented by 2 after storing the data
    let data = cpu.pull_word(&mut cycles, &mut bus);
    assert_eq!(data, 0x2A37);
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_array_of_words() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    let mut cycles = 80; // push and pull must cost 80 cycles
    for i in 0x2A2A..0x2A34 {
        cpu.push_word(&mut cycles, i, &mut bus);
        let mut addr = 0x0100 | (cpu.sp + 1) as Word;
        assert_eq!(bus[addr], i as Byte); // check lo first
        addr += 1;
        assert_eq!(bus[addr], (i >> 8) as Byte); // check hi
    }
    assert_eq!(cpu.sp, 0xFD - 20); // check if the SP decremented by 20 after storing the data
    for i in (0x2A2A..0x2A34).rev() {
        let data = cpu.pull_word(&mut cycles, &mut bus);
        assert_eq!(data, i);
    }
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}
