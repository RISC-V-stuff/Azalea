mod memories;
pub use memories::*;

pub mod peripherals;

mod bus;
pub use bus::*;

#[derive(Copy, Clone)]
pub enum AccessSize {
    Byte,
    Half,
    Word,
}

pub trait MemoryDevice {
    fn contains_addr(&self, addr: u32) -> bool;
    fn load(&mut self, addr: u32, size: AccessSize) -> u32;
    fn store(&mut self, addr: u32, size: AccessSize, value: u32);
}

// For "cycle accurate" simulation
// add a function advance_clock(cycles) that way you can inform a decice of a time step,
// reads will return a (value, cycles) pair with the latency, the return is inmediate, the receiver
// can decide to model that latency in its behaviour or not, that way buses or similar passthrough devices
// can decice to add latency to each operation and similar behaviours similarly you can model clock domains by
// implementing a CDC that impls MemoryDevice and holds a Memory device (likely a bus) and that way you can perform
// unit conversion between the cycles.
