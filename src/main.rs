#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use crate::vga_buffer::{ ColorCode, Color };

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
   println_at!(None, 0, ColorCode::new(Color::Red, Color::Black), "{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println_at!(None, 0, ColorCode::new(Color::LightRed, Color::Black), "Hello world");

    loop {}
}
