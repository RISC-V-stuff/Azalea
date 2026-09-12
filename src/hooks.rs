use crate::{memory_device::AccessSize, riscv::instructions::Instruction};

#[derive(Debug, Copy, Clone)]
pub enum BranchKind {
    TODO,
}

pub trait CpuHooks {
    fn on_run(&mut self);

    fn on_instruction(&mut self, pc: u32, instr: &Instruction);

    fn on_mem_read(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32);

    fn on_mem_write(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32);

    fn on_branch(&mut self, pc: u32, target: u32, taken: bool, kind: BranchKind);
}

macro_rules! impl_cpu_hooks_for_tuple {
    ($($t:ident),+) => {
        impl<$($t: CpuHooks),+> CpuHooks for ($($t,)+) {
            fn on_run(&mut self) {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                $($t.on_run();)+
            }
            fn on_instruction(&mut self, pc: u32, instr: &Instruction) {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                $($t.on_instruction(pc, instr);)+
            }
            fn on_mem_read(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                $($t.on_mem_read(pc, addr, size, value);)+
            }
            fn on_mem_write(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                $($t.on_mem_write(pc, addr, size, value);)+
            }
            fn on_branch(&mut self, pc: u32, target: u32, taken: bool, kind: BranchKind) {
                #[allow(non_snake_case)]
                let ($($t,)+) = self;
                $($t.on_branch(pc, target, taken, kind);)+
            }
        }
    };
}

impl_cpu_hooks_for_tuple!(A);
impl_cpu_hooks_for_tuple!(A, B);
impl_cpu_hooks_for_tuple!(A, B, C);
impl_cpu_hooks_for_tuple!(A, B, C, D);
impl_cpu_hooks_for_tuple!(A, B, C, D, E);
impl_cpu_hooks_for_tuple!(A, B, C, D, E, F);
impl_cpu_hooks_for_tuple!(A, B, C, D, E, F, G);
impl_cpu_hooks_for_tuple!(A, B, C, D, E, F, G, H);
impl_cpu_hooks_for_tuple!(A, B, C, D, E, F, G, H, I);

impl CpuHooks for () {
    fn on_run(&mut self) {}

    fn on_instruction(&mut self, _pc: u32, _instr: &Instruction) {}

    fn on_mem_read(&mut self, _pc: u32, _addr: u32, _size: AccessSize, _value: u32) {}

    fn on_mem_write(&mut self, _pc: u32, _addr: u32, _size: AccessSize, _value: u32) {}

    fn on_branch(&mut self, _pc: u32, _target: u32, _taken: bool, _kind: BranchKind) {}
}
