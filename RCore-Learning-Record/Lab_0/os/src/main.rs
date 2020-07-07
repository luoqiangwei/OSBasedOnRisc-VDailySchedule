//!  - `#![no_std]`
//! 禁用Rust标准库
#![no_std]

//!  - `#![no_main]`
//! 不使用`main`函数等全部Rust-level入口点作为程序入口
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {}
}