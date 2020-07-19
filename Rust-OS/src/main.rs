#![no_std]
#![no_main] /// Overwrite crt0 entry point
use core::panic::PanicInfo;
mod vga_display;

/// no_mangle to disable name mangling to really outputs function witn name "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_display::test_print();
    use core::fmt::Write;
    vga_display::WRITER.lock().write_str("Jesus it actually works!\n").unwrap();
    write!(vga_display::WRITER.lock(), "number: {}\n", 1.0 / 4.0).unwrap();
    loop{}
}


/// panic handler, "!" represents the never type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
