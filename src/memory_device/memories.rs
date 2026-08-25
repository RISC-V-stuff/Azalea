use super::*;

pub struct BasicRam {
    start: u32,
    data: Vec<u8>,
}

impl BasicRam {
    pub fn new(start_addr: u32, size: usize) -> Self {
        Self {
            start: start_addr,
            data: vec![0; size],
        }
    }
}

impl MemoryDevice for BasicRam {
    fn load(&mut self, addr: u32, size: AccessSize) -> MemResult<Access> {
        let i = (addr - self.start) as usize;
        let cycles = 0;
        let value = match size {
            AccessSize::Byte => self.data[i] as u32,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u32
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes)
            }
        };
        Ok(Access { value, cycles })
    }

    fn store(&mut self, addr: u32, size: AccessSize, value: u32) -> MemResult<u32> {
        let i = (addr - self.start) as usize;
        let cycles = 0;
        match size {
            AccessSize::Byte => {
                self.data[i] = value as u8;
            }
            AccessSize::Half => {
                self.data[i..i + 2].copy_from_slice(&(value as u16).to_le_bytes());
            }
            AccessSize::Word => {
                self.data[i..i + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
        Ok(cycles)
    }

    fn contains_addr(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u32
    }

    fn advance_clock(&mut self, _cycles: u32) {
        ()
    }
}

pub struct BasicRom {
    start: u32,
    data: Vec<u8>,
}

impl BasicRom {
    pub fn new(start_addr: u32, data: &[u8]) -> Self {
        Self {
            start: start_addr,
            data: data.into(),
        }
    }
}

impl MemoryDevice for BasicRom {
    fn load(&mut self, addr: u32, size: AccessSize) -> MemResult<Access> {
        let i = (addr - self.start) as usize;
        let cycles = 0;
        let value = match size {
            AccessSize::Byte => self.data[i] as u32,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u32
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes)
            }
        };
        Ok(Access { value, cycles })
    }

    fn store(&mut self, addr: u32, size: AccessSize, value: u32) -> MemResult<u32> {
        _ = addr;
        _ = size;
        _ = value;

        Err(MemFault::ReadOnly)
    }

    fn contains_addr(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u32
    }

    fn advance_clock(&mut self, _cycles: u32) {
        ()
    }
}
