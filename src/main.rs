#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use crate::vga_buffer::{ ColorCode, Color, BUFFER_HEIGHT };

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
   println_c!(0, ColorCode::new(Color::Red, Color::Black), "{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_c!(30, ColorCode::new(Color::LightBlue, Color::LightBlue), "**************");
    print!("\n");
    print_c!(30, ColorCode::new(Color::White, Color::LightBlue),     " RUST KERNEL! ");
    print!("\n");
    print_c!(30, ColorCode::new(Color::LightBlue, Color::LightBlue), "**************");
    print!("\n");

    for _i in 0..(BUFFER_HEIGHT / 2) {
        println!();
    }
    loop {}
}
