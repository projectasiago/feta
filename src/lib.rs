#![no_std]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![allow(unused_variables)]

fn default_handle_panic(_msg: core::fmt::Arguments,
                        _file: &'static str,
                        _line: u32) -> ! {
	loop {}
}

pub static mut handle_panic: fn(_msg: core::fmt::Arguments,
                                _file: &'static str,
                                _line: u32) -> ! = default_handle_panic;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
	unsafe {
		handle_panic(_msg, _file, _line);
	}
}