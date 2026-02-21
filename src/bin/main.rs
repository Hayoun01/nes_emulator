use std::collections::BTreeMap;

use cpu_6502::{
    bus::{Byte, Word, simple_bus::SimpleBus},
    cpu::{CPU, Flag},
};
use raylib::prelude::*;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

fn setup_cpu_bus() -> CPU {
    let mut cpu = CPU::new();
    let bus = SimpleBus::default();
    cpu.connect_bus(Box::new(bus));
    cpu.reset();
    cpu
}

struct DemoCPU {
    cpu: CPU,
    map: BTreeMap<Word, String>,
}

impl DemoCPU {
    fn new() -> Self {
        Self {
            cpu: setup_cpu_bus(),
            map: BTreeMap::new(),
        }
    }

    fn init(&mut self) {
        let program: [Byte; _] = [
            0x00, 0x10, 0xa9, 0x01, 0xe6, 0x2a, 0x45, 0x2a, 0xf0, 0xf8, 0xc6, 0x37, 0x08, 0xa9,
            0xcc, 0x85, 0x42, 0xa9, 0x33, 0x24, 0x42, 0x28, 0x4c, 0x00, 0x10,
        ];
        self.cpu.load_program(&program);
        self.cpu.pc = 0x1000 as Word;
        self.map = self.cpu.disassemble(0x0000, 0xFFFF);
    }
    fn draw_cpu(&mut self, d: &mut RaylibDrawHandle, origin_x: i32, origin_y: i32) {
        let next_y = |n| origin_y + (HEIGHT / 2) + (n * 20);
        self.draw_memory(d, origin_x + 10, origin_y);
        self.draw_cpu_status(d, origin_x, next_y(0));
        d.draw_text(
            format!("PC: ${:04X}", self.cpu.pc).as_str(),
            origin_x + 10,
            next_y(1),
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("A: ${:02X} [{:03}]", self.cpu.a, self.cpu.a).as_str(),
            origin_x + 10,
            next_y(2),
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("X: ${:02X} [{:03}]", self.cpu.x, self.cpu.x).as_str(),
            origin_x + 10,
            next_y(3),
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("Y: ${:02X} [{:03}]", self.cpu.y, self.cpu.y).as_str(),
            origin_x + 10,
            next_y(4),
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("SP: ${:02X}", self.cpu.sp).as_str(),
            origin_x + 10,
            next_y(5),
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("CYCLES: {}", self.cpu.general_cycles).as_str(),
            origin_x + 10,
            next_y(6),
            18,
            Color::WHITE,
        );
        self.draw_code(d, 10, origin_x, next_y(7));
    }

    fn draw_code(&self, d: &mut RaylibDrawHandle, lines: i32, origin_x: i32, origin_y: i32) {
        let mut line_y = (lines >> 1) * 20 + origin_y;
        if let Some((_, first_line)) = self.map.range(self.cpu.pc..).next() {
            d.draw_text(first_line, origin_x + 10, line_y, 18, Color::CYAN);

            for (_, line) in self.map.range((&self.cpu.pc + 1)..) {
                if line_y >= (lines * 20) + origin_y {
                    break;
                }
                line_y += 20;
                d.draw_text(line, origin_x + 10, line_y, 18, Color::WHITE);
            }
        }
        line_y = (lines >> 1) * 20 + origin_y;
        for (_, line) in self.map.range(..self.cpu.pc).rev() {
            if line_y <= origin_y {
                break;
            }
            line_y -= 20;
            d.draw_text(line, origin_x + 10, line_y, 18, Color::WHITE);
        }
    }
    fn draw_cpu_status(&self, d: &mut RaylibDrawHandle, origin_x: i32, origin_y: i32) {
        let next_x = |n| origin_x + 100 + (n * 20);
        let get_status_color = |flag_to_test| {
            if self.cpu.flag.contains(flag_to_test) {
                Color::GREEN
            } else {
                Color::RED
            }
        };
        d.draw_text("STATUS: ", origin_x + 10, origin_y, 18, Color::WHITE);
        d.draw_text(
            "N",
            next_x(0),
            origin_y,
            18,
            get_status_color(Flag::NEGATIVE),
        );
        d.draw_text(
            "V",
            next_x(1),
            origin_y,
            18,
            get_status_color(Flag::OVERFLOW),
        );
        d.draw_text("-", next_x(2), origin_y, 18, Color::GREEN);
        d.draw_text(
            "B",
            next_x(3),
            origin_y,
            18,
            get_status_color(Flag::BREAK_COMMAND),
        );
        d.draw_text(
            "D",
            next_x(4),
            origin_y,
            18,
            get_status_color(Flag::DECIMAL_MODE),
        );
        d.draw_text(
            "I",
            next_x(5),
            origin_y,
            18,
            get_status_color(Flag::INTERRUPT_DISABLE),
        );
        d.draw_text("Z", next_x(6), origin_y, 18, get_status_color(Flag::ZERO));
        d.draw_text("C", next_x(7), origin_y, 18, get_status_color(Flag::CARRY));
    }
    fn draw_memory(&mut self, d: &mut RaylibDrawHandle, origin_x: i32, mut origin_y: i32) {
        origin_y += 5;
        d.draw_line_ex(
            Vector2::new(origin_x as f32, origin_y as f32),
            Vector2::new((origin_x + WIDTH / 2 - 100) as f32, origin_y as f32),
            2.5,
            Color::BLUEVIOLET,
        );
        d.draw_line_ex(
            Vector2::new(origin_x as f32, origin_y as f32),
            Vector2::new(origin_x as f32, (origin_y + HEIGHT / 2 - 10) as f32),
            2.5,
            Color::BLUEVIOLET,
        );
        d.draw_line_ex(
            Vector2::new(origin_x as f32, (origin_y + HEIGHT / 2 - 10) as f32),
            Vector2::new(
                (origin_x + WIDTH / 2 - 100) as f32,
                (origin_y + HEIGHT / 2 - 10) as f32,
            ),
            2.5,
            Color::BLUEVIOLET,
        );
        d.draw_line_ex(
            Vector2::new((origin_x + WIDTH / 2 - 100) as f32, origin_y as f32),
            Vector2::new(
                (origin_x + WIDTH / 2 - 100) as f32,
                (origin_y + HEIGHT / 2 - 10) as f32,
            ),
            2.5,
            Color::BLUEVIOLET,
        );
        origin_y += 5;
        for addr in (0..=0xFF).step_by(16) {
            d.draw_text(
                format!("${:04X}: ", addr).as_str(),
                origin_x + 5,
                origin_y,
                18,
                Color::WHITE,
            );
            for i in 0..16 {
                d.draw_text(
                    format!("{:02X}", self.cpu.read_byte(addr + i)).as_str(),
                    origin_x + 70 + (25 * i as i32),
                    origin_y,
                    18,
                    Color::WHITE,
                );
            }
            origin_y += 20;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Emulator").build();
    let mut demo = DemoCPU::new();
    demo.init();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.set_window_title(&thread, &format!("Emulator FPS: {}", rl.get_fps()));
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || rl.is_key_pressed_repeat(KeyboardKey::KEY_SPACE)
        {
            demo.cpu.execute();
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
        demo.draw_cpu(&mut d, HEIGHT, 0);
        d.draw_fps(0, 0);
    }
}
