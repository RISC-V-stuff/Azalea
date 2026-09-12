use super::*;

pub struct BasicRam {
    start: u64,
    data: Vec<u8>,
}

impl BasicRam {
    pub fn new(start_addr: u64, size: usize) -> Self {
        Self {
            start: start_addr,
            data: vec![0; size],
        }
    }
}

impl MemoryDevice for BasicRam {
    fn load(&mut self, addr: u64, size: AccessSize) -> MemResult<Access> {
        let i = (addr - self.start) as usize;
        let cycles = 0;
        let value = match size {
            AccessSize::Byte => self.data[i] as u64,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u64
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes) as u64
            }
            AccessSize::Doubleword => {
                let bytes: [u8; 8] = self.data[i..i + 8].try_into().unwrap();
                u64::from_le_bytes(bytes)
            }
        };
        Ok(Access { value, cycles })
    }

    fn store(&mut self, addr: u64, size: AccessSize, value: u64) -> MemResult<u64> {
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
                self.data[i..i + 4].copy_from_slice(&(value as u32).to_le_bytes());
            }
            AccessSize::Doubleword => {
                self.data[i..i + 8].copy_from_slice(&value.to_le_bytes());
            }
        }
        Ok(cycles)
    }

    fn contains_addr(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u64
    }

    fn advance_clock(&mut self, _cycles: u32) {
        ()
    }
}

pub struct BasicRom {
    start: u64,
    data: Vec<u8>,
}

impl BasicRom {
    pub fn new(start_addr: u64, data: &[u8]) -> Self {
        Self {
            start: start_addr,
            data: data.into(),
        }
    }
}

impl MemoryDevice for BasicRom {
    fn load(&mut self, addr: u64, size: AccessSize) -> MemResult<Access> {
        let i = (addr - self.start) as usize;
        let cycles = 0;
        let value = match size {
            AccessSize::Byte => self.data[i] as u64,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u64
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes) as u64
            }
            AccessSize::Doubleword => {
                let bytes: [u8; 8] = self.data[i..i + 8].try_into().unwrap();
                u64::from_le_bytes(bytes)
            }
        };
        Ok(Access { value, cycles })
    }

    fn store(&mut self, addr: u64, size: AccessSize, value: u64) -> MemResult<u64> {
        _ = addr;
        _ = size;
        _ = value;

        Err(MemFault::ReadOnly)
    }

    fn contains_addr(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u64
    }

    fn advance_clock(&mut self, _cycles: u32) {
        ()
    }
}
