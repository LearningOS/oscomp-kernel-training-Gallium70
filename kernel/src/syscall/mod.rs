//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

mod fs;
mod id;
pub mod process;
mod sync;
mod thread;
mod time;

use crate::{fs::Stat, task::current_process};
use core::convert::TryFrom;
use fs::*;
use id::SyscallNo;
use process::*;
use sync::*;
use thread::*;

use self::time::sys_clock_gettime;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 4]) -> isize {
    if let Ok(syscall) = SyscallNo::try_from(syscall_id) {
        match syscall {
            SyscallNo::WRITE | SyscallNo::WAIT4 | SyscallNo::YIELD => (),
            _ => trace!(
                "P{} syscall: {:?}({}), args: {:x?}",
                current_process().getpid(),
                syscall,
                syscall_id,
                args
            ),
        }

        let ret = match syscall {
            SyscallNo::DUP => sys_dup(args[0]),
            SyscallNo::IOCTL => sys_ioctl(args[0], args[1]),
            SyscallNo::UNLINKAT => sys_unlinkat(args[1] as *const u8),
            SyscallNo::LINKAT => sys_linkat(args[1] as *const u8, args[3] as *const u8),
            SyscallNo::OPEN => sys_open(args[1] as *const u8, args[2] as u32),
            SyscallNo::CLOSE => sys_close(args[0]),
            SyscallNo::PIPE => sys_pipe(args[0] as *mut usize),
            SyscallNo::READ => sys_read(args[0], args[1] as *const u8, args[2]),
            SyscallNo::WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
            SyscallNo::WRITEV => sys_writev(args[0], args[1] as *const _, args[2]),
            SyscallNo::FSTAT => sys_fstat(args[0], args[1] as *mut Stat),
            SyscallNo::EXIT => sys_exit(args[0] as i32),
            SyscallNo::EXIT_GROUP => sys_exit(args[0] as i32),
            SyscallNo::SET_TID_ADDRESS => sys_set_tid_address(),
            SyscallNo::NANOSLEEP => sys_sleep(args[0]),
            SyscallNo::CLOCK_GET_TIME => sys_clock_gettime(args[0] as _, args[1] as _),
            SyscallNo::YIELD => sys_yield(),
            SyscallNo::GETPID => sys_getpid(),
            SyscallNo::GETTID => sys_gettid(),
            SyscallNo::CLONE => sys_fork(),
            SyscallNo::EXECVE => sys_exec(
                args[0] as *const u8,
                args[1] as *const usize,
                args[2] as *const usize,
            ),
            SyscallNo::BRK => sys_brk(args[0] as _),
            SyscallNo::MUNMAP => sys_munmap(args[0], args[1]),
            SyscallNo::MMAP => sys_mmap(args[0], args[1], args[2]),
            SyscallNo::WAIT4 => sys_waitpid(args[0] as _, args[1] as _) as isize,
            _ => panic!("Unsupported syscall: {:?}({})", syscall, syscall_id),
        };

        match syscall {
            SyscallNo::WRITE | SyscallNo::WAIT4 | SyscallNo::YIELD => (),
            _ => trace!(
                "P{} syscall: {:?}({}), ret: {}",
                current_process().getpid(),
                syscall,
                syscall_id,
                ret
            ),
        }
        ret
    } else {
        panic!("Unknown syscall_id: {}", syscall_id);
    }
}
