mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::instructions::Instruction;

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
