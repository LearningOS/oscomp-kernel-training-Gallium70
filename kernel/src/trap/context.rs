//! Implementation of [`TrapContext`]

use core::fmt;

use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// General-Purpose Register x0-31
    pub x: [usize; 32],
    /// sstatus
    pub sstatus: Sstatus,
    /// sepc
    pub sepc: usize,
    /// Token of kernel address space
    pub kernel_satp: usize,
    /// Kernel stack pointer of the current application
    pub kernel_sp: usize,
    /// Virtual address of trap handler entry point in kernel
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        // set CPU privilege to User after trapping back
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}

impl fmt::Display for TrapContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "  ra, sp, gp, tp: {:x?}", &self.x[1..5])?;
        writeln!(
            f,
            "  t0-t6: {:x?}",
            &[&self.x[5..8], &self.x[28..32]].concat()
        )?;
        writeln!(f, "  a0-a7: {:x?}", &self.x[10..17])?;
        writeln!(
            f,
            "  s0-s5: {:x?}",
            &[&self.x[8..10], &self.x[18..22]].concat()
        )?;
        writeln!(f, "  s6-s11: {:x?}", &self.x[22..28])?;
        write!(f, "}}")?;
        Ok(())
    }
}
