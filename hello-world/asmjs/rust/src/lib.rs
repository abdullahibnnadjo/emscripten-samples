#![feature(lang_items)]
//#![feature(start)]
#![no_std]
//#![no_main]
#![feature(collections)]

extern crate collections;
use collections::*;

extern "C" {
    // In rust, we can't use something similar to EM_ASM
    // I tried to simulate the same behavior with a macro, calling the
    // emscripten_asm_const function with a stringify!'ed text, but this
    // feature works in a very tricky way, not very compatible with Rust
    // So we simply implement a binding to a JS function that will do what
    // we want, so here just displaying some text
    //
    // Thins function is implemented in the minilib.js file
    fn my_log(text: *const u8, len: usize) -> ();
}

fn log(text: &str) {
    unsafe {
        my_log(text.as_ptr(), text.len());
    }
}

fn log_s(text: &collections::String) {
    unsafe {
        my_log(text.as_ptr(), text.len());
    }
}

// Pull in the system libc library for what crt0.o likely requires.
//extern crate libc;

// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    log("Hello World!"); 
    log_s(&format!("1 + 1 = {}", 1 + 1));
    0
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// // This function may be needed based on the compilation target.
// #[lang = "eh_unwind_resume"]
// #[no_mangle]
// pub extern fn rust_eh_unwind_resume() {
// }

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
