use core::arch::asm;

use crate::kernel_main;

/// 汇编入口函数
///
/// 分配栈 并调到rust入口函数
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    const STACK_SIZE: usize = 4096;

    #[link_section = ".bss.stack"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        "   la  sp, {stack} + {stack_size}
            j   kinit
        ",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        options(noreturn),
    )
}

/// 进行操作系统的初始化，
#[no_mangle]
pub extern "C" fn kinit(hart_id: usize, _device_tree_addr: usize) -> ! {
    // 让其他核心进入等待
    if hart_id != 0 {
        support_hart_resume(hart_id, 0);
    }

    clear_bss();
    println!("Hello from Ga-K!");
    kernel_main();
}

/// 辅助核心进入的函数
///
/// 目前让除 0 核之外的其他内核进入该函数进行等待
#[allow(unused)]
extern "C" fn support_hart_resume(hart_id: usize, _param: usize) {
    loop {
        // 使用wfi 省电
        unsafe { asm!("wfi") }
    }
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
