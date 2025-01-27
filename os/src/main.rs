#![no_std]
#![no_main]

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hi{}", "!");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}



