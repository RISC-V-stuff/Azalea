#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Csr {
    // =========== Unprivileged Floating-Point ============ //
    /// Floating-Point Accrued Exceptions
    fflags = 0x1,
    /// Floating-Point Dynamic Rounding Mode
    frm = 0x2,
    /// Floating-Point Control and Status Register
    fcsr = 0x3,

    // =========== Unprivileged Counter/Timers ============ //
    /// Cycle counter for RDCYCLE instruction
    cycle = 0xC00,
    /// Timer for RDTIME instruction
    time = 0xC01,
    /// Instructions-retired counter for RDINSTRET instruction
    instret = 0xC02,
    /// Upper 32 bits of cycle, RV32I only
    cycleh = 0xC80,
    /// Upper 32 bits of time, RV32I only
    timeh = 0xC81,
    /// Upper 32 bits of instret, RV32I only
    instreth = 0xC82,
}

impl TryFrom<u16> for Csr {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x001 => Ok(Self::fflags),
            0x002 => Ok(Self::frm),
            0x003 => Ok(Self::fcsr),
            0xC00 => Ok(Self::cycle),
            0xC01 => Ok(Self::time),
            0xC02 => Ok(Self::instret),
            0xC80 => Ok(Self::cycleh),
            0xC81 => Ok(Self::timeh),
            0xC82 => Ok(Self::instreth),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CsrAccess {
    ReadOnly,
    ReadWrite,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Privilege {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}

pub struct CsrPerms {
    pub access: CsrAccess,
    pub min_privilege: Privilege,
}

pub fn get_access_perms(csr: Csr) -> CsrPerms {
    let addr = csr as u16;

    let access = if (addr >> 10) & 0b11 == 0b11 {
        CsrAccess::ReadOnly
    } else {
        CsrAccess::ReadWrite
    };

    let min_privilege = match (addr >> 8) & 0b11 {
        0b00 => Privilege::User,
        0b01 => Privilege::Supervisor,
        0b10 => Privilege::Hypervisor,
        0b11 => Privilege::Machine,
        _ => unreachable!("2-bit mask can only produce 0..=3"),
    };

    CsrPerms {
        access,
        min_privilege,
    }
}
