#![no_std]
#![feature(no_std, lang_items, const_fn, unique, core_str_ext)]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;
use core::fmt::Write;

#[no_mangle]
pub extern fn rust_main() {
  vga_buffer::clear_screen();
  println!("Hello monster{}", "!");
  loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

