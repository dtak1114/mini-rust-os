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

    // get section info for out kernel ELF file
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections(){
        let section_attr = match section.flags {
            1u64 => "writable",
            2u64 => "in memory",
            4u64 => "executable",
            _ => "the other"
        };

        println!("  addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}({:})",
             section.addr, section.size, section.flags, section_attr);

    }

    // calculate kernel (start|end)
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start
            , kernel_end);;

    // calculate multiboot info section
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);


    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again");

    // // use core::fmt::Write; // required import to use write! macro 
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);
    loop{}
} 
//lang defines language item: plugin to the compiler
#[lang = "eh_personality"] extern fn eh_personality() {} 


// define action in response to Rust's panic!.
#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("  {}", fmt);
    loop{}
}



#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
