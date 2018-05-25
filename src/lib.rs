#![feature(const_unique_new)]
#![feature(unique)]
#![feature(lang_items)]
#![no_std] 
#![feature(const_fn)]
#![feature(ptr_internals)]

#[macro_use]
mod vga_buffer;

extern crate multiboot2;
extern crate rlibc;
extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    vga_buffer::clear_screen();

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("  start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personal() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
