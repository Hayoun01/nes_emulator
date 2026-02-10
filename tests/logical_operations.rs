mod common;
use bitflags::Flags;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::Byte,
    cpu::{Flag, instructions::Instruction},
};

enum LogicalOp {
    AND,
    XOR,
    OR,
}

fn do_logical_op(a: Byte, b: Byte, op: LogicalOp) -> Byte {
    match op {
        LogicalOp::AND => a & b,
        LogicalOp::XOR => a ^ b,
        LogicalOp::OR => a | b,
    }
}

fn test_logical_op_on_a_register_immediate(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndIMM.into(),
        LogicalOp::XOR => Instruction::EorIMM.into(),
        LogicalOp::OR => Instruction::OraIMM.into(),
    };
    bus[0xFFFD] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(2, &mut bus);
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_zero_page(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndZPG.into(),
        LogicalOp::XOR => Instruction::EorZPG.into(),
        LogicalOp::OR => Instruction::OraZPG.into(),
    };
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_zero_page_x(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x02;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndZPX.into(),
        LogicalOp::XOR => Instruction::EorZPX.into(),
        LogicalOp::OR => Instruction::OraZPX.into(),
    };
    bus[0xFFFD] = 0x40;
    bus[0x0042] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndABS.into(),
        LogicalOp::XOR => Instruction::EorABS.into(),
        LogicalOp::OR => Instruction::OraABS.into(),
    };
    bus[0xFFFD] = 0x37;
    bus[0xFFFE] = 0x42;
    bus[0x4237] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_x(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x02;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndABX.into(),
        LogicalOp::XOR => Instruction::EorABX.into(),
        LogicalOp::OR => Instruction::OraABX.into(),
    };
    bus[0xFFFD] = 0x35;
    bus[0xFFFE] = 0x42;
    bus[0x4237] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_x_page_crossed(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0xCB;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndABX.into(),
        LogicalOp::XOR => Instruction::EorABX.into(),
        LogicalOp::OR => Instruction::OraABX.into(),
    };
    bus[0xFFFD] = 0x35;
    bus[0xFFFE] = 0x42;
    bus[0x4300] = 0x84; // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_y(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0x02;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndABY.into(),
        LogicalOp::XOR => Instruction::EorABY.into(),
        LogicalOp::OR => Instruction::OraABY.into(),
    };
    bus[0xFFFD] = 0x35;
    bus[0xFFFE] = 0x42;
    bus[0x4237] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_y_page_crossed(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0xCB;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndABY.into(),
        LogicalOp::XOR => Instruction::EorABY.into(),
        LogicalOp::OR => Instruction::OraABY.into(),
    };
    bus[0xFFFD] = 0x35;
    bus[0xFFFE] = 0x42;
    bus[0x4300] = 0x84; // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indexed_indirect_x(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x12;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndIDX.into(),
        LogicalOp::XOR => Instruction::EorIDX.into(),
        LogicalOp::OR => Instruction::OraIDX.into(),
    };
    bus[0xFFFD] = 0x30;
    bus[0x0042] = 0x42;
    bus[0x0043] = 0x42;
    bus[0x4242] = 0x84; // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indirect_indexed_y(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0x12;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndIDY.into(),
        LogicalOp::XOR => Instruction::EorIDY.into(),
        LogicalOp::OR => Instruction::OraIDY.into(),
    };
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x30;
    bus[0x0043] = 0x42;
    bus[0x4242] = 0x84;
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(5, &mut bus);
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indirect_indexed_y_page_crossed(op: LogicalOp) {
    let (mut cpu, mut bus) = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0xD0;
    bus[0xFFFC] = match op {
        LogicalOp::AND => Instruction::AndIDY.into(),
        LogicalOp::XOR => Instruction::EorIDY.into(),
        LogicalOp::OR => Instruction::OraIDY.into(),
    };
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x30;
    bus[0x0043] = 0x42; // 0x4230 + 0x00D0 = 0x4300
    bus[0x4300] = 0x84; // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute(6, &mut bus);
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

// * AND TESTS
#[test]
fn and_imm_can_and_with_a_register() {
    test_logical_op_on_a_register_immediate(LogicalOp::AND);
}

#[test]
fn and_zpg_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page(LogicalOp::AND);
}

#[test]
fn and_zpx_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page_x(LogicalOp::AND);
}

#[test]
fn and_abs_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute(LogicalOp::AND);
}

#[test]
fn and_abs_x_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x(LogicalOp::AND);
}

#[test]
fn and_abs_x_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x_page_crossed(LogicalOp::AND);
}

#[test]
fn and_abs_y_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y(LogicalOp::AND);
}

#[test]
fn and_abs_y_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y_page_crossed(LogicalOp::AND);
}

#[test]
fn and_idx_can_and_with_a_register() {
    test_logical_op_on_a_register_indexed_indirect_x(LogicalOp::AND);
}

#[test]
fn and_idy_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y(LogicalOp::AND);
}

#[test]
fn and_idy_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y_page_crossed(LogicalOp::AND);
}

// * EOR TESTS
#[test]
fn eor_imm_can_and_with_a_register() {
    test_logical_op_on_a_register_immediate(LogicalOp::XOR);
}

#[test]
fn eor_zpg_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page(LogicalOp::XOR);
}

#[test]
fn eor_zpx_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page_x(LogicalOp::XOR);
}

#[test]
fn eor_abs_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute(LogicalOp::XOR);
}

#[test]
fn eor_abs_x_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x(LogicalOp::XOR);
}

#[test]
fn eor_abs_x_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x_page_crossed(LogicalOp::XOR);
}

#[test]
fn eor_abs_y_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y(LogicalOp::XOR);
}

#[test]
fn eor_abs_y_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y_page_crossed(LogicalOp::XOR);
}

#[test]
fn eor_idx_can_and_with_a_register() {
    test_logical_op_on_a_register_indexed_indirect_x(LogicalOp::XOR);
}

#[test]
fn eor_idy_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y(LogicalOp::XOR);
}

#[test]
fn eor_idy_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y_page_crossed(LogicalOp::XOR);
}

// * ORA TESTS
#[test]
fn ora_imm_can_and_with_a_register() {
    test_logical_op_on_a_register_immediate(LogicalOp::OR);
}

#[test]
fn ora_zpg_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page(LogicalOp::OR);
}

#[test]
fn ora_zpx_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page_x(LogicalOp::OR);
}

#[test]
fn ora_abs_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute(LogicalOp::OR);
}

#[test]
fn ora_abs_x_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x(LogicalOp::OR);
}

#[test]
fn ora_abs_x_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x_page_crossed(LogicalOp::OR);
}

#[test]
fn ora_abs_y_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y(LogicalOp::OR);
}

#[test]
fn ora_abs_y_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y_page_crossed(LogicalOp::OR);
}

#[test]
fn ora_idx_can_and_with_a_register() {
    test_logical_op_on_a_register_indexed_indirect_x(LogicalOp::OR);
}

#[test]
fn ora_idy_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y(LogicalOp::OR);
}

#[test]
fn ora_idy_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y_page_crossed(LogicalOp::OR);
}

#[test]
fn test_bit_zero_page() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x2A;
    cpu.a = 0x2A;
    cpu.flag.insert(Flag::ZERO);
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert!(cpu.flag.is_empty());
}

#[test]
fn test_bit_zero_page_value_zero() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x0;
    cpu.a = 0x2A;
    cpu.flag.clear();
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn test_bit_zero_page_value_with_6_and_7_zero() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0xCC;
    cpu.a = 0x33;
    cpu.flag.clear();
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(
        cpu.flag.bits(),
        (Flag::OVERFLOW | Flag::NEGATIVE | Flag::ZERO).bits()
    );
}

#[test]
fn test_bit_zero_page_value_with_6_and_7_mixed() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitZPG.into();
    bus[0xFFFD] = 0x42;
    bus[0x0042] = 0x40;
    cpu.a = 0xCC;
    cpu.flag.clear();
    let cycle_used = cpu.execute(3, &mut bus);
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.flag.bits(), Flag::OVERFLOW.bits());
}

#[test]
fn test_bit_absolute() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x37;
    bus[0x3742] = 0x2A;
    cpu.a = 0x2A;
    cpu.flag.insert(Flag::ZERO);
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert!(cpu.flag.is_empty());
}

#[test]
fn test_bit_absolute_value_zero() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x37;
    bus[0x3742] = 0x0;
    cpu.a = 0x2A;
    cpu.flag.clear();
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn test_bit_absolute_value_with_6_and_7_zero() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x37;
    bus[0x3742] = 0xCC;
    cpu.a = 0x33;
    cpu.flag.clear();
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(
        cpu.flag.bits(),
        (Flag::OVERFLOW | Flag::NEGATIVE | Flag::ZERO).bits()
    );
}

#[test]
fn test_bit_absolute_value_with_6_and_7_mixed() {
    let (mut cpu, mut bus) = setup_cpu_bus();
    bus[0xFFFC] = Instruction::BitABS.into();
    bus[0xFFFD] = 0x42;
    bus[0xFFFE] = 0x37;
    bus[0x3742] = 0x40;
    cpu.a = 0xCC;
    cpu.flag.clear();
    let cycle_used = cpu.execute(4, &mut bus);
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), Flag::OVERFLOW.bits());
}
