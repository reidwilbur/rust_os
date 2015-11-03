#![no_std]
#![feature(no_std, lang_items)]

extern crate rlibc;

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

  loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

