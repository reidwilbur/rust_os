#![no_std]
#![feature(no_std, lang_items, const_fn, unique, core_str_ext)]

extern crate rlibc;
extern crate spin;

mod vga_buffer;
use core::fmt::Write;

#[no_mangle]
pub extern fn rust_main() {
  let hello = b"Hello monster";
  let color_byte = 0x1f; // white fg, blue bk

  let mut hello_colored = [color_byte; 26];
  for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i*2] = *char_byte;
  }

  let buffer_ptr = (0xb8000 + 1988) as *mut _;
  unsafe { *buffer_ptr = hello_colored };

  vga_buffer::WRITER.lock().write_str("monster monster\n");
  write!(vga_buffer::WRITER.lock(), "Some line woof\n");

  loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

