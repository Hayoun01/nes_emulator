use bitflags::Flags;

use cpu_6502::{
    bus::{
        Bus, Byte, Word,
        simple_bus::{MEMORY_SIZE, SimpleBus},
    },
    cpu::{CPU, Flag, instructions::Instruction},
};

fn setup_cpu_bus() -> (CPU, SimpleBus) {
    let mut cpu = CPU::new();
    let bus = SimpleBus::default();
    cpu.reset();
    (cpu, bus)
}

#[test]
fn cpu_initialized_properly() {
    let (cpu, bus) = setup_cpu_bus();
    assert_eq!(cpu.pc, 0xFFFC);
    assert_eq!(cpu.sp, 0xFD);
    assert!(cpu.flag.is_empty());
    assert_eq!(bus.ram, [0x0; MEMORY_SIZE]);
}

#[test]
#[should_panic(expected = "unknown CPU instruction")]
fn invalid_cpu_instruction() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus.write(0xFFFC, 0x0);
    cpu.execute(1, &mut bus);
}

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

// * STA TESTS
#[test]
fn sta_zero_page_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaZPG.into();
    bus[0x0FFFD] = 0x42;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_zero_page_x_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaZPX.into();
    bus[0x0FFFD] = 0x40;
    cpu.x = 0x02;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaABS.into();
    bus[0x0FFFD] = 0x42;
    bus[0x0FFFE] = 0x37; // 0x3742
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x3742], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_x_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaABX.into();
    bus[0x0FFFD] = 0x37;
    bus[0x0FFFE] = 0x42; // 0x4237
    cpu.x = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(bus[0x4242], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_y_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaABY.into();
    bus[0x0FFFD] = 0x37;
    bus[0x0FFFE] = 0x42; // 0x4237
    cpu.y = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(bus[0x4242], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_indexed_indirect_x_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaIDX.into();
    bus[0x0FFFD] = 0x37;
    bus[0x0042] = 0x42;
    bus[0x0043] = 0x42;
    cpu.x = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(bus[0x4242], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_indirect_indexed_y_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StaIDY.into();
    bus[0x0FFFD] = 0x37;
    bus[0x0037] = 0x37;
    bus[0x0038] = 0x42;
    cpu.y = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(bus[0x4242], 42);
    assert!(cpu.flag.is_empty());
}
// * STX TESTS
#[test]
fn stx_zero_page_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StxZPG.into();
    bus[0x0FFFD] = 0x42;
    cpu.x = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn stx_zero_page_y_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StxZPY.into();
    bus[0x0FFFD] = 0x40;
    cpu.y = 0x02;
    cpu.x = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn stx_absolute_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StxABS.into();
    bus[0x0FFFD] = 0x42;
    bus[0x0FFFE] = 0x37; // 0x3742
    cpu.x = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x3742], 42);
    assert!(cpu.flag.is_empty());
}
// * STY TESTS
#[test]
fn sty_zero_page_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StyZPG.into();
    bus[0x0FFFD] = 0x42;
    cpu.y = 0x2A;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sty_zero_page_x_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StyZPX.into();
    bus[0x0FFFD] = 0x40;
    cpu.x = 0x02;
    cpu.y = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x42], 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sty_absolute_can_store_the_a_register_into_bus() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0x0FFFC] = Instruction::StyABS.into();
    bus[0x0FFFD] = 0x42;
    bus[0x0FFFE] = 0x37; // 0x3742
    cpu.y = 0x2A;
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(bus[0x3742], 42);
    assert!(cpu.flag.is_empty());
}

// * JSR TESTS
#[test]
fn jsr_absolute_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JsrABS.into();
    bus[0xFFFD] = 0x80;
    bus[0xFFFE] = 0x80;
    bus[0x8080] = Instruction::LdaIMM.into();
    bus[0x8081] = 0x2A;
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * RTS TESTS
#[test]
fn jsr_rts_load_value_to_register_a() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JsrABS.into();
    bus[0xFFFD] = 0x80;
    bus[0xFFFE] = 0x80;
    bus[0xFFFF] = Instruction::LdaIMM.into();
    bus[0x0000] = 0x2A;
    bus[0x8080] = Instruction::RtsIMP.into(); // return
    // 6 cycle to execute JSR instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // another 6 cycle to execute RTS instruction
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    // check if the RTS returned successfully
    assert_eq!(cpu.pc, 0xFFFF);
    // 2 cycle to execute LdaIMM instruction
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x2A);
    assert!(cpu.flag.is_empty());
}

// * JMP TESTS
#[test]
fn jmp_abs() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.pc, 0x4242);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpIND.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    bus[0x4242] = 0x37;
    bus[0x4243] = 0x2A;
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
}

#[test]
fn jmp_ind_to_lda() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::JmpIND.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x42;
    bus[0x4242] = 0x37;
    bus[0x4243] = 0x2A;
    bus[0x2A37] = Instruction::LdaIMM.into();
    bus[0x2A38] = 0x0;
    // Jump to ins
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.pc, 0x2A37);
    assert!(cpu.flag.is_empty());
    // Execute ins after jump
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

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
