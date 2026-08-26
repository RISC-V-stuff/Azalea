use std::fs::File;

use azalea::cpu::Cpu32;
use azalea::loader::load;
use azalea::memory_device::*;

use azalea::{
    hooks::{BranchKind, CpuHooks},
    instructions::Instruction,
};
use ctxp::Format::Text;
use ctxp::{Event, Source};
pub struct BranchTrace {
    encoder: ctxp::Encoder<File>,
}

impl BranchTrace {
    pub fn new() -> Self {
        let f = File::create_new("Test_output.ctxp.txt").unwrap();
        let src = Source {
            id: 1,
            name: "Test".into(),
        };

        Self {
            encoder: ctxp::Encoder::new(f, &[src], Text).unwrap(),
        }
    }
}

impl CpuHooks for BranchTrace {
    fn on_run(&mut self) {}

    fn on_instruction(&mut self, pc: u32, instr: &Instruction) {}

    fn on_mem_read(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {}

    fn on_mem_write(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {}

    fn on_branch(&mut self, pc: u32, target: u32, taken: bool, kind: BranchKind) {
        if taken {
            let event = Event {
                source_id: 1,
                kind: ctxp::EventKind::BranchTaken {
                    origin: pc as u64,
                    target: target as u64,
                },
                cycle: None,
            };
            self.encoder.write_event(&event).unwrap();
        } else {
            let event = Event {
                source_id: 1,
                kind: ctxp::EventKind::BranchNotTaken {
                    origin: pc as u64,
                    target: target as u64,
                },
                cycle: None,
            };
            self.encoder.write_event(&event).unwrap();
        }
    }
}

mod instructions;

fn main() {
    let mut cpu = Cpu32::new();
    let ram = BasicRam::new(0x80000000, 65 * 1024);
    let uart = peripherals::uart::SimpleUart::new(
        0xFF000000,
        peripherals::uart::backends::TcpBackend::bind("127.0.0.1:5555"),
    );

    let mut bus = Bus::new();

    bus.add_device(ram);
    bus.add_device(uart);

    let start = load(
        "F:/lab/rust/Azalea-Emulator/test-app/target/riscv32i-unknown-none-elf/debug/test-app",
        &mut bus,
    );

    let mut hooks = BranchTrace::new();
    //hooks.set_next(Box::new(BranchTrace::new()));

    cpu.run(start, &mut bus, &mut hooks);
    println!("Core paused execution by executing a system instruction.");
    loop {}
}
