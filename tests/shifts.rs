mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

// * ASL TESTS

#[test]
fn asl_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslIMP.into());
    cpu.a = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 2);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn asl_can_shift_a_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslIMP.into());
    cpu.a = 0xFF;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 254);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn asl_zpg_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x01);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 2);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn asl_zpg_can_shift_a_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0xFF);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 254);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn asl_zpx_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x01);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 2);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn asl_zpx_can_shift_a_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0xFF);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 254);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn asl_abs_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x01);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 2);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn asl_abs_can_shift_a_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0xFF);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 254);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn asl_abx_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x01);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 2);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn asl_abx_can_shift_a_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::AslABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0xFF);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 254);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

// * LSR TESTS

#[test]
fn lsr_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrIMP.into());
    cpu.a = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn lsr_can_shift_a_zero_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrIMP.into());
    cpu.a = 0x08;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn lsr_zpg_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x01);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn lsr_zpg_can_shift_a_zero_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x08);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x04);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn lsr_zpx_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x01);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn lsr_zpx_can_shift_a_zero_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x08);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x04);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn lsr_abs_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x01);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn lsr_abs_can_shift_a_zero_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x08);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x04);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn lsr_abx_can_shift_the_value_of_one() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x01);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn lsr_abx_can_shift_a_zero_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LsrABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x08);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x04);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

// * ROL TESTS

#[test]
fn rol_can_shift_a_bit_out_of_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolIMP.into());
    cpu.a = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn rol_can_shift_a_bit_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolIMP.into());
    cpu.a = 0x80;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn rol_can_shift_zero_without_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolIMP.into());
    cpu.a = 0x00;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn rol_can_shift_a_value_that_results_in_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolIMP.into());
    cpu.a = 0b01110011;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0b11100111);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn rol_zpg_can_shift_a_bit_out_of_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x01);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn rol_zpg_can_shift_a_bit_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x80);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn rol_zpg_can_shift_zero_without_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x00);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn rol_zpg_can_shift_a_value_that_results_in_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0b01110011);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0b11100111);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn rol_zpx_can_shift_a_bit_out_of_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x0);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x01);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn rol_zpx_can_shift_a_bit_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x80);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn rol_zpx_can_shift_zero_without_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x00);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn rol_zpx_can_shift_a_value_that_results_in_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0b01110011);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0b11100111);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn rol_abs_can_shift_a_bit_out_of_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x01);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn rol_abs_can_shift_a_bit_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x80);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn rol_abs_can_shift_zero_without_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x00);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn rol_abs_can_shift_a_value_that_results_in_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0b01110011);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0b11100111);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn rol_abx_can_shift_a_bit_out_of_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x01);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn rol_abx_can_shift_a_bit_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x80);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::CARRY).bits());
}

#[test]
fn rol_abx_can_shift_zero_without_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RolABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x00);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn rol_abx_can_shift_a_value_that_results_in_negative_value() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RolABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0b01110011);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0b11100111);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

// * ROR TESTS

#[test]
fn ror_can_shift_the_carry_flag_into_operand() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorIMP.into());
    cpu.a = 0x0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ror_can_shift_the_value_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RorIMP.into());
    cpu.a = 0x1;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::ZERO).bits());
}

#[test]
fn ror_can_rotate_a_number() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorIMP.into());
    cpu.a = 0b01101101;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0b10110110);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn ror_zpg_can_shift_the_carry_flag_into_operand() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ror_zpg_can_shift_the_value_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RorZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0x1);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::ZERO).bits());
}

#[test]
fn ror_zpg_can_rotate_a_number() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorZPG.into());
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, 0b01101101);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x0042), 0b10110110);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn ror_zpx_can_shift_the_carry_flag_into_operand() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x0);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ror_zpx_can_shift_the_value_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RorZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0x1);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::ZERO).bits());
}

#[test]
fn ror_zpx_can_rotate_a_number() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, 0b01101101);
    cpu.x = 0x02;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x0042), 0b10110110);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn ror_abs_can_shift_the_carry_flag_into_operand() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ror_abs_can_shift_the_value_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RorABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x1);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::ZERO).bits());
}

#[test]
fn ror_abs_can_rotate_a_number() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorABS.into());
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0b01101101);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x1337), 0b10110110);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}

#[test]
fn ror_abx_can_shift_the_carry_flag_into_operand() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x0);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x80);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ror_abx_can_shift_the_value_into_the_carry_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::RorABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0x1);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0x0);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::ZERO).bits());
}

#[test]
fn ror_abx_can_rotate_a_number() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::CARRY);
    cpu.write(0xFFFC, Opcode::RorABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, 0b01101101);
    cpu.x = 0x07;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.read_byte(0x1337), 0b10110110);
    assert_eq!(cpu.flag.bits(), (Flag::CARRY | Flag::NEGATIVE).bits());
}
