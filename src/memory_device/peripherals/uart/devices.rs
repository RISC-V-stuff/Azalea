use crate::memory_device::{
    Access, AccessSize, MemFault, MemResult, MemoryDevice, peripherals::uart::UartBackend,
};

// Models a very simple uart:
//  - offset 0x00: read write byte interface
//  - offset 0x01: reads return 1 if there are bytes to read, writes are ignored
pub struct SimpleUart {
    base: u64,
    backend: Box<dyn super::UartBackend>,
}

impl SimpleUart {
    pub fn new(base: u64, backend: impl UartBackend + 'static) -> Self {
        Self {
            base,
            backend: Box::new(backend),
        }
    }
}
impl MemoryDevice for SimpleUart {
    fn load(&mut self, addr: u64, _size: AccessSize) -> MemResult<Access> {
        let value = match addr - self.base {
            0x00 => self.backend.read_byte().unwrap_or(0) as u32,
            0x04 => self.backend.rx_ready() as u32,
            _ => 0,
        };
        Ok(Access {
            value: value as u64,
            cycles: 0,
        })
    }

    fn store(&mut self, addr: u64, _size: AccessSize, value: u64) -> MemResult<u64> {
        if addr - self.base == 0x00 {
            self.backend.write_byte(value as u8);
            Ok(0)
        } else {
            Err(MemFault::Misaligned)
        }
    }

    fn contains_addr(&self, addr: u64) -> bool {
        if addr < self.base {
            return false;
        }
        let off = addr - self.base;
        off < 8
    }

    fn advance_clock(&mut self, _cycles: u32) {
        ()
    }
}
