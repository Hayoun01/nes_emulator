mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::Instruction};

// * LDA TESTS

#[test]
fn lda_immediate_load_zero_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIMM.into();
    bus[0xFFFD] = 0x0;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn lda_immediate_load_negative_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIMM.into();
    bus[0xFFFD] = 0x84;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn lda_immediate_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIMM.into();
    bus[0xFFFD] = 0x2A;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_x_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaZPX.into();
    bus[0xFFFD] = 0x40;
    bus[0x0042] = 0x2A;
    cpu.x = 0x2;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_x_must_wrap_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaZPX.into();
    bus[0xFFFD] = 0x43;
    bus[0x0042] = 0x2A;
    cpu.x = 0xFF;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abs_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x41; // Ox4142
    bus[0x4142] = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abx_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaABX.into();
    bus[0xFFFD] = 0x41;
    bus[0xFFFE] = 0x42; // 0x4241
    bus[0x4242] = 0x2A;
    cpu.x = 0x01;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abx_cross_page_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaABX.into();
    bus[0xFFFD] = 0xF0;
    bus[0xFFFE] = 0x02; // 0x0300
    bus[0x0300] = 0x2A;
    cpu.x = 0x10;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_aby_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaABY.into();
    bus[0xFFFD] = 0x41;
    bus[0xFFFE] = 0x42; // 0x4241
    bus[0x4242] = 0x2A;
    cpu.y = 0x01;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_aby_cross_page_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaABY.into();
    bus[0xFFFD] = 0xF0;
    bus[0xFFFE] = 0x02; // 0x0300
    bus[0x0300] = 0x2A;
    cpu.y = 0x10;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idx_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIDX.into();
    bus[0xFFFD] = 0x20;
    bus[0x0024] = 0x00;
    bus[0x0025] = 0x80;
    bus[0x8000] = 0x2A;
    cpu.x = 0x4;
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idy_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIDY.into();
    bus[0xFFFD] = 0x20;
    bus[0x0020] = 0x00;
    bus[0x0021] = 0x80;
    bus[0x8004] = 0x2A;
    cpu.y = 0x4;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idy_cross_page_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdaIDY.into();
    bus[0xFFFD] = 0x20;
    bus[0x0020] = 0x10;
    bus[0x0021] = 0x80;
    bus[0x8100] = 0x2A;
    cpu.y = 0xF0;
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * LDX TESTS

#[test]
fn ldx_immediate_load_zero_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxIMM.into();
    bus[0xFFFD] = 0x0;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn ldx_immediate_load_negative_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxIMM.into();
    bus[0xFFFD] = 0x84;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ldx_immediate_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxIMM.into();
    bus[0xFFFD] = 0x2A;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_zero_page_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_zero_page_y_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxZPY.into();
    bus[0xFFFD] = 0x40;
    bus[0x0042] = 0x2A;
    cpu.y = 0x2;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_abs_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxABS.into();
    bus[0xFFFD] = 0x42; // lo
    bus[0xFFFE] = 0x41; // Ox4142
    bus[0x4142] = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_aby_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxABY.into();
    bus[0xFFFD] = 0x41;
    bus[0xFFFE] = 0x42; // 0x4241
    bus[0x4242] = 0x2A;
    cpu.y = 0x01;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_aby_cross_page_load_value_to_register_x() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdxABY.into();
    bus[0xFFFD] = 0xF0;
    bus[0xFFFE] = 0x02; // 0x0300
    bus[0x0300] = 0x2A;
    cpu.y = 0x10;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * LDY TESTS

#[test]
fn ldy_immediate_load_zero_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyIMM.into();
    bus[0xFFFD] = 0x0;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn ldy_immediate_load_negative_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyIMM.into();
    bus[0xFFFD] = 0x84;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ldy_immediate_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyIMM.into();
    bus[0xFFFD] = 0x2A;
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_zero_page_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_zero_page_x_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyZPX.into();
    bus[0xFFFD] = 0x40;
    bus[0x0042] = 0x2A;
    cpu.x = 0x2;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abs_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyABS.into();
    bus[0xFFFD] = 0x42; // lo
    bus[0xFFFE] = 0x41; // Ox4142
    bus[0x4142] = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abx_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyABX.into();
    bus[0xFFFD] = 0x41;
    bus[0xFFFE] = 0x42; // 0x4241
    bus[0x4242] = 0x2A;
    cpu.x = 0x01;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abx_cross_page_load_value_to_register_y() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::LdyABX.into();
    bus[0xFFFD] = 0xF0;
    bus[0xFFFE] = 0xFF; // 0xFFF0
    bus[0x0000] = 0x2A;
    cpu.x = 0x10;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}
