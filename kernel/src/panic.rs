use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    println!("-- WILL NOW HALT --");
    shutdown()
}

/// 终止程序
///
/// abort
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}
