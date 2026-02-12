use cpu_6502::{
    bus::{Byte, Word, simple_bus::SimpleBus},
    cpu::{CPU, Flag},
};
use raylib::prelude::*;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

fn setup_cpu_bus() -> (CPU, SimpleBus) {
    let mut cpu = CPU::new();
    let bus = SimpleBus::default();
    cpu.reset();
    (cpu, bus)
}

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Emulator").build();
    let (mut cpu, mut bus) = setup_cpu_bus();
    let program: [Byte; _] = [
        0x00, 0x10, 0xa9, 0xcc, 0x85, 0x42, 0xa9, 0x33, 0x24, 0x42, 0x4c, 0x00, 0x10,
    ];
    cpu.load_program(&program, &mut bus);
    let mut addr = 0x1000 as Word;
    cpu.pc = addr;
    for &v in program.iter().skip(2) {
        assert_eq!(bus[addr], v);
        addr += 1;
    }

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || rl.is_key_pressed_repeat(KeyboardKey::KEY_SPACE)
        {
            cpu.execute(1, &mut bus);
        }
        let mut d = rl.begin_drawing(&thread);
        let width = d.get_screen_width();
        let height = d.get_screen_height();
        d.clear_background(Color::DEEPSKYBLUE);
        d.draw_rectangle(0, 0, height, height, Color::BLACK);
        d.draw_rectangle(
            height,
            0,
            width - height,
            height,
            Color {
                r: 65,
                g: 65,
                b: 65,
                a: 255,
            },
        );
        draw_cpu(&mut d, &cpu, HEIGHT, 0);
    }
}

fn draw_cpu_status(d: &mut RaylibDrawHandle, flag: &Flag, origin_x: i32, origin_y: i32) {
    let next = |n| origin_x + 100 + (n * 20);
    let get_status_color = |flag_to_test| {
        if flag.contains(flag_to_test) {
            Color::GREEN
        } else {
            Color::RED
        }
    };
    d.draw_text("STATUS: ", origin_x + 10, origin_y, 18, Color::WHITE);
    d.draw_text("N", next(0), origin_y, 18, get_status_color(Flag::NEGATIVE));
    d.draw_text("V", next(1), origin_y, 18, get_status_color(Flag::OVERFLOW));
    d.draw_text("-", next(2), origin_y, 18, Color::GREEN);
    d.draw_text(
        "B",
        next(3),
        origin_y,
        18,
        get_status_color(Flag::BREAK_COMMAND),
    );
    d.draw_text(
        "D",
        next(4),
        origin_y,
        18,
        get_status_color(Flag::DECIMAL_MODE),
    );
    d.draw_text(
        "I",
        next(5),
        origin_y,
        18,
        get_status_color(Flag::INTERRUPT_DISABLE),
    );
    d.draw_text("Z", next(6), origin_y, 18, get_status_color(Flag::ZERO));
    d.draw_text("C", next(7), origin_y, 18, get_status_color(Flag::CARRY));
}

fn draw_cpu(d: &mut RaylibDrawHandle, cpu: &CPU, origin_x: i32, origin_y: i32) {
    let next = |n| origin_y + (HEIGHT / 2) + (n * 20);
    draw_cpu_status(d, &cpu.flag, origin_x, next(0));
    d.draw_text(
        format!("PC: ${:X}", cpu.pc).as_str(),
        origin_x + 10,
        next(1),
        18,
        Color::WHITE,
    );
    d.draw_text(
        format!("A: ${:X} [{}]", cpu.a, cpu.a).as_str(),
        origin_x + 10,
        next(2),
        18,
        Color::WHITE,
    );
    d.draw_text(
        format!("X: ${:X} [{}]", cpu.x, cpu.x).as_str(),
        origin_x + 10,
        next(3),
        18,
        Color::WHITE,
    );
    d.draw_text(
        format!("Y: ${:X} [{}]", cpu.y, cpu.y).as_str(),
        origin_x + 10,
        next(4),
        18,
        Color::WHITE,
    );
    d.draw_text(
        format!("SP: ${:X}", cpu.sp).as_str(),
        origin_x + 10,
        next(5),
        18,
        Color::WHITE,
    );
}
