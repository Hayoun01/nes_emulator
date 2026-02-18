mod common;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::{Byte, Word},
    cpu::{Flag, instructions::Opcode},
};

// * TSX TESTS

#[test]
fn tsx_can_transfer_sp_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::TsxIMP.into());
    cpu.sp = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn tsx_can_transfer_zero_sp_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::TsxIMP.into());
    cpu.sp = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn tsx_can_transfer_negative_sp_to_x_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::TsxIMP.into());
    cpu.sp = 0b10000000;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0b10000000);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * TXS TESTS

#[test]
fn txs_can_transfer_x_register_to_sp() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::TxsIMP.into());
    cpu.x = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.sp, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * PHA TESTS

#[test]
fn pha_can_push_a_register_to_the_stack() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::PhaIMP.into());
    cpu.a = 0x2A;
    assert_eq!(cpu.sp, 0xFD);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(cpu.stack_addr() + 1), 0x2A);
    assert_eq!(cpu.sp, 0xFC);
    assert!(cpu.flag.is_empty());
}

// * PHP TESTS

#[test]
fn php_can_push_cpu_status_to_the_stack() {
    let mut cpu = setup_cpu_bus();
    let flags = Flag::CARRY | Flag::DECIMAL_MODE;
    let flags_as_byte = flags.bits();
    cpu.write(0xFFFC, Opcode::PhpIMP.into());
    cpu.flag.insert(flags);
    assert_eq!(cpu.sp, 0xFD);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(cpu.stack_addr() + 1), flags_as_byte);
    assert_eq!(cpu.sp, 0xFC);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
}

// * PLA TESTS

#[test]
fn pla_can_pull_value_from_stack_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::PhaIMP.into());
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    cpu.write(0xFFFD, Opcode::PlaIMP.into());
    // reset a register
    cpu.a = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn pla_can_pull_zero_value_from_stack_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::PhaIMP.into());
    cpu.a = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    cpu.write(0xFFFD, Opcode::PlaIMP.into());
    // change a register
    cpu.a = 0xFF;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn pla_can_pull_negative_value_from_stack_to_a_register() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::PhaIMP.into());
    cpu.a = 0x80;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    cpu.write(0xFFFD, Opcode::PlaIMP.into());
    // reset a register
    cpu.a = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * PLP TESTS

#[test]
fn plp_can_pull_cpu_status_from_the_stack() {
    let mut cpu = setup_cpu_bus();
    let flags = Flag::CARRY | Flag::DECIMAL_MODE;
    let flags_as_byte = flags.bits();
    cpu.flag.insert(flags);
    // push
    cpu.write(0xFFFC, Opcode::PhpIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(cpu.stack_addr() + 1), flags_as_byte);
    assert_eq!(cpu.sp, 0xFC);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
    // change cpu status
    cpu.write(0xFFFD, Opcode::LdaIMM.into());
    cpu.write(0xFFFE, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    // pull
    cpu.write(0xFFFF, Opcode::PlpIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), flags_as_byte);
}

// * Stack Operations TESTS

#[test]
fn stack_can_push_and_pull_byte() {
    let mut cpu = setup_cpu_bus();
    cpu.push_byte(0x2A);
    assert_eq!(cpu.read_byte(0x01FD), 0x2A); // check the data if stored in the correct place
    assert_eq!(cpu.sp, 0xFC); // check if the SP decremented by 1 after storing the data
    let data = cpu.pull_byte();
    assert_eq!(data, 0x2A);
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_array_of_bytes() {
    let mut cpu = setup_cpu_bus();
    for i in 1..=10 {
        cpu.push_byte(i);
        let addr = 0x0100 | (cpu.sp + 1) as Word;
        assert_eq!(cpu.read_byte(addr), i); // check the data if stored in the correct place
    }
    assert_eq!(cpu.sp, 0xFD - 10); // check if the SP decremented by 10 after storing the data
    for i in (1..=10).rev() {
        let data = cpu.pull_byte();
        assert_eq!(data, i);
    }
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_word() {
    let mut cpu = setup_cpu_bus();
    cpu.push_word(0x2A37);
    assert_eq!(cpu.read_byte(0x01FD), 0x2A); // hi first
    assert_eq!(cpu.read_byte(0x01FC), 0x37); // lo
    assert_eq!(cpu.sp, 0xFB); // check if the SP decremented by 2 after storing the data
    let data = cpu.pull_word();
    assert_eq!(data, 0x2A37);
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}

#[test]
fn stack_can_push_and_pull_array_of_words() {
    let mut cpu = setup_cpu_bus();
    for i in 0x2A2A..0x2A34 {
        cpu.push_word(i);
        let mut addr = 0x0100 | (cpu.sp + 1) as Word;
        assert_eq!(cpu.read_byte(addr), i as Byte); // check lo first
        addr += 1;
        assert_eq!(cpu.read_byte(addr), (i >> 8) as Byte); // check hi
    }
    assert_eq!(cpu.sp, 0xFD - 20); // check if the SP decremented by 20 after storing the data
    for i in (0x2A2A..0x2A34).rev() {
        let data = cpu.pull_word();
        assert_eq!(data, i);
    }
    assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
}
