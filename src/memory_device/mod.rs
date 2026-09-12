mod memories;
pub use memories::*;

pub mod peripherals;

mod bus;
pub use bus::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccessSize {
    Byte,
    Half,
    Word,
    Doubleword,
}

#[derive(Copy, Clone, Debug)]
pub struct Access {
    pub value: u64,
    pub cycles: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemFault {
    Unmapped,
    Misaligned,
    ReadOnly,
    WriteOnly,
}

pub type MemResult<T> = Result<T, MemFault>;

/// A memory-mapped device: RAM, ROM, a peripheral, or a bus/passthrough
/// element composing other devices.
///
/// /// - `load`/`store` return how many cycles *this specific request* took.
///   That number is what the CPU actually experiences: "I asked for this
///   word, it cost 40 cycles, now I have the value." Nothing about it is
///   retroactive or deferred — it's the real answer to "how long until I
///   can use this."
///
/// - A device may use its own internal state (left over from previous
///   accesses) to decide that number — e.g. "is the row I need already
///   open?" — because that's the only state it can causally know about.
///   It cannot know about future accesses, same as real hardware.
///
/// - After an access completes, the *caller* (typically a `Bus`) is
///   responsible for calling [`advance_clock`](MemoryDevice::advance_clock).
///   The caller may choose to "block" on that request and advance the time
///   with the returned value from the access or have multiple operation in
///   flight by advancing the time a smaller interval.
pub trait MemoryDevice {
    /// Whether this device (or anything it owns, for passthrough devices
    /// like a bus) claims the given address.
    fn contains_addr(&self, addr: u64) -> bool;

    /// Read `size` bytes from `addr`.
    ///
    /// Returns the value plus the number of cycles this specific request
    /// took, computed from the device's state as of *before* this access
    /// Returns `Err` if this device can't service the request
    /// (e.g. `MemFault::Misaligned` for devices that enforce their own alignment)
    fn load(&mut self, addr: u64, size: AccessSize) -> MemResult<Access>;

    /// Write `value` (`size` bytes) to `addr`.
    ///
    /// Returns the number of cycles this request took, same contract as
    /// `load`. Returns `Err` if this device can't service the write.
    fn store(&mut self, addr: u64, size: AccessSize, value: u64) -> MemResult<u64>;

    /// Advance this device's internal clock by `cycles` of elapsed
    /// simulated time.
    ///
    /// /// Implementers: do **not** advance time-dependent state inside
    /// `load`/`store` itself only in `advance_clock`.
    ///
    /// Devices with no time-dependent behavior can ignore this (default
    /// no-op).
    fn advance_clock(&mut self, cycles: u32);
}

/// Narrows a u64-native memory subsystem to 32-bit CPU semantics.
/// Not a MemoryDevice itself — this wraps the root of the tree from
/// the CPU's side, not the device side. Handles address wraparound
/// and value masking/widening at the one place a 32-bit CPU talks
/// to the (u64-native) bus.
pub struct Bus32<'a> {
    inner: &'a mut dyn MemoryDevice,
}

impl<'a> Bus32<'a> {
    pub fn new(inner: &'a mut dyn MemoryDevice) -> Self {
        Self { inner }
    }

    pub fn load(&mut self, addr: u32, size: AccessSize) -> MemResult<Access32> {
        let access = self.inner.load(addr as u64, size)?;
        Ok(Access32 {
            value: access.value as u32,
            cycles: access.cycles,
        })
    }

    pub fn store(&mut self, addr: u32, size: AccessSize, value: u32) -> MemResult<u32> {
        self.inner
            .store(addr as u64, size, value as u64)
            .map(|r| r as u32)
    }

    pub fn advance_clock(&mut self, cycles: u32) {
        self.inner.advance_clock(cycles);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Access32 {
    pub value: u32,
    pub cycles: u32,
}
