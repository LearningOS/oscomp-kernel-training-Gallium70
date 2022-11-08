//! Implementation of [`Scheduler`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.

use super::thread::Thread;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;

pub struct Scheduler {
    ready_queue: VecDeque<Arc<Thread>>,
}

/// A simple FIFO scheduler.
impl Scheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<Thread>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<Thread>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

pub fn add_task(task: Arc<Thread>) {
    SCHEDULER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<Thread>> {
    SCHEDULER.lock().fetch()
}
