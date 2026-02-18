mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::instructions::Opcode;

// * STA TESTS

#[test]
fn sta_zero_page_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaZPG.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_zero_page_x_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaZPX.into());
    cpu.write(0x0FFFD, 0x40);
    cpu.x = 0x02;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaABS.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.write(0x0FFFE, 0x37); // 0x3742
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x3742), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_x_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaABX.into());
    cpu.write(0x0FFFD, 0x37);
    cpu.write(0x0FFFE, 0x42); // 0x4237
    cpu.x = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x4242), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_absolute_y_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaABY.into());
    cpu.write(0x0FFFD, 0x37);
    cpu.write(0x0FFFE, 0x42); // 0x4237
    cpu.y = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.read_byte(0x4242), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_indexed_indirect_x_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaIDX.into());
    cpu.write(0x0FFFD, 0x37);
    cpu.write(0x0042, 0x42);
    cpu.write(0x0043, 0x42);
    cpu.x = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x4242), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sta_indirect_indexed_y_can_store_the_a_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StaIDY.into());
    cpu.write(0x0FFFD, 0x37);
    cpu.write(0x0037, 0x37);
    cpu.write(0x0038, 0x42);
    cpu.y = 0xB;
    cpu.a = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.read_byte(0x4242), 42);
    assert!(cpu.flag.is_empty());
}

// * STX TESTS

#[test]
fn stx_zero_page_can_store_the_x_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StxZPG.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.x = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn stx_zero_page_y_can_store_the_x_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StxZPY.into());
    cpu.write(0x0FFFD, 0x40);
    cpu.y = 0x02;
    cpu.x = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn stx_absolute_can_store_the_x_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StxABS.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.write(0x0FFFE, 0x37); // 0x3742
    cpu.x = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x3742), 42);
    assert!(cpu.flag.is_empty());
}

// * STY TESTS

#[test]
fn sty_zero_page_can_store_the_y_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StyZPG.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.y = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sty_zero_page_x_can_store_the_y_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StyZPX.into());
    cpu.write(0x0FFFD, 0x40);
    cpu.x = 0x02;
    cpu.y = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x42), 42);
    assert!(cpu.flag.is_empty());
}

#[test]
fn sty_absolute_can_store_the_y_register_into_memory() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0x0FFFC, Opcode::StyABS.into());
    cpu.write(0x0FFFD, 0x42);
    cpu.write(0x0FFFE, 0x37); // 0x3742
    cpu.y = 0x2A;
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.read_byte(0x3742), 42);
    assert!(cpu.flag.is_empty());
}
