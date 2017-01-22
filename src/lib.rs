#![feature(const_fn)] 
#![feature(unique)] 
#![feature(lang_items)] 
// defining attributes
// allow lang_items feature attribute.(only in nightly build)
#![feature(lang_items)] 
// do not auto link std library. we cannot use!
#![no_std] 

extern crate rlibc; //import baremetal rawr api in rust
extern crate volatile; //keep buffer out of optimization
extern crate spin; //introduce spinlock to control race condition for out static writer
extern crate multiboot2; //to take and operate multiboot information

#[macro_use]
mod vga_buffer;

// put extern to be compartible with C lang 
#[no_mangle] 
pub extern fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("{}", { println!("Hello"); "World!"});

    // loading multiboot2 info
    // boot info consists of multi tags
    let boot_info = unsafe{
        multiboot2::load(multiboot_information_address)
    };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas");
    for area in memory_map_tag.memory_areas(){
        println!("   start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }



    // println!("Hello, {}!", "World");


    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again");

    // // use core::fmt::Write; // required import to use write! macro 
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);
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
