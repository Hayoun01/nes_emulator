mod common;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::Byte,
    cpu::{Flag, instructions::opcode::Opcode},
};

#[derive(PartialEq)]
enum Operation {
    Add,
    Sub,
}

struct ArithmeticTestData {
    carry: bool,
    a: Byte,
    operand: Byte,
    answer: Byte,
    expected_flags: Flag,
}

impl ArithmeticTestData {
    fn new(carry: bool, a: Byte, operand: Byte, answer: Byte, expected_flags: Flag) -> Self {
        Self {
            carry,
            a,
            operand,
            answer,
            expected_flags,
        }
    }
}

fn test_adc_sbc_imm(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcIMM.into()
        } else {
            Opcode::SbcIMM.into()
        },
    );
    cpu.write(0xFFFD, data.operand);
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 2);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_zpg(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcZPG.into()
        } else {
            Opcode::SbcZPG.into()
        },
    );
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, data.operand);
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 3);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_zpx(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcZPX.into()
        } else {
            Opcode::SbcZPX.into()
        },
    );
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, data.operand);
    cpu.x = 0x02;
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_abs(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcABS.into()
        } else {
            Opcode::SbcABS.into()
        },
    );
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_abx(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcABX.into()
        } else {
            Opcode::SbcABX.into()
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.x = 0x02;
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_aby(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcABY.into()
        } else {
            Opcode::SbcABY.into()
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.y = 0x02;
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 4);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_idx(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcIDX.into()
        } else {
            Opcode::SbcIDX.into()
        },
    );
    cpu.write(0xFFFD, 0x003F);
    cpu.write(0x0041, 0x37);
    cpu.write(0x0042, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.x = 0x02;
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 6);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn test_adc_sbc_idy(data: &ArithmeticTestData, op: Operation) {
    let mut cpu = setup_cpu_bus();
    cpu.flag.set(Flag::CARRY, data.carry);
    cpu.flag.set(
        Flag::NEGATIVE,
        !data.expected_flags.contains(Flag::NEGATIVE),
    );
    cpu.flag
        .set(Flag::ZERO, !data.expected_flags.contains(Flag::ZERO));
    cpu.flag.set(
        Flag::OVERFLOW,
        !data.expected_flags.contains(Flag::OVERFLOW),
    );
    cpu.write(
        0xFFFC,
        if op == Operation::Add {
            Opcode::AdcIDY.into()
        } else {
            Opcode::SbcIDY.into()
        },
    );
    cpu.write(0xFFFD, 0x0041);
    cpu.write(0x0041, 0x35);
    cpu.write(0x0042, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.y = 0x02;
    cpu.a = data.a;
    let cycle_expected = cpu.execute();
    assert_eq!(cycle_expected, 5);
    assert_eq!(cpu.a, data.answer);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

#[test]
fn adc_imm_can_add_zero_to_zero_and_get_zero() {
    let data = ArithmeticTestData::new(false, 0, 0, 0, Flag::ZERO);
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_can_add_carry_and_zero_to_zero_and_get_one() {
    let data = ArithmeticTestData::new(true, 0, 0, 1, Flag::empty());
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_can_add_carry_and_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x14, 0x15, 0x2A, Flag::empty());
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_can_add_carry_and_positive_and_negative_number() {
    // 00010100
    // 11111111
    // 00010011
    let data = ArithmeticTestData::new(false, 0x14, 0xFF, 0x13, Flag::CARRY);
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_can_add_one_to_ff_and_get_zero_with_carry() {
    let data = ArithmeticTestData::new(false, 0xFF, 1, 0, Flag::CARRY | Flag::ZERO);
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_will_set_the_negative_flag_when_the_result_is_negative() {
    let data = ArithmeticTestData::new(false, 0, 0xFF, 0xFF, Flag::NEGATIVE);
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_can_add_two_negative_numbers_and_get_negative() {
    let data = ArithmeticTestData::new(
        false,
        0xFD, // -3
        0xFE, // -2
        0xFB, // -5
        Flag::NEGATIVE | Flag::CARRY,
    );
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_imm_will_set_the_overflow_flag_when_signed_negative_addition_fails() {
    // 11111111
    // 10000000
    // 01111111
    let data = ArithmeticTestData::new(
        false,
        0xFF, // -1
        0x80, // -128
        0x7F, // 127 // * Overflow
        Flag::CARRY | Flag::OVERFLOW,
    );
    test_adc_sbc_imm(&data, Operation::Add);
}

#[test]
fn adc_zpg_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_zpg(&data, Operation::Add);
}

#[test]
fn adc_zpg_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_zpg(&data, Operation::Add);
}

#[test]
fn adc_zpx_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_zpx(&data, Operation::Add);
}

#[test]
fn adc_zpx_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_zpx(&data, Operation::Add);
}

#[test]
fn adc_abs_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_abs(&data, Operation::Add);
}

#[test]
fn adc_abs_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_abs(&data, Operation::Add);
}

#[test]
fn adc_abx_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_abx(&data, Operation::Add);
}

#[test]
fn adc_abx_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_abx(&data, Operation::Add);
}

#[test]
fn adc_aby_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_aby(&data, Operation::Add);
}

#[test]
fn adc_aby_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_aby(&data, Operation::Add);
}

#[test]
fn adc_idx_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_idx(&data, Operation::Add);
}

#[test]
fn adc_idx_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_idx(&data, Operation::Add);
}

#[test]
fn adc_idy_can_add_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x1F, 0x0A, 0x2A, Flag::empty());
    test_adc_sbc_idy(&data, Operation::Add);
}

#[test]
fn adc_idy_can_add_positive_and_negative_number() {
    let data = ArithmeticTestData::new(
        true,
        0x30, // 48
        0xF9, // -6
        0x2A,
        Flag::CARRY,
    );
    test_adc_sbc_idy(&data, Operation::Add);
}

// * SBC TESTS

#[test]
fn sbc_imm_can_sub_zero_from_zero_and_get_zero() {
    let data = ArithmeticTestData::new(true, 0, 0, 0, Flag::CARRY | Flag::ZERO);
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_zero_from_zero_with_carry_and_get_minus_one() {
    let data = ArithmeticTestData::new(
        false,
        0,
        0,
        0xFF, // -1
        Flag::NEGATIVE,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_one_from_zero_and_get_minus_one() {
    let data = ArithmeticTestData::new(
        true,
        0,
        1,
        0xFF, // -1
        Flag::NEGATIVE,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_one_from_zero_with_carry_and_get_minus_two() {
    let data = ArithmeticTestData::new(
        false,
        0,
        1,
        0xFE, // -2
        Flag::NEGATIVE,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_two_signed_numbers_and_get_signed_overflow() {
    let data = ArithmeticTestData::new(
        true,
        0x80, // -128
        1,
        0x7F, // 127
        Flag::OVERFLOW | Flag::CARRY,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_positive_and_negative_numbers_and_get_signed_overflow() {
    let data = ArithmeticTestData::new(
        true,
        0x7F, // 127
        0xFF, // -1
        0x80, // 128
        Flag::OVERFLOW | Flag::NEGATIVE,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_imm_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_imm(&data, Operation::Sub);
}

#[test]
fn sbc_zpg_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_zpg(&data, Operation::Sub);
}

#[test]
fn sbc_zpg_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_zpg(&data, Operation::Sub);
}

#[test]
fn sbc_zpx_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_zpx(&data, Operation::Sub);
}

#[test]
fn sbc_zpx_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_zpx(&data, Operation::Sub);
}

#[test]
fn sbc_abs_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_abs(&data, Operation::Sub);
}

#[test]
fn sbc_abs_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_abs(&data, Operation::Sub);
}

#[test]
fn sbc_abx_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_abx(&data, Operation::Sub);
}

#[test]
fn sbc_abx_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_abx(&data, Operation::Sub);
}

#[test]
fn sbc_aby_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_aby(&data, Operation::Sub);
}

#[test]
fn sbc_aby_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_aby(&data, Operation::Sub);
}

#[test]
fn sbc_idx_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_idx(&data, Operation::Sub);
}

#[test]
fn sbc_idx_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_idx(&data, Operation::Sub);
}

#[test]
fn sbc_idy_can_sub_two_unsigned_numbers() {
    let data = ArithmeticTestData::new(true, 0x2F, 0x05, 0x2A, Flag::CARRY);
    test_adc_sbc_idy(&data, Operation::Sub);
}

#[test]
fn sbc_idy_can_sub_two_negative_numbers() {
    let data = ArithmeticTestData::new(
        true,
        0xD1, // -47
        0xFB, // -5
        0xD6,
        Flag::NEGATIVE,
    );
    test_adc_sbc_idy(&data, Operation::Sub);
}
