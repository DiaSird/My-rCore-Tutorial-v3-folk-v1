//! The panic handler
use crate::sbi::shutdown;
use crate::task::current_kstack_top;
use core::arch::asm;
use core::panic::PanicInfo;

/// Prints to the standard output, with a newline
/// and shutdown.
///
/// # Examples
///
/// ```
/// panic!("Shutdown machine!");
/// // >[kernel] Panicked at src/main.rs:16 Shutdown machine!
/// ```
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    unsafe {
        backtrace();
    }
    shutdown()
}

unsafe fn backtrace() {
    let mut fp: usize;
    let stop = current_kstack_top();
    asm!("mv {}, s0", out(reg) fp);
    println!("---START BACKTRACE---");
    for i in 0..10 {
        if fp == stop {
            break;
        }
        println!("#{}:ra={:#x}", i, *((fp - 8) as *const usize));
        fp = *((fp - 16) as *const usize);
    }
    println!("---END   BACKTRACE---");
}
