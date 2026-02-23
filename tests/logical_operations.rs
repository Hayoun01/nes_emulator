mod common;
use bitflags::Flags;
use common::setup_cpu_bus;
use cpu_6502::{
    bus::Byte,
    cpu::{Flag, instructions::opcode::Opcode},
};

enum LogicalOp {
    And,
    Xor,
    OR,
}

fn do_logical_op(a: Byte, b: Byte, op: LogicalOp) -> Byte {
    match op {
        LogicalOp::And => a & b,
        LogicalOp::Xor => a ^ b,
        LogicalOp::OR => a | b,
    }
}

fn test_logical_op_on_a_register_immediate(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndIMM.into(),
            LogicalOp::Xor => Opcode::EorIMM.into(),
            LogicalOp::OR => Opcode::OraIMM.into(),
        },
    );
    cpu.write(0xFFFD, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 2);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_zero_page(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndZPG.into(),
            LogicalOp::Xor => Opcode::EorZPG.into(),
            LogicalOp::OR => Opcode::OraZPG.into(),
        },
    );
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_zero_page_x(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x02;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndZPX.into(),
            LogicalOp::Xor => Opcode::EorZPX.into(),
            LogicalOp::OR => Opcode::OraZPX.into(),
        },
    );
    cpu.write(0xFFFD, 0x40);
    cpu.write(0x0042, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndABS.into(),
            LogicalOp::Xor => Opcode::EorABS.into(),
            LogicalOp::OR => Opcode::OraABS.into(),
        },
    );
    cpu.write(0xFFFD, 0x37);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4237, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_x(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x02;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndABX.into(),
            LogicalOp::Xor => Opcode::EorABX.into(),
            LogicalOp::OR => Opcode::OraABX.into(),
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4237, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_x_page_crossed(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0xCB;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndABX.into(),
            LogicalOp::Xor => Opcode::EorABX.into(),
            LogicalOp::OR => Opcode::OraABX.into(),
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4300, 0x84); // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_y(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0x02;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndABY.into(),
            LogicalOp::Xor => Opcode::EorABY.into(),
            LogicalOp::OR => Opcode::OraABY.into(),
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4237, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_absolute_y_page_crossed(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0xCB;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndABY.into(),
            LogicalOp::Xor => Opcode::EorABY.into(),
            LogicalOp::OR => Opcode::OraABY.into(),
        },
    );
    cpu.write(0xFFFD, 0x35);
    cpu.write(0xFFFE, 0x42);
    cpu.write(0x4300, 0x84); // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indexed_indirect_x(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.x = 0x12;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndIDX.into(),
            LogicalOp::Xor => Opcode::EorIDX.into(),
            LogicalOp::OR => Opcode::OraIDX.into(),
        },
    );
    cpu.write(0xFFFD, 0x30);
    cpu.write(0x0042, 0x42);
    cpu.write(0x0043, 0x42);
    cpu.write(0x4242, 0x84); // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indirect_indexed_y(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0x12;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndIDY.into(),
            LogicalOp::Xor => Opcode::EorIDY.into(),
            LogicalOp::OR => Opcode::OraIDY.into(),
        },
    );
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x30);
    cpu.write(0x0043, 0x42);
    cpu.write(0x4242, 0x84);
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 5);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

fn test_logical_op_on_a_register_indirect_indexed_y_page_crossed(op: LogicalOp) {
    let mut cpu = setup_cpu_bus();
    cpu.a = 0xCC;
    cpu.y = 0xD0;
    cpu.write(
        0xFFFC,
        match op {
            LogicalOp::And => Opcode::AndIDY.into(),
            LogicalOp::Xor => Opcode::EorIDY.into(),
            LogicalOp::OR => Opcode::OraIDY.into(),
        },
    );
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x30);
    cpu.write(0x0043, 0x42); // 0x4230 + 0x00D0 = 0x4300
    cpu.write(0x4300, 0x84); // page crossed!
    let res = do_logical_op(cpu.a, 0x84, op);
    let excepted_flag = if (res & 0b10000000) > 0 {
        Flag::NEGATIVE.bits()
    } else {
        Flag::empty().bits()
    };
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 6);
    assert_eq!(cpu.a, res);
    assert_eq!(cpu.flag.bits(), excepted_flag);
}

// * AND TESTS
#[test]
fn and_imm_can_and_with_a_register() {
    test_logical_op_on_a_register_immediate(LogicalOp::And);
}

#[test]
fn and_zpg_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page(LogicalOp::And);
}

#[test]
fn and_zpx_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page_x(LogicalOp::And);
}

#[test]
fn and_abs_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute(LogicalOp::And);
}

#[test]
fn and_abs_x_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x(LogicalOp::And);
}

#[test]
fn and_abs_x_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x_page_crossed(LogicalOp::And);
}

#[test]
fn and_abs_y_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y(LogicalOp::And);
}

#[test]
fn and_abs_y_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y_page_crossed(LogicalOp::And);
}

#[test]
fn and_idx_can_and_with_a_register() {
    test_logical_op_on_a_register_indexed_indirect_x(LogicalOp::And);
}

#[test]
fn and_idy_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y(LogicalOp::And);
}

#[test]
fn and_idy_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y_page_crossed(LogicalOp::And);
}

// * EOR TESTS
#[test]
fn eor_imm_can_and_with_a_register() {
    test_logical_op_on_a_register_immediate(LogicalOp::Xor);
}

#[test]
fn eor_zpg_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page(LogicalOp::Xor);
}

#[test]
fn eor_zpx_can_and_with_a_register() {
    test_logical_op_on_a_register_zero_page_x(LogicalOp::Xor);
}

#[test]
fn eor_abs_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute(LogicalOp::Xor);
}

#[test]
fn eor_abs_x_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x(LogicalOp::Xor);
}

#[test]
fn eor_abs_x_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_x_page_crossed(LogicalOp::Xor);
}

#[test]
fn eor_abs_y_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y(LogicalOp::Xor);
}

#[test]
fn eor_abs_y_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_absolute_y_page_crossed(LogicalOp::Xor);
}

#[test]
fn eor_idx_can_and_with_a_register() {
    test_logical_op_on_a_register_indexed_indirect_x(LogicalOp::Xor);
}

#[test]
fn eor_idy_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y(LogicalOp::Xor);
}

#[test]
fn eor_idy_page_crossed_can_and_with_a_register() {
    test_logical_op_on_a_register_indirect_indexed_y_page_crossed(LogicalOp::Xor);
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
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x2A);
    cpu.a = 0x2A;
    cpu.flag.insert(Flag::ZERO);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert!(cpu.flag.is_empty());
}

#[test]
fn test_bit_zero_page_value_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x0);
    cpu.a = 0x2A;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn test_bit_zero_page_value_with_6_and_7_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0xCC);
    cpu.a = 0x33;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(
        cpu.flag.bits(),
        (Flag::OVERFLOW | Flag::NEGATIVE | Flag::ZERO).bits()
    );
}

#[test]
fn test_bit_zero_page_value_with_6_and_7_mixed() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitZPG.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0x0042, 0x40);
    cpu.a = 0xCC;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 3);
    assert_eq!(cpu.flag.bits(), Flag::OVERFLOW.bits());
}

#[test]
fn test_bit_absolute() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x37);
    cpu.write(0x3742, 0x2A);
    cpu.a = 0x2A;
    cpu.flag.insert(Flag::ZERO);
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert!(cpu.flag.is_empty());
}

#[test]
fn test_bit_absolute_value_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x37);
    cpu.write(0x3742, 0x0);
    cpu.a = 0x2A;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
}

#[test]
fn test_bit_absolute_value_with_6_and_7_zero() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x37);
    cpu.write(0x3742, 0xCC);
    cpu.a = 0x33;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(
        cpu.flag.bits(),
        (Flag::OVERFLOW | Flag::NEGATIVE | Flag::ZERO).bits()
    );
}

#[test]
fn test_bit_absolute_value_with_6_and_7_mixed() {
    let mut cpu = setup_cpu_bus();
    cpu.write(0xFFFC, Opcode::BitABS.into());
    cpu.write(0xFFFD, 0x42);
    cpu.write(0xFFFE, 0x37);
    cpu.write(0x3742, 0x40);
    cpu.a = 0xCC;
    cpu.flag.clear();
    let cycle_used = cpu.execute();
    assert_eq!(cycle_used, 4);
    assert_eq!(cpu.flag.bits(), Flag::OVERFLOW.bits());
}
