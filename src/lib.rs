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

    vga_buffer::clear_screen();

    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });
    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personal() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
