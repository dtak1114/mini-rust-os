// defining attributes
#![feature(lang_items)] // allow lang_items feature attribute.(only in nightly build0
#![no_std] // do not auto link std library. we cannot use!

pub extern fn rust_main() {} // put extern to be compartible with C lang 

//lang defines language item: plugin to the compiler
#[lang = "eh_personality"] extern fn eh_personality() {} // define action in response to Rust's panic!.
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}


