// defining attributes
// allow lang_items feature attribute.(only in nightly build)
#![feature(lang_items)] 
// do not auto link std library. we cannot use!
#![no_std] 

extern crate rlibc; //import baremetal rawr api in rust

// put extern to be compartible with C lang 
#[no_mangle] 
pub extern fn rust_main() {
    let hello = b"Hello World!"; //define byte string
    let color_byte = 0x1f;
    
    let mut hello_colored = [color_byte; 24]; // create array size of 24  with color_byte
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte; //create color=bye + char_byte pair for each character
    }

    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored }; //tell rust that this ptr operation is unsafe

    loop{}
} 
//lang defines language item: plugin to the compiler
#[lang = "eh_personality"] extern fn eh_personality() {} // define action in response to Rust's panic!.
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}



#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
