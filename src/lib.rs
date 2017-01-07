// defining attributes
// allow lang_items feature attribute.(only in nightly build)
#![feature(lang_items)] 
// do not auto link std library. we cannot use!
#![no_std] 

extern crate rlibc; //import baremetal rawr api in rust

// put extern to be compartible with C lang 
#[no_mangle] 
pub extern fn rust_main() {
    let x = ["Hello", "World", "!"];
    let y = x;
    let test = (0..3).flat_map(|x| 0..x).zip(0..);
} 
//lang defines language item: plugin to the compiler
#[lang = "eh_personality"] extern fn eh_personality() {} // define action in response to Rust's panic!.
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}



#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
