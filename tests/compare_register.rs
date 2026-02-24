mod common;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::Byte,
    cpu::{Flag, instructions::opcode::Opcode},
};

struct CMPTestData {
    register_value: Byte,
    operand: Byte,
    expected_flags: Flag,
}

enum Register {
    A,
    X,
    Y,
}

fn compare_two_identical_values() -> CMPTestData {
    CMPTestData {
        register_value: 42,
        operand: 42,
        expected_flags: Flag::ZERO | Flag::CARRY,
    }
}

fn compare_a_large_to_a_small_positive() -> CMPTestData {
    CMPTestData {
        register_value: 82,
        operand: 42,
        expected_flags: Flag::CARRY,
    }
}

fn compare_a_negative_to_a_positive() -> CMPTestData {
    CMPTestData {
        register_value: 130,
        operand: 42,
        expected_flags: Flag::CARRY,
    }
}

fn compare_two_values_and_get_negative_flag() -> CMPTestData {
    CMPTestData {
        register_value: 37,
        operand: 42,
        expected_flags: Flag::NEGATIVE,
    }
}

fn compare_imm(register: Register, data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(
        0xFFFC,
        match register {
            Register::A => Opcode::CmpIMM.into(),
            Register::X => Opcode::CpxIMM.into(),
            Register::Y => Opcode::CpyIMM.into(),
        },
    );
    cpu.write(0xFFFD, data.operand);
    let register = match register {
        Register::A => &mut cpu.a,
        Register::X => &mut cpu.x,
        Register::Y => &mut cpu.y,
    };
    *register = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_zpg(register: Register, data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(
        0xFFFC,
        match register {
            Register::A => Opcode::CmpZPG.into(),
            Register::X => Opcode::CpxZPG.into(),
            Register::Y => Opcode::CpyZPG.into(),
        },
    );
    cpu.write(0xFFFD, 0x0042);
    cpu.write(0x0042, data.operand);
    let register = match register {
        Register::A => &mut cpu.a,
        Register::X => &mut cpu.x,
        Register::Y => &mut cpu.y,
    };
    *register = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_zpx(data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::CmpZPX.into());
    cpu.write(0xFFFD, 0x0040);
    cpu.write(0x0042, data.operand);
    cpu.x = 0x02;
    cpu.a = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_abs(register: Register, data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(
        0xFFFC,
        match register {
            Register::A => Opcode::CmpABS.into(),
            Register::X => Opcode::CpxABS.into(),
            Register::Y => Opcode::CpyABS.into(),
        },
    );
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, data.operand);
    let register = match register {
        Register::A => &mut cpu.a,
        Register::X => &mut cpu.x,
        Register::Y => &mut cpu.y,
    };
    *register = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_abx(data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::CmpABX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0xFFFE, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.x = 0x07;
    cpu.a = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_idx(data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::CmpIDX.into());
    cpu.write(0xFFFD, 0x30);
    cpu.write(0x0041, 0x37);
    cpu.write(0x0042, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.x = 0x11;
    cpu.a = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

fn compare_idy(data: CMPTestData) {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::CmpIDY.into());
    cpu.write(0xFFFD, 0x41);
    cpu.write(0x0041, 0x30);
    cpu.write(0x0042, 0x13);
    cpu.write(0x1337, data.operand);
    cpu.y = 0x07;
    cpu.a = data.register_value;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.flag.bits(), data.expected_flags.bits());
}

#[test]
fn cmp_imm_can_compare_two_identical_values() {
    compare_imm(Register::A, compare_two_identical_values());
}

#[test]
fn cmp_imm_can_compare_a_large_to_a_small_positive() {
    compare_imm(Register::A, compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_imm_can_compare_a_negative_to_a_positive() {
    compare_imm(Register::A, compare_a_negative_to_a_positive());
}

#[test]
fn cmp_imm_can_compare_two_values_and_get_negative_flag() {
    compare_imm(Register::A, compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_zpg_can_compare_two_identical_values() {
    compare_zpg(Register::A, compare_two_identical_values());
}

#[test]
fn cmp_zpg_can_compare_a_large_to_a_small_positive() {
    compare_zpg(Register::A, compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_zpg_can_compare_a_negative_to_a_positive() {
    compare_zpg(Register::A, compare_a_negative_to_a_positive());
}

#[test]
fn cmp_zpg_can_compare_two_values_and_get_negative_flag() {
    compare_zpg(Register::A, compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_zpx_can_compare_two_identical_values() {
    compare_zpx(compare_two_identical_values());
}

#[test]
fn cmp_zpx_can_compare_a_large_to_a_small_positive() {
    compare_zpx(compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_zpx_can_compare_a_negative_to_a_positive() {
    compare_zpx(compare_a_negative_to_a_positive());
}

#[test]
fn cmp_zpx_can_compare_two_values_and_get_negative_flag() {
    compare_zpx(compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_abs_can_compare_two_identical_values() {
    compare_abs(Register::A, compare_two_identical_values());
}

#[test]
fn cmp_abs_can_compare_a_large_to_a_small_positive() {
    compare_abs(Register::A, compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_abs_can_compare_a_negative_to_a_positive() {
    compare_abs(Register::A, compare_a_negative_to_a_positive());
}

#[test]
fn cmp_abs_can_compare_two_values_and_get_negative_flag() {
    compare_abs(Register::A, compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_abx_can_compare_two_identical_values() {
    compare_abx(compare_two_identical_values());
}

#[test]
fn cmp_abx_can_compare_a_large_to_a_small_positive() {
    compare_abx(compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_abx_can_compare_a_negative_to_a_positive() {
    compare_abx(compare_a_negative_to_a_positive());
}

#[test]
fn cmp_abx_can_compare_two_values_and_get_negative_flag() {
    compare_abx(compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_idx_can_compare_two_identical_values() {
    compare_idx(compare_two_identical_values());
}

#[test]
fn cmp_idx_can_compare_a_large_to_a_small_positive() {
    compare_idx(compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_idx_can_compare_a_negative_to_a_positive() {
    compare_idx(compare_a_negative_to_a_positive());
}

#[test]
fn cmp_idx_can_compare_two_values_and_get_negative_flag() {
    compare_idx(compare_two_values_and_get_negative_flag());
}

#[test]
fn cmp_idy_can_compare_two_identical_values() {
    compare_idy(compare_two_identical_values());
}

#[test]
fn cmp_idy_can_compare_a_large_to_a_small_positive() {
    compare_idy(compare_a_large_to_a_small_positive());
}

#[test]
fn cmp_idy_can_compare_a_negative_to_a_positive() {
    compare_idy(compare_a_negative_to_a_positive());
}

#[test]
fn cmp_idy_can_compare_two_values_and_get_negative_flag() {
    compare_idy(compare_two_values_and_get_negative_flag());
}

#[test]
fn cpx_imm_can_compare_two_identical_values() {
    compare_imm(Register::X, compare_two_identical_values());
}

#[test]
fn cpx_imm_can_compare_a_large_to_a_small_positive() {
    compare_imm(Register::X, compare_a_large_to_a_small_positive());
}

#[test]
fn cpx_imm_can_compare_a_negative_to_a_positive() {
    compare_imm(Register::X, compare_a_negative_to_a_positive());
}

#[test]
fn cpx_imm_can_compare_two_values_and_get_negative_flag() {
    compare_imm(Register::X, compare_two_values_and_get_negative_flag());
}

#[test]
fn cpx_zpg_can_compare_two_identical_values() {
    compare_zpg(Register::X, compare_two_identical_values());
}

#[test]
fn cpx_zpg_can_compare_a_large_to_a_small_positive() {
    compare_zpg(Register::X, compare_a_large_to_a_small_positive());
}

#[test]
fn cpx_zpg_can_compare_a_negative_to_a_positive() {
    compare_zpg(Register::X, compare_a_negative_to_a_positive());
}

#[test]
fn cpx_zpg_can_compare_two_values_and_get_negative_flag() {
    compare_zpg(Register::X, compare_two_values_and_get_negative_flag());
}

#[test]
fn cpx_abs_can_compare_two_identical_values() {
    compare_abs(Register::X, compare_two_identical_values());
}

#[test]
fn cpx_abs_can_compare_a_large_to_a_small_positive() {
    compare_abs(Register::X, compare_a_large_to_a_small_positive());
}

#[test]
fn cpx_abs_can_compare_a_negative_to_a_positive() {
    compare_abs(Register::X, compare_a_negative_to_a_positive());
}

#[test]
fn cpx_abs_can_compare_two_values_and_get_negative_flag() {
    compare_abs(Register::X, compare_two_values_and_get_negative_flag());
}

#[test]
fn cpy_imm_can_compare_two_identical_values() {
    compare_imm(Register::Y, compare_two_identical_values());
}

#[test]
fn cpy_imm_can_compare_a_large_to_a_small_positive() {
    compare_imm(Register::Y, compare_a_large_to_a_small_positive());
}

#[test]
fn cpy_imm_can_compare_a_negative_to_a_positive() {
    compare_imm(Register::Y, compare_a_negative_to_a_positive());
}

#[test]
fn cpy_imm_can_compare_two_values_and_get_negative_flag() {
    compare_imm(Register::Y, compare_two_values_and_get_negative_flag());
}

#[test]
fn cpy_zpg_can_compare_two_identical_values() {
    compare_zpg(Register::Y, compare_two_identical_values());
}

#[test]
fn cpy_zpg_can_compare_a_large_to_a_small_positive() {
    compare_zpg(Register::Y, compare_a_large_to_a_small_positive());
}

#[test]
fn cpy_zpg_can_compare_a_negative_to_a_positive() {
    compare_zpg(Register::Y, compare_a_negative_to_a_positive());
}

#[test]
fn cpy_zpg_can_compare_two_values_and_get_negative_flag() {
    compare_zpg(Register::Y, compare_two_values_and_get_negative_flag());
}

#[test]
fn cpy_abs_can_compare_two_identical_values() {
    compare_abs(Register::Y, compare_two_identical_values());
}

#[test]
fn cpy_abs_can_compare_a_large_to_a_small_positive() {
    compare_abs(Register::Y, compare_a_large_to_a_small_positive());
}

#[test]
fn cpy_abs_can_compare_a_negative_to_a_positive() {
    compare_abs(Register::Y, compare_a_negative_to_a_positive());
}

#[test]
fn cpy_abs_can_compare_two_values_and_get_negative_flag() {
    compare_abs(Register::Y, compare_two_values_and_get_negative_flag());
}
