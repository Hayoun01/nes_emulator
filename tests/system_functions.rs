mod common;
use common::setup_cpu_bus;
use cpu_6502::cpu::{Flag, instructions::opcode::Opcode};

// * NOP

#[test]
fn nop_do_nothing_but_consumes_cycles() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::NopIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.pc, 0xFFFD);
    assert_eq!(cpu.flag.bits(), Flag::empty().bits());
}

#[test]
fn brk_load_the_program_counter_from_interrupt_vector() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BrkIMP.into());
    cpu.write(0xFFFE, 0x37);
    cpu.write(0xFFFF, 0x13);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.pc, 0x1337);
}

#[test]
fn brk_will_set_the_break_flag() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BrkIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(cpu.flag.bits(), Flag::INTERRUPT_DISABLE.bits());
}

#[test]
fn brk_will_push_pc_and_flag_to_stack() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::ZERO | Flag::OVERFLOW);
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BrkIMP.into());
    cpu.write(0xFFFE, 0x37);
    cpu.write(0xFFFF, 0x13);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    assert_eq!(
        cpu.read_byte(cpu.stack_addr() + 1),
        (Flag::ZERO | Flag::BREAK_COMMAND | Flag::OVERFLOW).bits()
    );
    assert_eq!(cpu.read_byte(cpu.stack_addr() + 2), 0x02);
    assert_eq!(cpu.read_byte(cpu.stack_addr() + 3), 0xFF);
    assert!(cpu.flag.contains(Flag::INTERRUPT_DISABLE));
}

#[test]
fn rti_can_return_from_an_interrupt_leaving_the_cpu_in_the_state_when_it_entered() {
    let mut cpu = setup_cpu_bus();
    cpu.flag.insert(Flag::ZERO | Flag::OVERFLOW);
    cpu.pc = 0xFF00;
    cpu.write(0xFF00, Opcode::BrkIMP.into());
    cpu.write(0xFFFE, 0x37);
    cpu.write(0xFFFF, 0x13);
    cpu.write(0x1337, Opcode::RtiIMP.into());
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 7);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.flag.bits(), (Flag::ZERO | Flag::OVERFLOW).bits());
    assert_eq!(cpu.pc, 0xFF02);
}
