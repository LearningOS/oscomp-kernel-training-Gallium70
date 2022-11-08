use crate::allocator::{Allocator, RecycleAllocator};
use crate::task::context::ThreadContext;
use crate::{
    mm::{PhysPageNum, VirtAddr},
    task::process::Process,
};
use alloc::sync::Weak;
use riscv::interrupt::Mutex;

pub struct Thread {
    tid: TidTracker,
    proc: Weak<Process>,
    entry: VirtAddr,
    status: ThreadStatus,
    kstack: KernelStack,
    ustack: UserStack,
    exit_code: Option<isize>,
    trap_context: PhysPageNum,
    thread_context: ThreadContext,
}

struct KernelStackTracker(usize);

impl Iterator for KernelStackTracker {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.0 += 1;
        Some(self.0)
    }
}

lazy_static::lazy_static! {
    pub static ref KERNEL_STACK_ALLOCATOR: Mutex<RecycleAllocator<KernelStackTracker>> = Mutex::new(RecycleAllocator::new());
}


