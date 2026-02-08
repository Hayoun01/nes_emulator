use cpu_6502::{bus::simple_bus::SimpleBus, cpu::CPU};


pub fn setup_cpu_bus() -> (CPU, SimpleBus) {
    let mut cpu = CPU::new();
    let bus = SimpleBus::default();
    cpu.reset();
    (cpu, bus)
}
