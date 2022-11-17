use core::convert::TryFrom;

use linux_raw_sys::general::timespec;

use crate::{mm::translated_refmut, task::current_user_token, timer::get_time_us};

pub type Secs = isize;
pub type Nsecs = isize;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Timespec {
    /// Seconds.
    pub tv_sec: Secs,

    /// Nanoseconds. Must be less than 1_000_000_000.
    pub tv_nsec: Nsecs,
}

numeric_enum_macro::numeric_enum! {
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[non_exhaustive]
    /// `CLOCK_*` constants for use with [`clock_gettime`].
    ///
    /// These constants are always supported at runtime, so `clock_gettime` never
    /// has to fail with `INVAL` due to an unsupported clock. See
    /// [`DynamicClockId`] for a greater set of clocks, with the caveat that not
    /// all of them are always supported.
    ///
    /// [`clock_gettime`]: crate::time::clock_gettime
    pub enum ClockId {
        /// `CLOCK_REALTIME`
        Realtime = linux_raw_sys::general::CLOCK_REALTIME,

        /// `CLOCK_MONOTONIC`
        Monotonic = linux_raw_sys::general::CLOCK_MONOTONIC,

        /// `CLOCK_PROCESS_CPUTIME_ID`
        ProcessCPUTime = linux_raw_sys::general::CLOCK_PROCESS_CPUTIME_ID,

        /// `CLOCK_THREAD_CPUTIME_ID`
        ThreadCPUTime = linux_raw_sys::general::CLOCK_THREAD_CPUTIME_ID,

        /// `CLOCK_REALTIME_COARSE`
        RealtimeCoarse = linux_raw_sys::general::CLOCK_REALTIME_COARSE,

        /// `CLOCK_MONOTONIC_COARSE`
        MonotonicCoarse = linux_raw_sys::general::CLOCK_MONOTONIC_COARSE,

        /// `CLOCK_MONOTONIC_RAW`
        MonotonicRaw = linux_raw_sys::general::CLOCK_MONOTONIC_RAW,
    }
}

pub fn sys_clock_gettime(clock_id: usize, tp: *mut Timespec) -> isize {
    if let Ok(clock_id) = ClockId::try_from(clock_id as u32) {
        match clock_id {
            ClockId::Realtime | ClockId::Monotonic => {
                let token = current_user_token();
                let us = get_time_us() as isize;
                let tp = translated_refmut(token, tp);
                *tp = Timespec {
                    tv_sec: us / 1_000_000,
                    tv_nsec: us % 1_000_000 * 1_000,
                };
                0
            }
            _ => {
                warn!("clock id {:?} is not implemented on the system!", clock_id);
                -1
            }
        }
    } else {
        warn!("clock id {:?} is not implemented on the system!", clock_id);
        -1
    }
}
