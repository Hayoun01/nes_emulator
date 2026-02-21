mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

// * LDA TESTS

#[test]
fn lda_immediate_load_zero_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIMM.into());
    cpu.write(0xFFFD, 0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn lda_immediate_load_negative_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIMM.into());
    cpu.write(0xFFFD, 0x84);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn lda_immediate_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIMM.into());
    cpu.write(0xFFFD, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_x_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaZPX.into());
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0x2A);
    cpu.x = 0x2;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_zero_page_x_must_wrap_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaZPX.into());
    cpu.write(0xFFFD, 0x43);
    cpu.write(0x0042, 0x2A);
    cpu.x = 0xFF;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abs_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x41); // Ox4142
    cpu.write(0x4142, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abx_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaABX.into());
    cpu.write(0xFFFD, 0x41);
    cpu.write(0xFFFE, 0x42); // 0x4241
    cpu.write(0x4242, 0x2A);
    cpu.x = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_abx_cross_page_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaABX.into());
    cpu.write(0xFFFD, 0xF0);
    cpu.write(0xFFFE, 0x02); // 0x0300
    cpu.write(0x0300, 0x2A);
    cpu.x = 0x10;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_aby_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaABY.into());
    cpu.write(0xFFFD, 0x41);
    cpu.write(0xFFFE, 0x42); // 0x4241
    cpu.write(0x4242, 0x2A);
    cpu.y = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_aby_cross_page_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaABY.into());
    cpu.write(0xFFFD, 0xF0);
    cpu.write(0xFFFE, 0x02); // 0x0300
    cpu.write(0x0300, 0x2A);
    cpu.y = 0x10;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idx_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIDX.into());
    cpu.write(0xFFFD, 0x20);
    cpu.write(0x0024, 0x00);
    cpu.write(0x0025, 0x80);
    cpu.write(0x8000, 0x2A);
    cpu.x = 0x4;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idy_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIDY.into());
    cpu.write(0xFFFD, 0x20);
    cpu.write(0x0020, 0x00);
    cpu.write(0x0021, 0x80);
    cpu.write(0x8004, 0x2A);
    cpu.y = 0x4;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn lda_idy_cross_page_load_value_to_register_a() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdaIDY.into());
    cpu.write(0xFFFD, 0x20);
    cpu.write(0x0020, 0x10);
    cpu.write(0x0021, 0x80);
    cpu.write(0x8100, 0x2A);
    cpu.y = 0xF0;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * LDX TESTS

#[test]
fn ldx_immediate_load_zero_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxIMM.into());
    cpu.write(0xFFFD, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn ldx_immediate_load_negative_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxIMM.into());
    cpu.write(0xFFFD, 0x84);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ldx_immediate_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxIMM.into());
    cpu.write(0xFFFD, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_zero_page_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_zero_page_y_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxZPY.into());
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0x2A);
    cpu.y = 0x2;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_abs_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxABS.into());
    cpu.write(0xFFFD, 0x42); // lo
    cpu.write(0xFFFE, 0x41); // Ox4142
    cpu.write(0x4142, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_aby_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxABY.into());
    cpu.write(0xFFFD, 0x41);
    cpu.write(0xFFFE, 0x42); // 0x4241
    cpu.write(0x4242, 0x2A);
    cpu.y = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldx_aby_cross_page_load_value_to_register_x() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdxABY.into());
    cpu.write(0xFFFD, 0xF0);
    cpu.write(0xFFFE, 0x02); // 0x0300
    cpu.write(0x0300, 0x2A);
    cpu.y = 0x10;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.x, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * LDY TESTS

#[test]
fn ldy_immediate_load_zero_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyIMM.into());
    cpu.write(0xFFFD, 0x0);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn ldy_immediate_load_negative_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyIMM.into());
    cpu.write(0xFFFD, 0x84);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x84);
    assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
}

#[test]
fn ldy_immediate_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyIMM.into());
    cpu.write(0xFFFD, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_zero_page_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_zero_page_x_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyZPX.into());
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0x2A);
    cpu.x = 0x2;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abs_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyABS.into());
    cpu.write(0xFFFD, 0x42); // lo
    cpu.write(0xFFFE, 0x41); // Ox4142
    cpu.write(0x4142, 0x2A);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abx_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyABX.into());
    cpu.write(0xFFFD, 0x41);
    cpu.write(0xFFFE, 0x42); // 0x4241
    cpu.write(0x4242, 0x2A);
    cpu.x = 0x01;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}

#[test]
fn ldy_abx_cross_page_load_value_to_register_y() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::LdyABX.into());
    cpu.write(0xFFFD, 0xF0);
    cpu.write(0xFFFE, 0xFF); // 0xFFF0
    cpu.write(0x0000, 0x2A);
    cpu.x = 0x10;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.y, 0x2A);
    assert!(cpu.flag.is_empty());
}
