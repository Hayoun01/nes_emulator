use cpu_6502::{bus::simple_bus::SimpleBus, cpu::CPU};


pub fn setup_cpu_bus() -> CPU {
    let mut cpu = CPU::new();
    let bus = SimpleBus::default();
    cpu.connect_bus(Box::new(bus));
    cpu.reset();
    cpu
}
