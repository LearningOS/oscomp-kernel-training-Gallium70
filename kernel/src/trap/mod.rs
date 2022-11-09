//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

mod context;

use crate::config::TRAMPOLINE;
use crate::mm::translated_ref;
use crate::syscall::syscall;
use crate::task::{
    current_process, current_task, current_trap_cx, current_trap_cx_user_va, current_user_token,
    exit_current_and_run_next, suspend_current_and_run_next,
};
use crate::timer::{check_timer, set_next_trigger};
use alloc::string::String;
use alloc::{fmt, format};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

core::arch::global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

fn peek_user_mem(addr: isize) {
    debug!("Peeking user memory near {:x}:", addr);
    let token = current_user_token();
    const PEEK_RANGE: isize = 16;
    const PEEK_STEP: isize = 1;
    type PeekType = usize;
    let size = core::mem::size_of::<PeekType>() as isize;
    for offset in -PEEK_RANGE..PEEK_RANGE {
        let base = addr + offset * PEEK_STEP * size;
        let mut f = String::new();
        f += &format!("{:x}: ", base);
        for sub_idx in 0..PEEK_STEP {
            let ptr = base + sub_idx * size;
            f += &format!("{:x} ", *translated_ref(token, ptr as *const PeekType));
        }
        debug!("{}", f);
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            // get system call return value
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12], cx.x[13]]);
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.x[10] = result as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            error!(
                "{:?} in application (P{}), bad addr = {:#x}, bad instr addr = {:#x}, trap_cx = {}, core dumped.",
                scause.cause(),
                current_process().getpid(),
                stval,
                current_trap_cx().sepc,
                current_trap_cx()
            );
            // peek stack
            peek_user_mem(current_trap_cx().x[2] as _);
            // page fault exit code
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            // illegal instruction exit code
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            check_timer();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    // trace!("return to user");
    set_user_trap_entry();
    let trap_cx_ptr = current_trap_cx_user_va();
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        core::arch::asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap {:?} from kernel!", scause::read().cause());
}

pub use context::TrapContext;
